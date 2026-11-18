# Java_to

**日本語 → English**

Java(2000年頃〜2026年のフレームワーク)のプロジェクトを**自動判定**し、**FastAPI / MFastAPI / Poem / RPoem** のいずれかへ変換するツールです。
*Detects the Java framework(s) of a project (c. 2000–2026) and converts it to FastAPI, MFastAPI, Poem or RPoem.*

- Repository: https://github.com/aon-co-jp/Java_to / License: MIT
- 兄弟 / Siblings: [COBOL_to](https://github.com/aon-co-jp/COBOL_to), [2FastAPI](https://github.com/aon-co-jp/2FastAPI), [2MFastAPI](https://github.com/aon-co-jp/2MFastAPI), [2Poem](https://github.com/aon-co-jp/2Poem); 統合構想 / unification plan: [pl2pl](https://github.com/aon-co-jp/pl2pl)

## 日本語
### 使い方
```
java_to scan <dir>                                   # 判定結果とルートを表示
java_to convert <dir> --target <fastapi|mfastapi|rust-poem|rpoem> --out <dir> [--rpoem-path ../RPoem]
java_to frameworks                                   # 判定できる58件の台帳
```
### 変換できるもの
- **ルート(変換対象)**: Spring MVC/Boot/WebFlux、JAX-RS(Jersey・RESTEasy・Quarkus・Dropwizard)、Micronaut、Servlet(`@WebServlet`)、Spark/Javalin(ラムダ)。
- **クラス**: DTO/record/Lombok → pydantic、enum → Enum、Service → 通常クラス、Spring Data リポジトリ → `InMemoryRepository`(導出クエリ対応)。
- **本体**: 式・文を Python へ翻訳(StringBuilder、String.format、Optional、ResponseEntity 等は `java_rt.py`)。
- **Rust(poem / rpoem)**: ルーティングと型の骨組み。本体は単純な `return` 以外 `TODO`。
- **報告のみ**: Struts・JSF・Tapestry・Wicket・GWT・Vaadin・Play・SOAP・Swing/JavaFX・Android など(画面・設定の移行は手作業)。
### 正直な制限
- 訳せない構文は黙って落とさず `raise NotImplementedError("TODO(Java_to): …")` と `# Java:` コメントで残し、`JAVA_to-REPORT.md` に集計します。
- 実行して確認したのは FastAPI 出力(`tests/e2e`、サンプル4種、7件成功)と Poem/RPoem の `cargo check` のみ。大規模な実プロジェクトでの検証は未了。
- Shift_JIS/EUC-JP のソースは ASCII 部分のみ読みます。Groovy/Kotlin/Scala は対象外。RPoem に PATCH は無く PUT へ寄せ警告します。
### インストーラー
Windows x64/ARM64 `Java_to-Setup-*.exe`、Linux `.deb`/`.tar.gz`、macOS `.pkg`(未署名)。タグ `v*` で CI が作成し、インストール→実行→削除を試験します。
### 試験
`bash tests/run-e2e.sh`(FastAPI を実行)、`tests/run-rust.sh`、`tests/run-rpoem.sh`(`../RPoem` が必要)、`tests/run-all.sh`。

## English
Same commands as above. Converts routes (Spring, JAX-RS family, Micronaut, Servlet, Spark/Javalin), DTOs/records/enums/services/Spring Data repositories and method bodies to Python; Rust targets get routing and typed scaffolding only. UI frameworks (Struts, JSF, Wicket, GWT…), SOAP, Swing/JavaFX and Android are report-only. Untranslatable code is never dropped silently: it becomes a `TODO(Java_to)` and is listed in `JAVA_to-REPORT.md`. Verified so far: FastAPI output of 4 samples runs under pytest (7 pass); Poem/RPoem outputs pass `cargo check`. Not yet tried on large real projects.
