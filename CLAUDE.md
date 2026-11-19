# pl2pl 開発メモ
- **やくわり**: 言葉(言語)・フレームワーク・OS をまたいで書きかえる道具(Rust 製、`src/`)。元になる言葉の一覧は `docs/LANGUAGES.md`、組み合わせは `docs/COMBINATIONS.md`。設計は持ち主が `docs/DESIGN.md` に少しずつ書く。**勝手に仕様を決めて書かない。**
- **ほうこう**: 有名な言葉どうしの「多対多」の書きかえを目ざす。全部を一度には作らず、選んだものから。WEB は説明とダウンロードだけで、書きかえは PC のアプリで行う。
- **きまり**: 書きかえられない所は `TODO` で残して報告する(こっそり消さない)。判定は慎重に(依存や副作用が心配なら変えない)。実際に動かしてから「できた」と言う。新しい REST は作らない。返事は日本語のみ。
- **共通の形(IR)**: `src/ir.rs`(型・本体・モデル・ミドルウェア・テンプレート)と、書き出し側の `src/ir_emit.rs`。入力側(PHP・Python・Java)と出力側(JSON・OpenAPI・FastAPI)を分けて、組み合わせを増やしやすくする。`pl2pl php` もこの共通の書き出しを通る(出力を変えない変更は、前後の出力の diff が空であることで確かめる)。ルートを見つけられなかったファイルは報告する。出力は `py_compile` などで実際に確かめる。
- **PHP**: `src/php.rs`・`src/blade.rs`・`src/middleware.rs`。通し試験は `tests/run-php.sh`(本物の FastAPI・SQLite・Jinja2)、単体は `cargo test --release`。WSL では `wsl -d Ubuntu -- bash <ファイル>`。
- **自動追従**: `pl2pl watch`(`src/watch.rs`、`registry/ecosystem.toml`)。確認コマンドが空の項目は「未検証」と出すだけで、勝手に更新しない。通知は version-control の `--notify-file`。
- **配備**: 紹介ページは `webpage/index.html`。VPS は `/root/easy-web.tokyo/pl2pl`(`git pull` で更新、静的なので再起動は不要)、`pl2pl-web.service` ポート 8142、`domains.toml` に追記済み。
- **落とし穴**: 日本語を含むファイルは Python に `-X utf8` で書く。シェルスクリプトは LF。Git Bash は `/mnt/...` を書きかえる。引用符や `\` の多い文字列は、heredoc でなく Write / Edit ツールで。
- **COBOL_to / Java_to**: `converters/` に取り込み済み。元のリポジトリを消すのは取り返しがつかないので、最後の確認を取ってから。
- **翻訳システム**(自然言語)は変換器と分ける。変換器は LLM に頼らない。
- **文書の言葉**: 原本は日本語。英語はやさしい言葉で書き直した。ほかの8言語(`docs/i18n/`)も、同じ内容に合わせて AI 翻訳した(ネイティブ確認前)。過去の経緯は `docs/HISTORY-ja.md`。

## 2026-11-19 いまの状況と、これから作るもの / Where we are now, and what comes next

### 日本語(やさしく)
**いまできること**: pl2pl は、PHP(Laravel など)や Python や Java で書かれたウェブアプリを、Python の FastAPI に作り直すお助けロボットです。道順(ルート)、データの入れ物(モデル)、見た目の部品(Blade の画面)、見張り番(ミドルウェア)まで変えられます。通し試験は 55 項目すべて合格です。
**新しく直したところ**: 「新しい版が出たら自動で確かめる」機能(`pl2pl watch`)が、古い道具のまま試していたので、合格しても本当の確認になっていませんでした。いまは、毎回**まっさらな部屋(新しい venv)に最新版を入れてから**試験します。fastapi 0.165 で本当に通ることを確かめました。Windows では WSL の中で動かすので、まちがった警報も出ません。
**これからやること(確認はいりません。自動で順に進めます)**:
1. 毎晩・毎朝の自動実行の登録(まちがい警報が出ないことを確かめてから)
2. メールが本当に届くかの確認(このPCにはメール送信の設定がありません)
3. aruaru-llm・Kokkos・open-cuda・open-directx での GPU 実行の確認
4. PHP の細かい残り(クラス部品のメソッドなど)
5. 8 か国語の文書を最新に合わせる
**人に聞くこと**: 元の COBOL_to / Java_to を消す作業だけは、元に戻せないので、最後に必ずあなたに聞きます。
**ルール**: 「未了(まだ終わっていないこと)は確認せず、自動で進める」と決まっています。

### English (simple words, easy to read)
**What it can do now**: pl2pl is a helper robot. It rebuilds web apps written in PHP (like Laravel), Python or Java into Python FastAPI. It can change routes, data models, Blade screens and middleware. All 55 end-to-end checks pass.
**What we fixed today**: The "check new versions by itself" feature (`pl2pl watch`) used old tools for its tests. So a pass did not really prove anything. Now every check starts in a **clean new room (a fresh venv), installs the newest version, and then runs the tests**. We confirmed it really passes on fastapi 0.165. On Windows it runs inside WSL, so false alarms are gone.
**What comes next (no need to ask; we go on by ourselves, in order)**:
1. Register the nightly and morning runs (after we are sure there are no false alarms)
2. Check that e-mail really arrives (this PC has no mail settings)
3. Check GPU runs with aruaru-llm, Kokkos, open-cuda and open-directx
4. Small PHP leftovers (class-component methods, and so on)
5. Bring the 8 other language docs up to date
**What we still ask a person**: deleting the original COBOL_to / Java_to cannot be undone, so we always ask you first.
**Rule**: "Unfinished work goes on automatically, without asking." This is the standing rule.

- 2026-11-19 追記 / Update: 夜 03:00・朝 07:00 の自動実行を登録し、手動実行で exit=0(誤警報なし)を確認。/ Nightly (03:00) and morning (07:00) runs are registered; a manual run ended with exit=0 and no false alarm. 次: メール到達・GPU(Kokkos等)・PHP残り。Next: mail arrival, GPU (Kokkos etc.), PHP leftovers.

- 2026-11-19 追記 / Update: Kokkos の CUDA 実行はこのPCでは不可(GT 730=Kepler は WSL の GPU 対象外・Kokkos 新版は sm_35 非対応)。CPU 版(OpenMP/Serial)と Windows の CUDA 11.8 直接実行は確認済み。別の新しい GPU の機械で試す。/ Kokkos CUDA cannot run on this PC (GT 730 Kepler is not supported by WSL GPU, and recent Kokkos drops sm_35). CPU backends and direct CUDA 11.8 on Windows are verified; try on a newer GPU machine. 大規模モデル側: 実3B(2分割)がCPUで動作(約0.6語/秒)。/ Large-model side: the real 3B (2 shards) runs on CPU (about 0.6 tokens/s).

## 最優先機能 / Top-priority feature: 組み合わせを選べる変換 (`pl2pl web --backend ...`) (2026-11-19, 10 languages)

```
pl2pl web <dir> --backend <rust-axum|rust-poem|fastapi|mfastapi|rpoem|rporea>
                [--frontend bootstrap-ts|rust-wasm|none] [--db none|mysql|mariadb|postgres|sqlite] --out <dir>
