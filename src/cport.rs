//! UNIX の C → Windows(MSVC/MinGW)/Linux で通る C。互換ヘッダーを挟み、機械的に直せる関数は置換し、残りは移植の指針つきで報告する。
use regex::Regex;

pub const COMPAT_H: &str = "/* pl2pl_compat.h — pl2pl cport が生成。UNIX 由来の簡単な差を吸収する(全てではない。REPORT を参照) */
#ifndef PL2PL_COMPAT_H
#define PL2PL_COMPAT_H
#include <string.h>
#include <stdlib.h>
#if defined(_WIN32)
#include <io.h>
#include <process.h>
#include <windows.h>
#if defined(_MSC_VER)
#define strcasecmp _stricmp
#define strncasecmp _strnicmp
#define getpid _getpid
#define access _access
typedef long long ssize_t;
#endif
#define sleep(s) Sleep((DWORD)(s) * 1000)
#define usleep(us) Sleep((DWORD)((us) / 1000))
#else
#include <unistd.h>
#include <strings.h>
#endif
#endif
";

// (正規表現, 内容, 指針)
const MANUAL: [(&str, &str, &str); 18] = [
    (r"\bfork\s*\(", "fork", "Windows に fork は無い。CreateProcess / posix_spawn、または pthread/スレッドへ設計を直す"),
    (r"\bexec[lv]p?e?\s*\(", "exec 系", "CreateProcess / _spawn 系へ。引数の引用符規則が違う"),
    (r"\bwaitpid\s*\(|\bwait\s*\(", "wait/waitpid", "WaitForSingleObject へ(fork と一体で書き換え)"),
    (r"\bpipe\s*\(", "pipe", "_pipe(Windows)または CreatePipe へ"),
    (r"\bmmap\s*\(|\bmunmap\s*\(", "mmap", "CreateFileMapping / MapViewOfFile へ"),
    (r"\bsocket\s*\(|\bbind\s*\(|\blisten\s*\(|\baccept\s*\(|\bconnect\s*\(", "BSD ソケット", "Winsock: WSAStartup/WSACleanup が必要、close→closesocket、型 SOCKET"),
    (r"\bselect\s*\(|\bpoll\s*\(", "select/poll", "Winsock の select は socket のみ。WSAPoll や IOCP へ"),
    (r"\bpthread_\w+", "pthread", "C11 threads、または winpthreads / pthreads-win32。MSVC は std::thread/Win32 スレッドへ"),
    (r"\bsigaction\s*\(|\bkill\s*\(|\balarm\s*\(", "シグナル", "Windows は signal() の一部のみ。SetConsoleCtrlHandler / タイマーへ"),
    (r"\bioctl\s*\(|\btermios\b|\btcgetattr\s*\(", "端末制御", "Windows コンソール API(SetConsoleMode)へ"),
    (r"\bdlopen\s*\(|\bdlsym\s*\(", "dlopen/dlsym", "LoadLibrary / GetProcAddress へ"),
    (r"\bsyslog\s*\(", "syslog", "Windows イベントログ、または独自ログへ"),
    (r"\bgetpw\w+\s*\(|\bgetgr\w+\s*\(|\bsetuid\s*\(|\bchown\s*\(", "ユーザー/権限", "Windows の ACL モデルは別物。設計を見直す"),
    (r"\bfcntl\s*\(", "fcntl", "用途ごとに Windows API(ロック: LockFileEx 等)へ"),
    (r"\bopendir\s*\(|\breaddir\s*\(", "dirent", "MinGW は対応。MSVC は FindFirstFile または dirent.h 互換ライブラリ"),
    (r"\bgettimeofday\s*\(", "gettimeofday", "MSVC に無い。GetSystemTimeAsFileTime か timespec_get へ"),
    (r"\bclock_gettime\s*\(", "clock_gettime", "MSVC は QueryPerformanceCounter か timespec_get へ"),
    (r"\bchmod\s*\(|\bmkfifo\s*\(", "chmod/mkfifo", "Windows の chmod は読み書き属性のみ。FIFO は名前付きパイプへ"),
];

