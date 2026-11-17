# pl2pl 開発メモ / Dev notes
**日本語**
- 役割: COBOL_to・Java_to・2FastAPI・2MFastAPI・2Poem を共通の中間表現(IR)で統合・融合する構想リポジトリ。現時点でコードは無い。設計・仕様は所有者が少しずつ書く(`docs/DESIGN.md`)。勝手に仕様を決めて書き込まないこと。
- 変換元の候補一覧は `docs/LANGUAGES.md`(知識ベース、最新性は未検証)。最初の言語は選定中(案: PHP)。
- IR は3系統に分ける想定: ①サーバー側(ルート・型・本体)②DB/SQL ③画面側(出力は RPoem の Rust→WASM)。
- 既存リポジトリは当面そのまま残す。破壊的な統合はしない。
- 前身として pg2pg という名で作ったが、所有者の指示で削除し pl2pl に改めた(2026-11-18)。
**English**
- Role: planning repository to unify COBOL_to, Java_to, 2FastAPI, 2MFastAPI and 2Poem on a shared intermediate representation. No code yet. The owner writes design/spec gradually (`docs/DESIGN.md`); do not invent spec on their behalf.
- Source candidates: `docs/LANGUAGES.md` (knowledge-based, not freshly verified). First language undecided (suggestion: PHP).
- Three IR families expected: server side, DB/SQL, UI side (RPoem Rust→WASM).
- Existing repositories stay as they are; no destructive merge.
- Created first as `pg2pg`, deleted at the owner's request and renamed `pl2pl` (2026-11-18).
