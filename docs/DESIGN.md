# pl2pl 設計メモ / Design notes
(少しずつ追記 / to be written gradually)

## 1. 目的 / Goal
## 2. 中間表現 IR / Intermediate representation
## 3. フロントエンド(入力) / Frontends: COBOL, Java, Flask, Django, FastAPI …
## 4. バックエンド(出力) / Backends: FastAPI, MFastAPI, Poem, RPoem
## 5. 既存リポジトリの取り込み順 / Migration order
## 6. 未決事項 / Open questions

---
## 決定事項 / Decisions (2026-11-18, owner)
**日本語**
1. **全部は載せない**: 変換元・変換先の全組み合わせは網羅しない(個人運営のWEBでは現実的でなく、完璧主義にならない)。`docs/LANGUAGES.md` の一覧から**選んだものだけ**を実装して完成とする。
2. **WEB版は常駐させず、ディスク上のプログラムを呼び出す**: 変換器は個別の実行ファイル(例: `java_to`、`cobol_to`)としてHDD/ディスクに置き、WEB側は要求のたびに必要な1つだけを起動して結果を返す。全変換器を同時にメモリへ載せない(システムメモリー不足の回避)。
3. **要求ごとに起動・終了**: 変換は短命プロセスで行い、同時実行数に上限・タイムアウト・入力サイズ上限を設ける(共有基盤の受付制御の方針と同じ。`runo/CLAUDE.md` の配備方針を参照)。
4. **追加は1つずつ**: 新しい変換元は、登録表(名前・実行ファイル・対応する出力先)に1行足すだけで増やせる形にする。
5. 未決: 登録表の形式、実行ファイルの置き場所、サンドボックス(別ユーザー・資源制限)、アップロード上限、結果の保持期間。
**English**
1. **Not everything**: not all source/target combinations will be covered (unrealistic for a personal web service; perfectionism is not the goal). Implement only the entries **chosen from `docs/LANGUAGES.md`**; that counts as complete.
2. **The web version does not keep converters resident; it calls programs on disk**: each converter is a separate executable (e.g. `java_to`, `cobol_to`) stored on the HDD/disk; the web side starts only the one needed per request and returns the result. Converters are never all loaded in memory at once (avoids running out of system memory).
3. **Start and exit per request**: conversions run as short-lived processes with caps on concurrency, timeout and input size (same idea as the shared-base admission control; see the deployment policy in `runo/CLAUDE.md`).
4. **Add one at a time**: a new source is added by one line in a registry (name, executable, supported targets).
5. Open: registry format, executable location, sandboxing (separate user, resource limits), upload limit, result retention.

---
## 訂正・決定事項(2026-11-18、所有者。上の「WEB版が変換器を呼び出す」案を置き換える) / Correction (supersedes the previous web-calls-converters idea)
**日本語**
1. **WEB版は変換しない**: WEB版は pl2pl の**ダウンロードと説明書(世界中の言語)の提示だけ**。変換器はWEB側で常駐も起動もしない。
2. **変換はPCにインストールしたアプリで行う**: 起動するとファイルエクスプローラー(フォルダ選択)が開き、変換元のディレクトリの**一番上(プロジェクトの親フォルダ等)を複数指定**できる。
3. **自動判定**: 変換元のプログラム言語・フレームワーク・プロジェクト名・管理関連ファイル(ビルド設定等)を自動で判定する(COBOL_to / Java_to と同じ流儀)。
4. **変換先を指定**: 変換先の言語・フレームワーク・DB などを利用者が選ぶ。
5. **全部は載せない方針は維持**: 一覧から選んだものだけ実装(個人運営で全組み合わせは現実的でない)。
6. **主要な変換元(最優先)**: OSACCESS(※表記のとおり記録。「OS」と「ACCESS」の区切りか別製品か要確認)、ACCESS VBA、VB、VB.NET、VBA、C、C++、C#.NET、COBOL。
7. 未決: アプリの形(デスクトップGUI / CLI+ダイアログ。COBOL_to は rfd のフォルダ選択)、登録表、各言語の判定ルール、変換先の組み合わせ表、WEB側の多言語説明書の言語数。
**English**
1. **The web version does not convert**: it only offers **downloads and the manual in many languages**. No converter runs or stays resident on the web side.
2. **Conversion runs in the installed PC app**: on start it opens a file explorer (folder picker) where the user can specify **several top-level directories** (e.g. parent folders of projects).
3. **Auto-detection** of source language, framework, project name and management/build files (same style as COBOL_to / Java_to).
4. **The user chooses the target** language, framework, DB, etc.
5. **Still not exhaustive**: only entries chosen from the list are implemented.
6. **Priority sources**: OSACCESS (recorded as written; unclear whether "OS" + "ACCESS" or a separate product — to confirm), Access VBA, VB, VB.NET, VBA, C, C++, C#.NET, COBOL.
7. Open: app form (desktop GUI vs CLI + dialog; COBOL_to uses an rfd folder picker), registry, detection rules per language, target combination table, number of languages for the web manual.

