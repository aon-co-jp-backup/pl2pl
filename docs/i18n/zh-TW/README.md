> 本文由 AI 翻譯,尚未經母語人士校對,內容以儲存庫根目錄的日文原文為準。

# pl2pl
pl2pl 是一套在程式語言、框架與作業系統之間轉換程式碼的工具,目前為 v0.1.0,仍持續開發中。提供 Windows(x64/ARM64)、Linux(amd64/arm64)與 macOS(arm64/x64)的安裝程式。轉換在安裝後的桌面應用程式(命令列)中進行,網站只提供說明書與下載。
轉換策略偏保守:**凡是無法轉換的部分,絕不會默默丟掉,而是在輸出中保留為 `TODO`,並在報告裡寫明原因。**

## 目前可以使用的功能
| 指令 | 功能 |
|---|---|
| `pl2pl cobol` | 將 IBM COBOL 加 JCL 轉換成 Linux 上的 GnuCOBOL 加 bash。已用 GnuCOBOL 實際建置並執行驗證。 |
| `pl2pl omp` | 為彼此獨立的 C/C++ 迴圈加上 `#pragma omp parallel for`(包含加總歸約),結果與原程式相同。 |
| `pl2pl php` | 將 Laravel / Symfony 轉換成 FastAPI(詳見下文)。已用真實的 FastAPI、SQLite 與 Jinja2 做過端對端測試。 |
| `pl2pl web` | 自動辨識 Java、COBOL 與 Flask/Django,再交給 java_to、cobol_to、2fastapi 或 2poem 處理。已用真實檔案驗證。 |
| `pl2pl cport` | 將 UNIX 的 C 程式碼轉成相容標頭檔,並附上機械式取代與移植說明。已用 gcc 驗證。 |
| `pl2pl pyhpc` | 為數值運算的 Python 函式加上 Numba `@njit`;加上 `--mpi` 還會產生 mpi4py 骨架。在 Numba 下結果一致。 |
| `pl2pl kokkos` | 將獨立迴圈改寫成 Kokkos(`parallel_for` / `parallel_reduce`)。已用 Kokkos 4.5.01(Serial)建置並執行。 |
| `pl2pl gpu` | 試作功能:將迴圈轉成 CUDA 風格的 `.cu` 或 HLSL。目前只檢查了產生的輸出,尚未在 open-cuda / open-directx 上執行。 |
| `pl2pl llm setup` / `llm start` | 啟動內建的 aruaru-llm,並依可用記憶體自動安裝合適的 Qwen2.5 模型。目前只驗證了模型選擇邏輯與 `--dry-run`,實際安裝尚未驗證。 |
| `pl2pl doctor` | 透過內建的 open-cpu 顯示 CPU 資訊。 |

## PHP 轉換(`pl2pl php`)
- **路由**:Laravel 的 `Route::get/post/put/patch/delete/any/resource/apiResource/view`、`prefix` 群組,以及 Symfony 的 `#[Route]`。
- **函式本體**:單純的 `return`、多行敘述、`response()->json($x, 201)` 與 `noContent()`。
- **Eloquent → SQLAlchemy**(`db.py`、`models.py`):欄位取自 migration 或 `$fillable`,其餘欄位存入 JSON 型別的 `extra` 欄。以 `DATABASE_URL` 指定資料庫(預設為 SQLite)。
- **查詢建構器**:`where / orWhere / whereIn / orderBy / latest / oldest / limit / take / offset / skip`、`paginate / simplePaginate / cursorPaginate`(讀取 `?page=N`,回傳與 Laravel 相同結構的 JSON,Blade 中可用 `links()`)、`get / first / firstOrFail / count / exists / delete`,以及 `$request->query('k', 預設值)`。
- **模型關聯**:`hasMany / hasOne / belongsTo / belongsToMany / morphMany / morphOne / morphTo`(自動產生中介表)、`with`(含巢狀的 `with('posts.comments')`)、`whereHas / has / whereDoesntHave / doesntHave`,以及 `attach / detach / sync`。
- **Blade → Jinja2**:輸出運算式、`@if`、`@foreach`、`@forelse`、`@for`、`@extends/@section/@yield/@include` 等。
- **中介層 → `Depends`**:`auth`(僅是簡易的 Bearer 檢查,**請換成真正的驗證機制**)、`guest`、`throttle:N,M`,以及「符合條件就拒絕,否則 `return $next($request)`」形式的自訂中介層。

## 後續規劃(尚未完成)
- PHP:`@can` 之類的授權指令與以類別為基礎的 Blade 元件(匿名的 `<x-...>` 元件已支援),以及在 `$next` 之後修改回應本文的中介層(新增回應標頭已支援)。
- COBOL_to 與 Java_to 已併入 `converters/`(含歷史的快照,測試通過)。剩下的工作:讓 `pl2pl web` 改用內建的副本,跟進原儲存庫之後的改動,然後刪除原儲存庫(擁有者的指示;刪除無法復原,所以會先確認)。
- 建立統一的中介表示(伺服器端、資料庫/SQL、畫面端),讓 N 種來源與 M 種目標只需 N+M 個元件,而不是 N×M 個。
- 串接 aruaru-llm。說明文字與自然語言翻譯會和轉換器分開,轉換器本身不依賴 LLM。
- Kokkos 的 GPU 執行與 `Kokkos::View`,以及在 open-cuda / open-directx 上執行 kernel。
- 從 `docs/LANGUAGES.md` 再挑選一些語言組合(不會做出全部組合)。

## 使用須知
pl2pl 的判斷偏謹慎:只要某段程式碼可能有相依性或副作用,就**不轉換,並在報告中說明原因**,也不保證效能。目前只測試過小型的手寫範例,尚未在大型實際專案上試過。
延伸閱讀:`docs/DESIGN.md`、`docs/LANGUAGES.md`、`docs/COMBINATIONS.md`。