```
COBOL は変換器で、C / C++ / Fortran は「そのままビルドして呼ぶ橋渡し」(`legacy/build.sh`)で、選んだ組み合わせのアプリの骨組みになります。Axum 版は実際に起動して呼び出しの成功を確認済みです(COBOL と C/C++)。RPoem / RPoRea は Bootstrap・TypeScript なし(Rust→WASM)。

### 日本語
やさしく言うと、「古いプログラム(COBOL・C・C++・Fortran)を、今のウェブのしくみに乗せる」機能です。サーバーは Axum・Poem・FastAPI・RPoem・RPoRea から、画面は HTML5・CSS3・TypeScript・Bootstrap から、データベースは MySQL・MariaDB・PostgreSQL・SQLite から、好きな組み合わせを選べます。COBOL は書き換えて変換します。C・C++・Fortran は元のまま動かし、ウェブから呼べるようにします(まだ Rust への書き換えはしません)。メインフレーム・UNIX・Linux・Windows・Mac の COBOL を別の OS の COBOL へ直す機能は `pl2pl cobol`(IBM→GnuCOBOL)にあります。
### English
In simple words: it puts old programs (COBOL, C, C++, Fortran) onto today's web. You pick the server (Axum, Poem, FastAPI, RPoem or RPoRea), the screen (HTML5, CSS3, TypeScript, Bootstrap) and the database (MySQL, MariaDB, PostgreSQL or SQLite) in any allowed mix. COBOL is rewritten. C, C++ and Fortran keep running as they are, and the web calls them (they are not rewritten into Rust yet). Turning COBOL from one OS into COBOL for another OS is `pl2pl cobol` (IBM to GnuCOBOL).
### 简体中文
简单地说:把旧程序(COBOL、C、C++、Fortran)放到今天的网页上。服务器可选 Axum、Poem、FastAPI、RPoem、RPoRea;界面可选 HTML5、CSS3、TypeScript、Bootstrap;数据库可选 MySQL、MariaDB、PostgreSQL、SQLite,自由组合。COBOL 会被改写;C、C++、Fortran 保持原样运行,由网页调用(暂不改写成 Rust)。把 COBOL 从一个系统转到另一个系统用 `pl2pl cobol`。
### 繁體中文
簡單地說:把舊程式(COBOL、C、C++、Fortran)放到現在的網頁上。伺服器可選 Axum、Poem、FastAPI、RPoem、RPoRea;畫面可選 HTML5、CSS3、TypeScript、Bootstrap;資料庫可選 MySQL、MariaDB、PostgreSQL、SQLite,自由搭配。COBOL 會被改寫;C、C++、Fortran 維持原樣執行,由網頁呼叫(暫不改寫成 Rust)。把 COBOL 從一個系統轉到另一個系統用 `pl2pl cobol`。
### 한국어
쉽게 말하면, 오래된 프로그램(COBOL, C, C++, Fortran)을 지금의 웹에 올려 주는 기능입니다. 서버는 Axum, Poem, FastAPI, RPoem, RPoRea 중에서, 화면은 HTML5, CSS3, TypeScript, Bootstrap 중에서, 데이터베이스는 MySQL, MariaDB, PostgreSQL, SQLite 중에서 마음대로 조합할 수 있습니다. COBOL은 다시 써서 변환하고, C·C++·Fortran은 그대로 실행하면서 웹에서 부릅니다(아직 Rust로 다시 쓰지는 않습니다). COBOL을 다른 OS의 COBOL로 바꾸는 일은 `pl2pl cobol`이 합니다.
### Deutsch
Einfach gesagt: Alte Programme (COBOL, C, C++, Fortran) werden ins heutige Web gebracht. Man wählt Server (Axum, Poem, FastAPI, RPoem oder RPoRea), Oberfläche (HTML5, CSS3, TypeScript, Bootstrap) und Datenbank (MySQL, MariaDB, PostgreSQL oder SQLite) frei im erlaubten Rahmen. COBOL wird umgeschrieben; C, C++ und Fortran laufen unverändert und werden vom Web aus aufgerufen (noch nicht nach Rust umgeschrieben). COBOL für ein anderes Betriebssystem erzeugt `pl2pl cobol`.
### Français
En termes simples : on met de vieux programmes (COBOL, C, C++, Fortran) sur le web d'aujourd'hui. On choisit le serveur (Axum, Poem, FastAPI, RPoem ou RPoRea), l'écran (HTML5, CSS3, TypeScript, Bootstrap) et la base de données (MySQL, MariaDB, PostgreSQL ou SQLite) dans les combinaisons permises. Le COBOL est réécrit ; C, C++ et Fortran tournent tels quels et sont appelés par le web (pas encore réécrits en Rust). Pour du COBOL d'un autre système, voir `pl2pl cobol`.
### Español
En palabras sencillas: lleva programas antiguos (COBOL, C, C++, Fortran) a la web de hoy. Eliges el servidor (Axum, Poem, FastAPI, RPoem o RPoRea), la pantalla (HTML5, CSS3, TypeScript, Bootstrap) y la base de datos (MySQL, MariaDB, PostgreSQL o SQLite) con las combinaciones permitidas. El COBOL se reescribe; C, C++ y Fortran siguen funcionando tal cual y la web los llama (todavía no se reescriben en Rust). Para COBOL de otro sistema operativo, usa `pl2pl cobol`.
### Português
Em palavras simples: leva programas antigos (COBOL, C, C++, Fortran) para a web de hoje. Você escolhe o servidor (Axum, Poem, FastAPI, RPoem ou RPoRea), a tela (HTML5, CSS3, TypeScript, Bootstrap) e o banco de dados (MySQL, MariaDB, PostgreSQL ou SQLite) nas combinações permitidas. O COBOL é reescrito; C, C++ e Fortran continuam rodando como estão e a web os chama (ainda não são reescritos em Rust). Para COBOL de outro sistema, use `pl2pl cobol`.
### Русский
Простыми словами: старые программы (COBOL, C, C++, Fortran) переносятся в сегодняшний веб. Сервер выбирается из Axum, Poem, FastAPI, RPoem, RPoRea; интерфейс — из HTML5, CSS3, TypeScript, Bootstrap; база данных — из MySQL, MariaDB, PostgreSQL, SQLite, в допустимых сочетаниях. COBOL переписывается; C, C++ и Fortran работают как есть, а веб их вызывает (в Rust пока не переписываются). COBOL для другой ОС делает `pl2pl cobol`.

**確認したこと / Verified**: Axum(COBOL 7本・C・C++)を起動して `POST /cobol_to.Program/Run` が正しい出力を返す。全組み合わせの検査は単体試験。**まだ / Not yet**: gfortran の実機ビルド(この PC の WSL に無い)、C/C++/Fortran→Rust の書き換え、COBOL→別 OS の COBOL の拡張、Axum 以外の起動確認。**進め方 / How we work**: 未了は確認せず自動で進める(元のリポジトリの削除だけは最後に確認)。 / Unfinished items go on automatically without asking (only deleting the original repos is confirmed first).

## 2026-11-19 大きなAIを小さなパソコンで動かす / Running big AI on a small PC (10 languages, easy words)

### 日本語
**できたこと**: 大きなAIは、重さが「いくつかのファイル」に分かれていることがあります。それを自動でぜんぶダウンロードして、読めるようになりました。本物のAI(Qwen 3B、約6GB)を、GPUのないふつうのパソコンで動かして、文章を作れました。ちょっと遅いけれど、ちゃんと動きます。
**わかったこと**: 東芝や富士通の「最適な組み合わせを探す機械」は、AIを直接速くする道具ではありません。「どの部品を何ビットに縮めるか」を決める手伝いに使えるかもしれません。作ったあとの記憶の圧縮(MLA)は、品質が落ちるので使えませんでした。
**これから**: AIの重さを小さい数で持って、もっと大きなAI(約12GB)も動かします。「確認しないで進める」ので、止まらずに作って、試して、直します。

### English
**Done**: Big AI models often come in several files. The program now downloads every part and loads them. We ran a real AI (Qwen 3B, about 6 GB) on an ordinary PC without a GPU, and it wrote text. It is a bit slow, but it works.
**Learned**: The "best-combination finder" machines from Toshiba and Fujitsu do not make AI faster by themselves. They might help decide how many bits each part gets. Squeezing the memory afterwards (MLA) hurt quality, so we cannot use it.
**Next**: Keep the AI's weights in small numbers so even bigger AI (about 12 GB) can run. We go on without asking each time: build, test and fix again and again.

### 简体中文
**已完成**:大型AI的权重常被拆成好几个文件。现在程序能自动全部下载并读取。我们在没有显卡的普通电脑上运行了真正的AI(Qwen 3B,约6GB),它能写出文字。虽然有点慢,但是能用。
**发现**:东芝和富士通那种"找最佳组合的机器"不能直接让AI变快,也许能帮忙决定每个部分用几位数。事后压缩记忆(MLA)会让质量变差,所以不能用。
**接下来**:用更小的数字保存权重,让更大的AI(约12GB)也能运行。我们不会每次都问,而是不停地做、测、改。

### 繁體中文
**已完成**:大型AI的權重常被拆成好幾個檔案。現在程式能自動全部下載並讀取。我們在沒有顯示卡的一般電腦上執行了真正的AI(Qwen 3B,約6GB),它能寫出文字。雖然有點慢,但可以用。
**發現**:東芝和富士通那種「找最佳組合的機器」不能直接讓AI變快,也許能幫忙決定每個部分用幾位元。事後壓縮記憶(MLA)會讓品質變差,所以不能用。
**接下來**:用更小的數字保存權重,讓更大的AI(約12GB)也能執行。我們不會每次都詢問,而是不停地做、測、改。

### 한국어
**해낸 일**: 큰 AI는 무게(가중치)가 여러 파일로 나뉘어 있을 때가 많아요. 이제 프로그램이 전부 자동으로 내려받아 읽을 수 있어요. 그래픽카드가 없는 보통 컴퓨터에서 진짜 AI(Qwen 3B, 약 6GB)를 돌려 글을 만들었어요. 조금 느리지만 잘 돼요.
**알게 된 것**: 도시바와 후지쯔의 "가장 좋은 조합을 찾는 기계"는 AI를 직접 빠르게 하지는 않아요. 각 부분을 몇 비트로 줄일지 정하는 데 도움이 될 수는 있어요. 나중에 기억을 줄이는 방법(MLA)은 품질이 떨어져서 쓸 수 없었어요.
**앞으로**: 무게를 더 작은 숫자로 들고 있어서 더 큰 AI(약 12GB)도 돌릴 거예요. 매번 묻지 않고 만들고, 시험하고, 고치기를 계속해요.

### Deutsch
**Geschafft**: Große KI-Modelle bestehen oft aus mehreren Dateien. Das Programm lädt jetzt alle Teile selbst herunter und liest sie. Wir haben eine echte KI (Qwen 3B, etwa 6 GB) auf einem normalen PC ohne Grafikkarte laufen lassen, und sie hat Text geschrieben. Etwas langsam, aber es klappt.
**Gelernt**: Die „Maschinen, die die beste Kombination suchen“ von Toshiba und Fujitsu machen eine KI nicht von selbst schneller. Sie könnten helfen zu entscheiden, wie viele Bits jeder Teil bekommt. Das nachträgliche Zusammendrücken des Gedächtnisses (MLA) machte die Qualität schlechter, deshalb nutzen wir es nicht.
**Als Nächstes**: Die Gewichte in kleineren Zahlen speichern, damit noch größere KI (etwa 12 GB) läuft. Wir machen ohne Rückfrage weiter: bauen, testen, verbessern, immer wieder.

### Français
**Fait** : Les grosses IA sont souvent livrées en plusieurs fichiers. Le programme télécharge maintenant toutes les parties et les lit. Nous avons fait tourner une vraie IA (Qwen 3B, environ 6 Go) sur un PC ordinaire sans carte graphique, et elle a écrit du texte. C'est un peu lent, mais ça marche.
**Appris** : Les « machines qui cherchent la meilleure combinaison » de Toshiba et Fujitsu ne rendent pas l'IA plus rapide toutes seules. Elles pourraient aider à choisir combien de bits donner à chaque partie. Compresser la mémoire après coup (MLA) a abîmé la qualité, donc nous ne l'utilisons pas.
**Ensuite** : Garder les poids en plus petits nombres pour faire tourner de plus grosses IA (environ 12 Go). Nous continuons sans demander à chaque fois : construire, tester, corriger, encore et encore.

### Español
**Hecho**: Las IA grandes suelen venir en varios archivos. Ahora el programa descarga todas las partes y las lee. Ejecutamos una IA real (Qwen 3B, unos 6 GB) en un PC normal sin tarjeta gráfica y escribió texto. Es un poco lento, pero funciona.
**Aprendido**: Las «máquinas que buscan la mejor combinación» de Toshiba y Fujitsu no hacen más rápida a la IA por sí solas. Podrían ayudar a decidir cuántos bits da cada parte. Comprimir la memoria después (MLA) empeoró la calidad, así que no lo usamos.
**Siguiente**: Guardar los pesos con números más pequeños para que corran IA aún más grandes (unos 12 GB). Seguimos sin preguntar cada vez: construir, probar y arreglar una y otra vez.

### Português
**Feito**: As IAs grandes costumam vir em vários arquivos. Agora o programa baixa todas as partes e as lê. Rodamos uma IA de verdade (Qwen 3B, cerca de 6 GB) num PC comum sem placa de vídeo, e ela escreveu texto. É um pouco lento, mas funciona.
**Aprendido**: As «máquinas que procuram a melhor combinação» da Toshiba e da Fujitsu não deixam a IA mais rápida sozinhas. Elas podem ajudar a decidir quantos bits cada parte recebe. Comprimir a memória depois (MLA) piorou a qualidade, então não usamos.
**Próximo**: Guardar os pesos em números menores para rodar IAs ainda maiores (cerca de 12 GB). Seguimos sem perguntar toda vez: construir, testar e consertar, de novo e de novo.

### Русский
**Сделано**: Большие ИИ часто поставляются в нескольких файлах. Теперь программа сама скачивает все части и читает их. Мы запустили настоящий ИИ (Qwen 3B, около 6 ГБ) на обычном компьютере без видеокарты, и он написал текст. Немного медленно, но работает.
**Узнали**: «Машины, которые ищут лучшую комбинацию» от Toshiba и Fujitsu сами по себе ИИ не ускоряют. Возможно, они помогут решить, сколько бит дать каждой части. Сжатие памяти задним числом (MLA) ухудшило качество, поэтому мы его не используем.
**Дальше**: Хранить веса в меньших числах, чтобы запускать ещё более крупные ИИ (около 12 ГБ). Мы продолжаем без вопросов каждый раз: строим, проверяем, исправляем снова и снова.

- 2026-11-19 追記 / Update: 選べる組み合わせのうち Axum・Poem・FastAPI を実際に起動して確認(COBOL の hello が HELLO WORLD を返し、存在しない名前は not_found、画面 `/` は 200)。RPoem/RPoRea は起動未確認。/ Axum, Poem and FastAPI were started and checked (COBOL hello returns HELLO WORLD, unknown name gives not_found, the screen `/` returns 200). RPoem/RPoRea not started yet. 次: C/C++/Fortran→Rust の書き換え、COBOL→別OSのCOBOL拡張。/ Next: rewriting C/C++/Fortran into Rust; extending COBOL-to-other-OS COBOL.
