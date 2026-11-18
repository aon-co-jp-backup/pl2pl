# Java_to 引き継ぎ / Hand-off
## 到達点 (2026-11-18)
初版: 判定58件、ルート抽出(Spring/JAX-RS/Micronaut/Servlet/Spark)、Python 本体翻訳、Rust 骨組み、4OSインストーラーのCI定義。FastAPI e2e 7件・poem/rpoem cargo check 成功。
## 未了 / Open
1. 実プロジェクト(Spring PetClinic 等)での変換・実行確認。
2. CI(release.yml)は未実行。タグ `v0.1.0` を打って各OSの試験を確認する。
3. Rust 側の本体翻訳、Struts/JSF 等の画面系、Kotlin/Scala。
4. pl2pl(統合構想)での共通IR化。
## 再開用メッセージ
「F:\Java_to を再開。PORTING.md の未了1から。まず Spring PetClinic を clone して `java_to convert` → FastAPI 実行確認。」
