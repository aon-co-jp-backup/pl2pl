# pl2pl

**Languages**: 日本語(このファイル) / [English](docs/i18n/en/README.md) / [简体中文](docs/i18n/zh-CN/README.md) / [繁體中文](docs/i18n/zh-TW/README.md) / [한국어](docs/i18n/ko/README.md) / [Deutsch](docs/i18n/de/README.md) / [Français](docs/i18n/fr/README.md) / [Español](docs/i18n/es/README.md) / [Русский](docs/i18n/ru/README.md) / [Português](docs/i18n/pt/README.md)
(日本語と英語は、やさしい言葉に書き直しました。ほかの言葉も、同じ内容に AI 翻訳しました。ネイティブの確認はまだです)

## pl2pl ってなに?
プログラムは、いろいろな「言葉」(プログラミング言語)で書かれています。
**pl2pl は、ある言葉で書かれたプログラムを、別の言葉に書きかえる道具です。**
たとえば、PHP(Laravel)で作ったサイトを、Python(FastAPI)のサイトに変えられます。
Windows・Linux・Mac で使えます(v0.1.0、まだ作っている途中です)。

- 使うときは、パソコンに入れます。WEBサイトには、説明とダウンロードだけがあります → <https://easy-web.tokyo/pl2pl/>
- 書きかえられない所は、こっそり消しません。`TODO`(あとでやること)として残して、理由を報告書に書きます。
- ゴールは、有名な言葉どうしを、できるだけたくさんの組み合わせで書きかえられるようにすることです。

## いまできること
| コマンド | 何をするか |
|---|---|
| `pl2pl php` | PHP(Laravel / Symfony)→ FastAPI。くわしくは [docs/PHP.md](docs/PHP.md) |
| `pl2pl web` | フォルダの中を見て、Java・COBOL・PHP・Flask/Django を見分けて、合う道具に任せる |
| `pl2pl ir` | (試作)PHP・Python・Java の「道すじ(ルート)」を同じ形にして、JSON・OpenAPI・FastAPI のひな形にする。PHP の変換も、型・関数の本体・モデル・ミドルウェア・画面をこの形にのせてから書き出す |
| `pl2pl watch` | 言葉や道具の新しい版を見つけて、自動で追いつく |
| `pl2pl cobol` | COBOL+JCL → GnuCOBOL(Linux) |
| `pl2pl omp` / `kokkos` / `gpu` | C/C++ のくり返しを、はやく動く形にする(gpu は試作) |
| `pl2pl cport` | UNIX 用の C を、Windows でも Linux でも動く形にする |
| `pl2pl pyhpc` | Python の計算を Numba ではやくする |
| `pl2pl llm setup` | AI の助っ人(aruaru-llm)を入れる(実際に入れる所は、まだ確かめていません) |

## 新しい版に、自動で追いつく
`pl2pl watch` を、毎晩と毎朝に動かします。
1. 言葉や道具の新しい版が出ていないか、調べます(一覧は `registry/ecosystem.toml`)。
2. 新しい版でテストします。通れば、「対応している版」を自動で新しくします。
3. 通らなければ、version-control(AI の助けつき)が直そうとします。
4. それでも直らなければ、登録してあるメールと LINE に知らせます。

注意: laravel・symfony・flask・django などは、いまは「新しい版が出たよ」と知らせるだけです。
GitHub では、毎晩と毎朝に、新しい版でテストが自動で動いています。

## 使い方
```
pl2pl php ./myapp --out ./out
pl2pl web ./project --target fastapi --out ./out
pl2pl ir ./project --out ./out --format openapi
```

## 気をつけること
- pl2pl は慎重です。あぶなそうな所は、変えずに理由を知らせます。はやさは約束できません。
- テストは、小さなサンプルだけです。大きな本物のプログラムでは、まだ試していません。

## これから
- 型や中身(関数の本体)も共通の形にのせて、FastAPI への変換をそこを通すようにする。
- PHP: クラスで作る Blade の部品。
- 本物のメール通知と、AI による自動修正の確認。
- GPU で動かす確認(open-cuda / open-directx)。

## もっと知る
[docs/LANGUAGES.md](docs/LANGUAGES.md)(言葉の一覧) / [docs/COMBINATIONS.md](docs/COMBINATIONS.md)(組み合わせ) / [docs/DESIGN.md](docs/DESIGN.md)(設計。持ち主が書いています) / [components/README.md](components/README.md)(いっしょに入る部品)

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
