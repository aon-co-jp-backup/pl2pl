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