---
## 主要OS・主要言語の想定(2026-11-18、所有者。直前の「主要な変換元」を置き換える) / Assumed main OSes and languages (supersedes the previous priority list)
**日本語**
- **主要OS**: メインフレーム、UNIX、Linux、Windows、Mac。(これにより、直前の「OSACCESS」は「OS」と「ACCESS」の区切りだったと解釈する。所有者に未確認。)
- **主要言語(21項目、表記は所有者のまま)**: ACCESS、ACCESS VBA、VB、VB.NET、VBA、EXCEL VBA、C、C++、C#.NET、F#、F#.NET、COBOL、Perl、PHP、Python、Ruby、Java、JavaScript、TypeScript、Go、Rust。
- **相互変換を想定**: 各言語の有名なフレームワークとDATABASEも含め、双方向に変換できる形を想定(あくまで想定。全部は載せない方針は維持)。
- **設計上の注意(私の見積もり)**: 21言語を総当たりで相互変換すると向きを区別して 21×20 = 420 通り。個別に作らず、共通の中間表現(IR)を挟み「入力側21+出力側21」で済ませる案が現実的。ただし言語の性質が大きく違う(ACCESS=DB・画面・VBAが一体、COBOL=手続き+帳票、Rust=所有権)ため、IRで表せない部分は `TODO` として残す方針(既存ツールと同じ)。
- **OSの意味**: 変換元/先の言語に加え、動作OS(JCL・UNIXシェル・Windows API・Macなど)への移植も含めるのか、インストール型アプリの対応OSだけを指すのかは未決。
**English**
- **Main OSes**: mainframe, UNIX, Linux, Windows, Mac. (So the earlier "OSACCESS" is read as "OS" + "ACCESS"; not yet confirmed with the owner.)
- **Main languages (21, as written by the owner)**: Access, Access VBA, VB, VB.NET, VBA, Excel VBA, C, C++, C#.NET, F#, F#.NET, COBOL, Perl, PHP, Python, Ruby, Java, JavaScript, TypeScript, Go, Rust.
- **Mutual (both-way) conversion assumed**, including each language's well-known frameworks and databases. Only an assumption; the not-exhaustive policy stays.
- **Design note (my estimate)**: all-pairs conversion of 21 languages is 21 x 20 = 420 directed pairs. Rather than build each, go through a shared IR (21 frontends + 21 backends). Languages differ a lot (Access = DB + UI + VBA in one, COBOL = procedural + reports, Rust = ownership), so what the IR cannot express stays as `TODO`, as in the existing tools.
- **Meaning of "OS"**: undecided whether it also covers porting to the OS (JCL, UNIX shell, Windows API, Mac) or only the OSes the installed app supports.

---
## 「OS」の意味(2026-11-18、所有者の決定) / Meaning of "OS" (owner decision)
**日本語**: 「OS」は**動作OSへの移植(JCL、UNIXシェル、Windows API など)を含める**。インストール型アプリ自体の対応OSの範囲は、情報を集めてから決める(下記は私の見積もりで未検証)。
- 変換の対象(コンテンツ)としてのOS: JCL・REXX・CLIST(メインフレーム)、sh/ksh/bash(UNIX/Linux)、PowerShell・バッチ・Win32/COM(Windows)、Cocoa/AppleScript(Mac)、各OSのAPI対応表。→ 言語変換とは別に**OS API対応表**が要る。
- アプリが動くOS: 実績=Windows x64/ARM64、Linux amd64/arm64、macOS arm64/x64(COBOL_to / Java_to のCIで試験)。メインフレーム(z/OS)ではRustの公式対応が無いと理解しており(未検証)、変換元ファイルをPCへ持ってきて変換する形が現実的。UNIX系(AIX・Solaris・HP-UX・FreeBSD)は対応状況が分かれる(要調査)。
**English**: "OS" **includes porting to the target OS (JCL, UNIX shell, Windows API, ...)**. The OSes the installed app itself runs on will be decided after gathering information (below is my unverified estimate).
- OS as conversion subject: JCL/REXX/CLIST (mainframe), sh/ksh/bash (UNIX/Linux), PowerShell/batch/Win32/COM (Windows), Cocoa/AppleScript (Mac), plus per-OS API mapping tables. This needs an **OS API mapping table** separate from language conversion.
- OSes the app runs on: proven = Windows x64/ARM64, Linux amd64/arm64, macOS arm64/x64 (tested in the COBOL_to / Java_to CI). I understand Rust has no official z/OS support (unverified), so bringing source files to a PC is the realistic path for mainframe. UNIX variants (AIX, Solaris, HP-UX, FreeBSD) vary (to investigate).
