> 이 문서는 AI 번역이며, 아직 원어민의 검수를 받지 않았습니다. 저장소 루트의 일본어 원문이 기준입니다.

# pl2pl 개발 메모
- **역할**: 언어·프레임워크·운영체제를 넘나드는 변환 도구(Rust CLI, `src/`). 원본 언어 목록은 `docs/LANGUAGES.md`, 조합은 `docs/COMBINATIONS.md`. 설계는 소유자가 `docs/DESIGN.md`에 조금씩 씁니다. **마음대로 사양을 정하지 마세요.**
- **방향**: 널리 알려진 언어·프레임워크 사이의 다대다 변환을 전제로 합니다. 한꺼번에 만들지 않고 고른 것부터 구현합니다. 웹사이트는 설명서와 다운로드만 제공하고, 변환은 PC 앱에서 합니다.
- **규칙**: 변환할 수 없는 부분은 `TODO`로 남기고 보고합니다(조용히 지우지 않기). 판단은 보수적으로(의존성이나 부작용이 걱정되면 바꾸지 않기). 실제로 돌려 본 뒤에 "완료"라고 말하기. 새 REST는 만들지 않기. 답변은 일본어로만.
- **PHP**: `src/php.rs`, `src/blade.rs`, `src/middleware.rs`. 종단 간 테스트는 `tests/run-php.sh`(실제 FastAPI·SQLite·Jinja2), 단위 테스트는 `cargo test --release`. WSL에서는 `wsl -d Ubuntu -- bash <파일>`.
- **자동 추적**: `pl2pl watch`(`src/watch.rs`, `registry/ecosystem.toml`). 확인 명령이 비어 있는 항목은 "미검증"이라고만 표시하고 절대 자동으로 올리지 않습니다. 알림은 version-control의 `--notify-file`로 보냅니다.
- **함정**: 일본어가 든 파일은 Python `-X utf8`로 쓰기. 셸 스크립트는 LF. Git Bash는 `/mnt/...` 경로를 바꿔 버림. 따옴표가 많은 문자열은 heredoc 대신 Write 도구로.
- **COBOL_to / Java_to**: `converters/`에 합쳤습니다. 원래 저장소의 삭제는 되돌릴 수 없으므로 마지막 확인을 받은 뒤에.
- **번역 시스템**(자연어)은 변환기와 분리합니다. 변환기는 LLM에 의존하지 않습니다.
- **문서 언어**: 원본은 일본어, `docs/i18n/`에 9개 언어(AI 번역, 원어민 검수 전). 원본을 고치면 번역도 고칩니다. 지난 경과는 `docs/HISTORY-ja.md`.