const HEADERS: [(&str, &str); 10] = [
    ("sys/socket.h", "Winsock(winsock2.h / ws2tcpip.h)"),
    ("netinet/in.h", "Winsock(winsock2.h)"),
    ("arpa/inet.h", "Winsock(ws2tcpip.h)"),
    ("sys/wait.h", "Windows に無い(プロセス管理を作り直し)"),
    ("sys/mman.h", "Windows に無い(CreateFileMapping)"),
    ("termios.h", "Windows に無い(コンソール API)"),
    ("poll.h", "Windows に無い(WSAPoll)"),
    ("dlfcn.h", "Windows に無い(LoadLibrary)"),
    ("pthread.h", "MSVC に無い(C11 threads / winpthreads)"),
    ("sys/ioctl.h", "Windows に無い"),
];

pub fn convert(src: &str, name: &str, rep: &mut Vec<String>) -> Vec<(String, String)> {
    let mut out = src.to_string();
    let mut changed = 0;
    // 機械的に直せるもの
    let bz = Regex::new(r"\bbzero\s*\(\s*([^,()]+?)\s*,\s*([^()]+?)\s*\)").unwrap();
    let n = bz.find_iter(&out).count();
    if n > 0 {
        out = bz.replace_all(&out, "memset($1, 0, $2)").to_string();
        changed += n;
        rep.push(format!("{name}: bzero → memset を {n} 箇所置換"));
    }
    let bc = Regex::new(r"\bbcopy\s*\(\s*([^,()]+?)\s*,\s*([^,()]+?)\s*,\s*([^()]+?)\s*\)").unwrap();
    let n = bc.find_iter(&out).count();
    if n > 0 {
        out = bc.replace_all(&out, "memmove($2, $1, $3)").to_string();
        changed += n;
        rep.push(format!("{name}: bcopy → memmove(引数の順序も入れ替え)を {n} 箇所置換"));
    }
    for (from, to) in [("rindex", "strrchr"), ("index", "strchr")] {
        let re = Regex::new(&format!(r"(^|[^\w.>]){from}\s*\(")).unwrap();
        let n = re.find_iter(&out).count();
        if n > 0 {
            out = re.replace_all(&out, format!("${{1}}{to}(")).to_string();
            changed += n;
            rep.push(format!("{name}: {from} → {to} を {n} 箇所置換"));
        }
    }
    // 互換ヘッダーを最初の #include の前へ
    if !out.contains("pl2pl_compat.h") {
        if let Some(i) = out.find("#include") {
            out.insert_str(i, "#include \"pl2pl_compat.h\"\n");
        } else {
            out.insert_str(0, "#include \"pl2pl_compat.h\"\n");
        }
    }
    // 手作業が要るもの
    for (pat, what, hint) in MANUAL {
        if let Some(m) = Regex::new(pat).unwrap().find(src) {
            rep.push(format!("{name}:{} {what}: {hint}", crate::util::line_of(src, m.start())));
        }
    }
    for (h, hint) in HEADERS {
        if let Some(i) = src.find(&format!("<{h}>")) {
            rep.push(format!("{name}:{} <{h}>: {hint}", crate::util::line_of(src, i)));
        }
    }
    if Regex::new(r#""[^"\n]*(/tmp/|/dev/|/etc/|/usr/|/var/)[^"\n]*""#).unwrap().is_match(src) {
        rep.push(format!("{name}: UNIX のパス(/tmp /dev /etc 等)が文字列にある。Windows 向けは getenv(\"TEMP\") 等へ"));
    }
    if src.contains("'\\n'") || src.contains("\\r") {
        rep.push(format!("{name}: 改行の扱い(\\n / \\r)。Windows のテキストモードは \\r\\n、バイナリは \"rb\"/\"wb\" で開く"));
    }
    let _ = changed;
    vec![(String::new(), out)]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rewrites_and_reports() {
        let src = "#include <stdio.h>\n#include <sys/socket.h>\nint main(){ char b[8]; bzero(b, 8); char *p = index(b, 'x'); fork(); return 0; }\n";
        let mut r = vec![];
        let o = convert(src, "a.c", &mut r).remove(0).1;
        assert!(o.starts_with("#include \"pl2pl_compat.h\""));
        assert!(o.contains("memset(b, 0, 8)"));
        assert!(o.contains("strchr(b, 'x')"));
        assert!(r.iter().any(|x| x.contains("fork")));
        assert!(r.iter().any(|x| x.contains("sys/socket.h")));
    }
    #[test]
    fn does_not_touch_member_index() {
        let mut r = vec![];
        let o = convert("int f(S*s){ return s->index(1) + obj.index(2); }", "a.c", &mut r).remove(0).1;
        assert!(o.contains("s->index(1)") && o.contains("obj.index(2)"));
    }
}
