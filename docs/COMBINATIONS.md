# 組み合わせと優先フレームワーク / Combinations and priority frameworks (2026-11-18)
知識に基づく見積もり(最新性は未検証)。既存ツールの現状は各リポジトリの記載による。
*Knowledge-based estimate, not freshly verified. Status of existing tools follows each repository's own docs.*

## 1. OS × 言語・モデル / OS x language/model
スパコンはほぼ全て Linux で動く、と理解している(未検証)ので Linux の一形態として扱う。◎標準 ○可 △限定/要確認 ✕不可。
*Supercomputers are understood to run almost entirely on Linux (unverified), so they are treated as a form of Linux.*

| 言語・モデル Language/model | メインフレーム Mainframe | UNIX | Linux (HPC) | Windows | Mac |
|---|---|---|---|---|---|
| C / C++ | ◎ (z/OS XL C/C++) | ◎ | ◎ | ◎ | ◎ |
| COBOL | ◎ | ○ | ○ (GnuCOBOL) | ○ | ○ |
| Python | △ | ○ | ◎ | ◎ | ◎ |
| MPI | △ | ○ | ◎ | ○ (MS-MPI) | ○ |
| OpenMP | △ | ○ | ◎ | ○ | ○ |
| OpenACC | ✕ | △ | ◎ | △ | ✕ |
| CUDA | ✕ | ✕ | ◎ | ◎ | ✕ |
| HIP | ✕ | ✕ | ◎ | △ | ✕ |
| SYCL / oneAPI | ✕ | ✕ | ◎ | ○ | △ |
| OpenCL | ✕ | △ | ○ | ○ | △ |
| Kokkos / RAJA | ✕ | △ | ◎ | ○ | ○ |
| Julia | ✕ | △ | ◎ | ◎ | ◎ |
| Chapel / UPC++ | ✕ | △ | ○ | △ | △ |
| Rust | △ (Linux on Z) | △ | ◎ | ◎ | ◎ |
| Slurm / PBS / LSF | ✕ | △ | ◎ | ✕ | ✕ |

Mac の GPU は Metal が標準で、この表には含めていない。/ Metal is the Mac GPU standard and is not in this table.

## 2. 既存ツール / Existing tools
| ツール Tool | 変換元 → 変換先 | 状態 Status |
|---|---|---|
| COBOL_to | COBOL → FastAPI / MFastAPI / Poem / RPoem | 実装あり(最小構文) |
| Java_to | Java (Spring, JAX-RS, Servlet, Spark…) → same four | v0.1.0 (CI green, installers) |
| 2FastAPI | Flask / Django → FastAPI | 実装あり |
| 2MFastAPI | Flask / Django / FastAPI → MFastAPI | 実装あり |
| 2Poem | Flask / Django / FastAPI → Poem / RPoem | 実装あり |
| MFastAPI / RPoem / RMojo | 変換先の実行基盤 / target runtimes | 実装あり |

## 3. 22言語の現在地 / Where the 22 languages stand
- 既存ツールで変換可 / convertible now: COBOL, Java, Python (Flask/Django/FastAPI)
- 変換先として既にある / already a target: Python (FastAPI/MFastAPI), Rust (Poem/RPoem)
- 未着手 / not started: Access, Access VBA, VBA, Excel VBA, VB, VB.NET, C#.NET, F#, F#.NET, C, C++, Perl, PHP, Ruby, JavaScript, TypeScript, Go, Fortran

## 4. 変換先の全体像 / Target overview
- 既存(Web) / existing: FastAPI, MFastAPI, Poem, RPoem
- 新規(OS移植) / new (OS porting): GnuCOBOL + shell on Linux, standard C / POSIX, Windows ports
- 新規(スパコン) / new (HPC): C/C++/Fortran with MPI + OpenMP; Python with Numba/mpi4py; CUDA/HIP/SYCL via Kokkos

## 5. メインに検討するフレームワーク(統合版) / Frameworks to consider first
| 優先 Prio | 対象 Item | 理由 Reason |
|---|---|---|
| 1 | FastAPI, MFastAPI, Poem, RPoem | 既存4種の変換先 / existing targets |
| 1 | MPI, OpenMP, CUDA, Slurm, CMake | スパコンの共通基盤 / HPC common base |
| 2 | Spring, Flask/Django (input) | 実装済みの入力を維持 / keep existing inputs |
| 2 | PHP (Laravel, Symfony), C# (ASP.NET), Go (Gin) | 次の入力候補。Java_to を流用 / next inputs, reuse Java_to |
| 2 | Kokkos, mpi4py, Numba | HPC の変換先 / HPC targets |
| 3 | Ruby (Rails), Perl, TypeScript (NestJS, Express) | Web系の追加 / more web |
| 3 | VB / VB.NET / VBA / Access | 日本の業務資産で重要、画面+DB一体で難しい / important in Japanese business, hard (UI + DB in one) |
| 4 | SYCL, HIP, Julia, Chapel | 使われる範囲が限られる / niche |

## 6. 最初の変換の組み合わせ(案) / First conversion pairs (draft)
1. COBOL (mainframe) → GnuCOBOL + shell on Linux (extend COBOL_to)
2. COBOL / Java / Python → FastAPI / Poem (existing; small additions)
3. Serial C/C++ → C/C++ with OpenMP (HPC entry; suggest loop parallelisation)
4. PHP → FastAPI / Poem (reuse Java_to design)
