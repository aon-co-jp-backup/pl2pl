# 同梱コンポーネント / Bundled components

| 部品 Component | 扱い How it is bundled | 状態 Status |
|---|---|---|
| [open-cpu](https://github.com/aon-co-jp/open-cpu) | **ライブラリとして内蔵**(`Cargo.toml` の git 依存)。`pl2pl doctor` が CPU の命令セット・選択される実装を表示。OpenMP の変換結果を見るときの目安になる。 / **Built in as a library** (git dependency); `pl2pl doctor` prints detected CPU features. | 動作確認済み / verified (`doctor`) |
| [open-cuda](https://github.com/aon-co-jp/open-cuda) | **試作の出力先**: `pl2pl gpu --target open-cuda` が独立ループから CUDA 形式のカーネル(`.cu`)を書き出す。open-cuda 上での実行は未検証。 / **Prototype target**: writes CUDA-style kernels; running on open-cuda is unverified. | 試作 / prototype |
| [open-directx](https://github.com/aon-co-jp/open-directx) | **試作の出力先**: `pl2pl gpu --target open-directx` が HLSL のコンピュートシェーダー(`.hlsl`)を書き出す(open-directx の `directx-shader-translate` への入力を想定)。実行は未検証。 / **Prototype target**: writes HLSL compute shaders; running is unverified. | 試作 / prototype |
| [aruaru-llm](https://github.com/aon-co-jp/aruaru-llm) | **別コンポーネント(任意)**: モデルの重みはモデル次第で約0.5GB(GPT-2)〜数十GB(DeepSeek-V2-Lite級、README では未検証)あり、pl2pl の実行ファイルには入れない(重みの大きさ=パラメータ数×1つあたりのバイト数。全部をメモリに載せて使う)。導入すると、変換できなかった箇所(`TODO`)の説明や提案に使う想定(`POST /aruaru_llm.v1.Llm/<Method>`、Connect)。pl2pl からの呼び出しは**未実装**。 / **Optional separate component**: model weights range from about 0.5 GB (GPT-2) to tens of GB (DeepSeek-V2-Lite class, unverified in its README) because size = parameters x bytes each and all of it is held in memory, so it is not inside the pl2pl binary; intended to explain/suggest for `TODO` parts. The call from pl2pl is **not implemented yet**. | 未連携 / not wired |

## 正直な注記 / Honest notes
- 「同梱」は、open-cpu だけが実際にバイナリへ入っています。他の3つは「出力先」または「別途導入」です。 / Only open-cpu is actually inside the binary; the other three are output targets or separate installs.
- カーネルは要素型を `float` と仮定し、縮約(和)は出力しません。 / Kernels assume `float` and do not emit reductions.

## aruaru-llm の同梱とモデルの自動導入(2026-11-18、所有者の決定) / Bundling aruaru-llm and automatic model install
**日本語**
- **同梱するもの**: aruaru-llm の実行ファイル(公開リリース `aruaru-llm-linux-x86_64.tar.gz` / `aruaru-llm-windows-x86_64.zip`)。配布物を作る前に `bash installer/bundle-components.sh <linux|windows>` で `components/bin/` へ取り込む。**モデルの重みは同梱しない**(GB級のため)。
- **モデルは自動導入**: `pl2pl llm setup` が空きメモリを調べ、aruaru-llm のカタログ(Qwen2.5)から合うものを選ぶ(空き ≥ 重みの約2.5倍の見積もり: 3B=約6.2GB、1.5B=約2.9GB、0.5B=約0.9GB。足りなければ最小+警告)。同梱の aruaru-llm が動いていなければ 127.0.0.1:4600 で自動起動し、`QwenInstall` → `QwenSelect` を呼ぶ。`--dry-run` で選択だけ確認、`--model <id>` で指定。
- **ダウンロードは `llm setup` を実行したときだけ**。変換コマンド(cobol・omp 等)は何もダウンロードしない。
- **未確認**: 同梱版を実際に起動しての導入(`QwenInstall` の実呼び出し)。選択ロジックと `--dry-run` は確認済み。空きメモリ 2.5 倍は私の見積もりで実測していない。
**English**
- **Bundled**: the aruaru-llm executable (public release assets). Pull it into `components/bin/` with `bash installer/bundle-components.sh <linux|windows>` before building a distribution. **Model weights are not bundled** (GB-scale).
- **Models install automatically**: `pl2pl llm setup` checks free memory and picks a fitting model from aruaru-llm's Qwen2.5 catalog (free memory >= about 2.5x the weights, my estimate; smallest plus a warning if short). If the bundled aruaru-llm is not running it starts it on 127.0.0.1:4600, then calls `QwenInstall` and `QwenSelect`. `--dry-run` shows the choice only; `--model <id>` forces one.
- **Downloads happen only when `llm setup` is run.** The conversion commands download nothing.
- **Unverified**: a real install against the bundled binary (`QwenInstall`). The selection logic and `--dry-run` are verified; the 2.5x factor is an unmeasured estimate.
