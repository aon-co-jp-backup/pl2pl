> 由 AI 翻譯,尚未經母語者校對。以日文原文(儲存庫根目錄)為準。

# pl2pl
**程式語言、框架與作業系統之間的轉換工具**(v0.1.0 已發布,開發中)。提供 Windows(x64/ARM64)、Linux(amd64/arm64)、macOS(arm64/x64)安裝程式。轉換在已安裝的 PC 應用程式(CLI)中進行,網頁版僅提供說明書與下載。
轉換採保守策略:**無法轉換的部分絕不會被悄悄丟棄,而是保留為 `TODO`,並在報告中說明原因。**

## 目前可用的功能
| 指令 | 說明 |
|---|---|
| `pl2pl cobol` | COBOL(IBM)+JCL → GnuCOBOL(Linux)+bash。已用 GnuCOBOL 實際建置並執行驗證。 |
| `pl2pl omp` | 為獨立的 C/C++ 迴圈加上 `#pragma omp parallel for`(含加總歸約)。結果與原程式相同。 |
| `pl2pl php` | Laravel / Symfony → FastAPI(詳見下文)。已用真實的 FastAPI、SQLite、Jinja2 完整測試。 |
| `pl2pl web` | 辨識 Java / COBOL / Flask·Django,並交給 java_to、cobol_to、2fastapi、2poem。已用真實檔案驗證。 |
| `pl2pl cport` | UNIX 的 C → 相容標頭檔+機械式取代+移植指南。已用 gcc 驗證。 |
| `pl2pl pyhpc` | 為數值型 Python 函式加上 Numba `@njit`;`--mpi` 產生 mpi4py 骨架。Numba 下結果相同。 |
| `pl2pl kokkos` | 獨立迴圈 → Kokkos(`parallel_for` / `parallel_reduce`)。已用 Kokkos 4.5.01(Serial)建置並執行。 |
| `pl2pl gpu` | (試作)迴圈 → CUDA 風格 `.cu` / HLSL。僅輸出;在 open-cuda / open-directx 上的執行尚未驗證。 |
| `pl2pl llm setup` / `llm start` | 啟動內建的 aruaru-llm,並依可用記憶體自動安裝合適的 Qwen2.5 模型。僅驗證了選擇邏輯與 --dry-run。 |
| `pl2pl doctor` | 透過 open-cpu(內建)顯示 CPU 資訊。 |

## PHP 轉換(`pl2pl php`)
- **路由**:Laravel 的 `Route::get/post/put/patch/delete/any/resource/apiResource/view`、`prefix` 群組、Symfony 的 `#[Route]`。
- **函式本體**:簡單的 `return`、多條敘述、`response()->json($x, 201)`、`noContent()`。
- **Eloquent → SQLAlchemy**(`db.py`、`models.py`):欄位取自 migration 或 `$fillable`,其餘欄位放入 `extra`(JSON)欄;以 `DATABASE_URL` 指定資料庫(預設 SQLite)。
- **查詢建構器**:`where / orWhere / whereIn / orderBy / latest / oldest / limit / take / offset / skip`、`paginate / simplePaginate`(`?page=N`、與 Laravel 相同的 JSON、Blade 的 `links()`)、`get / first / firstOrFail / count / exists / delete`、`$request->query('k', 預設值)`。
- **關聯**:`hasMany / hasOne / belongsTo / belongsToMany`(自動產生中介表)、`with`、`whereHas / has / whereDoesntHave / doesntHave`、`attach / detach / sync`。
- **Blade → Jinja2**:輸出、`@if`、`@foreach`、`@forelse`、`@for`、`@extends/@section/@yield/@include` 等。
- **中介層 → `Depends`**:`auth`(簡易 Bearer 判斷,**須替換為真正的驗證**)、`guest`、`throttle:N,M`、自訂的「符合條件則拒絕,否則 `return $next($request)`」。

## 規劃中(尚未完成)
- PHP:巢狀 `with('a.b')`、`morph` 關聯、`cursorPaginate`、Blade 的元件/堆疊/授權指令、在 `$next` 之後處理回應的中介層。
- 待 COBOL_to 與 Java_to 開發成熟後併入 pl2pl,並刪除原儲存庫(擁有者的指示;刪除無法復原,併入後會再次確認)。
- 通用中介表示(伺服器端、DB/SQL、畫面端),使 N 個來源與 M 個目標只需 N+M 個零件而非 N×M。
- 與 aruaru-llm 整合(說明與自然語言翻譯與轉換器分離,轉換器不依賴 LLM)。
- Kokkos 的 GPU 執行與 `Kokkos::View`;在 open-cuda / open-directx 上執行核心。
- 在 Linux arm64 與 macOS 的 CI 中執行新的 PHP 範例;從 `docs/LANGUAGES.md` 挑選更多語言組合(不會實作全部組合)。

## 注意
判斷保守:可能有相依或副作用時,**不轉換並說明原因**,不保證效能。僅測試過小型自製範例,尚未在大型真實程式上試驗。
文件:`docs/DESIGN.md`、`docs/LANGUAGES.md`、`docs/COMBINATIONS.md`。
