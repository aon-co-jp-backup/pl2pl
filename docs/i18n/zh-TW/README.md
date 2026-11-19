> 本文為 AI 翻譯,尚未經母語人士審閱。請以儲存庫根目錄的日文原文為準。

# pl2pl

**pl2pl 可以把用某種程式語言或框架寫的程式碼,轉換成另一種。**
例如,把用 PHP(Laravel)做的網站,變成 Python(FastAPI)的網站。支援 Windows、Linux 和 Mac(v0.1.0,開發中)。

- 轉換是在你電腦上安裝的應用程式(命令列工具)裡進行。網站只提供說明書和下載。
- 遇到無法轉換的地方,pl2pl 不會默默刪掉,而是留下 `TODO`,並在報告裡寫明原因。
- 目標是讓知名的語言和框架之間,盡可能多地互相轉換。

## 現在能做什麼
| 指令 | 作用 |
|---|---|
| `pl2pl php` | PHP(Laravel / Symfony)→ FastAPI。詳情見 [PHP.md](../../PHP.md)(日文) |
| `pl2pl web` | 檢查資料夾,自動辨識 Java、COBOL、PHP、Flask/Django,再交給合適的轉換器 |
| `pl2pl cobol` | COBOL + JCL → GnuCOBOL(Linux) |
| `pl2pl omp` / `kokkos` / `gpu` | 把 C/C++ 的迴圈改寫成 OpenMP、Kokkos 或 GPU 版本(gpu 為試作) |
| `pl2pl cport` | 讓 UNIX 的 C 程式碼在 Windows 和 Linux 上都能執行 |
| `pl2pl pyhpc` | 用 Numba 加快 Python 的運算 |
| `pl2pl watch` | 發現語言和框架的新版本,並自動跟上(見下文) |
| `pl2pl llm setup` | 依你的電腦配置安裝 AI 助手(aruaru-llm)(實際安裝尚未驗證) |

## 自動跟上新版本
每天晚上和早上執行 `pl2pl watch`。
1. 檢查語言和框架有沒有新版本(清單在 `registry/ecosystem.toml`)。
2. 用新版本測試,通過就自動更新「已支援的版本」。
3. 沒通過,就由 version-control(借助 AI)嘗試修復。
4. 還是不行,就寄通知給已登記的電子郵件信箱和 LINE。

排程用的腳本是 `scripts/pl2pl-watch.cmd`(Windows)和 `scripts/pl2pl-watch.sh`(Linux/Mac)。
注意:laravel、symfony、flask、django 等目前只會提醒「有新版本了」。用真正的新版本做測試,還有待完成。

## 使用範例
```
pl2pl php ./myapp --out ./out
pl2pl web ./project --target fastapi --out ./out
```

## 請注意
- pl2pl 很謹慎。看起來有風險的地方,會原樣保留並說明原因。不保證速度。
- 目前只在小型範例上測試過,還沒有在大型真實程式上試過。

## 接下來
- PHP: 用類別撰寫的 Blade 元件.
- 共同的中介表示(IR),一次增加更多語言組合。
- 確認真實的電子郵件通知和 AI 自動修復。
- 在 GPU 上執行(open-cuda / open-directx)。

## 了解更多
[LANGUAGES.md](../../LANGUAGES.md)(語言清單) / [COMBINATIONS.md](../../COMBINATIONS.md)(組合) / [DESIGN.md](../../DESIGN.md)(設計,由擁有者撰寫) / [components/README.md](../../../components/README.md)(內建元件)。這些文件為日文。
