> 이 문서는 AI 번역이며, 아직 원어민의 검수를 받지 않았습니다. 저장소 루트의 일본어 원문이 기준입니다.

# pl2pl 인수인계
(지난 자세한 기록은 `docs/HISTORY-ja.md`에 있습니다. 일본어입니다.)

## 현재 상태(2026-11-19)
- v0.1.0 공개. Windows·Linux·Mac 설치 프로그램과 CI 테스트가 있습니다.
- PHP → FastAPI는 실제 FastAPI로 종단 간 테스트를 마쳤습니다(49개 항목): 라우트, Eloquent, 쿼리 빌더, Blade, 미들웨어, 관계.
- `pl2pl web`이 이제 PHP도 알아보고 넘겨 줍니다.
- 새 기능: `pl2pl watch`가 새 버전을 찾고, 테스트하고, 고치고, 안 되면 알려 줍니다. version-control에 `--notify-file` 옵션을 추가했습니다.
- COBOL_to와 Java_to는 `converters/`에 합쳤습니다.

## 아직 남은 일(우선순위 순)
1. **확인**: 실제 이메일 도착(`version-control --test-notify`), version-control `--loop`로 하는 자동 수정, 최신 fastapi로 테스트(CI의 매일 밤 실행이 확인해 줌).
2. 진짜 새 버전으로 `watch` 시험(laravel, symfony, spring-boot, flask, django, poem은 아직 미검증으로 취급).
3. 매일 밤·아침 실행 등록(`schtasks /Create` 등. 소유자 본인의 환경 설정이라 아직 하지 않음).
4. 공통 중간 표현(IR) 시제품. 언어 조합을 늘리는 토대입니다.
5. PHP: 클래스로 만든 Blade 컴포넌트, `$attributes`, `@php`, `morphToMany`.
6. aruaru-llm 연동(`llm setup` 실제 설치는 미확인). Kokkos GPU 실행, open-cuda / open-directx에서 커널 실행.
7. 원래의 COBOL_to와 Java_to 삭제(**되돌릴 수 없으니 마지막 확인을 받은 뒤에**).
8. `docs/DESIGN.md`는 소유자가 씁니다. `docs/LANGUAGES.md`가 오래되지 않았는지 확인.

## 테스트
`bash tests/run-all.sh`(WSL) / `bash tests/run-php.sh`(PHP 종단 간) / `cargo test --release`

## 작업 재개용 메시지
"F:\pl2pl 재개. PORTING.md의 '아직 남은 일'을 위에서부터 진행. 변환할 수 없는 부분은 조용히 지우지 말고 TODO로 남기기. 실제로 돌려 확인한 뒤에 완료 보고. 답변은 일본어로만. 단락마다 push."
