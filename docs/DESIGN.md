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
