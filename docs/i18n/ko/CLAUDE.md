> AI가 번역했으며 원어민 검증 전입니다. 일본어 원문(저장소 루트)이 기준입니다.

# pl2pl 개발 메모
- 역할: 언어·프레임워크·OS 간 변환 도구(Rust CLI, `src/`). 변환 원본 후보는 `docs/LANGUAGES.md`, 조합은 `docs/COMBINATIONS.md`. 설계 사양은 소유자가 `docs/DESIGN.md`에 조금씩 작성하므로 임의로 사양을 쓰지 말 것.
- 규칙: 변환할 수 없는 부분은 `TODO`로 남기고 보고서에 기재하며 조용히 버리지 않는다. 완료 보고 전에 실제로 실행해 검증한다. 판정은 보수적(의존성·부작용 우려 시 변환하지 않음).
- PHP: `src/php.rs`, `src/blade.rs`, `src/middleware.rs`; 통합 시험 `tests/run-php.sh`(실제 FastAPI·SQLite·Jinja2), 단위 시험 `cargo test --release`. WSL에서 `wsl -d Ubuntu -- bash <스크립트 파일>`로 실행.
- 함정: Windows에서 일본어가 든 파일은 Python `-X utf8`로 쓴다. 셸 스크립트는 LF 유지. Git Bash는 `/mnt/...` 경로를 바꾼다.
- COBOL_to·Java_to는 다른 세션이 개발 중이며, 나중에 통합한 뒤 최종 확인을 거쳐서만 원 저장소를 삭제(되돌릴 수 없음).
- 번역 시스템(자연어)은 변환기와 분리한다.
- 문서 언어: 일본어(루트)와 `docs/i18n/`의 9개 언어(AI 번역, 원어민 검증 전).
