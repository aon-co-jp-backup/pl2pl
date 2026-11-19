# pl2pl

**Languages**: 日本語(このファイル) / [English](docs/i18n/en/README.md) / [简体中文](docs/i18n/zh-CN/README.md) / [繁體中文](docs/i18n/zh-TW/README.md) / [한국어](docs/i18n/ko/README.md) / [Deutsch](docs/i18n/de/README.md) / [Français](docs/i18n/fr/README.md) / [Español](docs/i18n/es/README.md) / [Русский](docs/i18n/ru/README.md) / [Português](docs/i18n/pt/README.md)
(AI翻訳です。ネイティブの確認はまだです。CLAUDE.md と PORTING.md も同じフォルダにあります)

**pl2pl は、プログラミング言語やフレームワークを、別のものへ変えるツールです。**
たとえば、PHP(Laravel)で作ったサイトを、Python(FastAPI)に変えられます。Windows・Linux・Mac で使えます(v0.1.0、開発中)。

- 変換は、パソコンに入れるアプリ(コマンド)で行います。WEBサイトは、説明書とダウンロードだけです。
- 変換できない所は、黙って消さずに `TODO` として残し、理由を報告書に書きます。
- 目標は、有名な言語やフレームワークどうしを、できるだけ多くの組み合わせで変えられるようにすることです。

## 今できること
| コマンド | 内容 |
|---|---|
| `pl2pl php` | PHP(Laravel / Symfony)→ FastAPI。くわしくは [docs/PHP.md](docs/PHP.md) |
| `pl2pl web` | フォルダを調べて、Java・COBOL・PHP・Flask/Django を自動で見分け、合う変換器に回す |
| `pl2pl cobol` | COBOL+JCL → GnuCOBOL(Linux) |
| `pl2pl omp` / `kokkos` / `gpu` | C/C++ の繰り返し処理を、OpenMP・Kokkos・GPU用に変える(gpu は試作) |
| `pl2pl cport` | UNIX 用の C を、WindowsでもLinuxでも動く形へ |
| `pl2pl pyhpc` | Python の計算を Numba で速くする |
| `pl2pl watch` | 言語やフレームワークの新しい版を見つけて、自動で追いつく(下を見てください) |
| `pl2pl llm setup` | AI補助(aruaru-llm)を、パソコンに合わせて入れる(実際の導入は未確認) |

## 新しい版へ、自動で追いつく
`pl2pl watch` を毎晩・毎朝に動かします。
1. 言語やフレームワークの新しい版を、調べます(`registry/ecosystem.toml` の一覧)。
2. 新しい版でテストして、通れば「対応済みの版」を自動で更新します。
3. 通らなければ、version-control(AIの助けつき)が直そうとします。
4. それでも直らなければ、登録してあるメールとLINEに知らせます。

毎晩・毎朝の動かし方は `scripts/pl2pl-watch.cmd`(Windows)と `scripts/pl2pl-watch.sh`(Linux/Mac)にあります。
注意: laravel・symfony・flask・django などは、まだ「新しい版が出たよ」と知らせるだけです(本物の新しい版でのテストは、これから作ります)。

## 使い方の例
```
pl2pl php ./myapp --out ./out
pl2pl web ./project --target fastapi --out ./out
```

## 注意
- 変換は慎重です。危なそうな所は、変えずに理由を報告します。速さは保証しません。
- テストは、小さなサンプルだけです。大きな本物のプログラムでは、まだ試していません。

## これから
- PHP: `@can`(権限)と、クラスで作る Blade 部品。
- 共通の中間表現(IR)を作り、言語×言語の組み合わせを一気に増やす。
- 実際のメール通知・AIによる自動修正の確認。
- GPU での実行確認(open-cuda / open-directx)。

## もっと知る
[docs/LANGUAGES.md](docs/LANGUAGES.md)(言語の一覧) / [docs/COMBINATIONS.md](docs/COMBINATIONS.md)(組み合わせ) / [docs/DESIGN.md](docs/DESIGN.md)(設計。所有者が書いています) / [components/README.md](components/README.md)(同梱の部品)
