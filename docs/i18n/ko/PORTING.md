> AI가 번역했으며 원어민 검증 전입니다. 일본어 원문(저장소 루트)이 기준입니다.

# pl2pl 인수인계
## 현재 상태(2026-11-19)
v0.1.0 공개(Windows/Linux/macOS 설치 프로그램, CI 통합 시험). PHP 변환은 라우트, 여러 문장 본문, Eloquent(SQLAlchemy), 페이지네이션 포함 쿼리 빌더, 관계, Blade(Jinja2), 미들웨어(`Depends`)까지 통합 시험을 마쳤습니다. Numba, Kokkos(Serial), `pl2pl web`은 실제 실행으로 확인했습니다.
## 미완료(우선순위 순)
1. PHP: 중첩 `with`, `morph`, `cursorPaginate`, Blade 컴포넌트·스택·권한, `$next` 이후 응답 가공 미들웨어.
2. COBOL_to / Java_to를 pl2pl로 통합한 뒤 원 저장소 삭제(먼저 확인, 되돌릴 수 없음, 다른 세션이 개발 중).
3. 공통 IR 시제품.
4. aruaru-llm 연동(`llm setup` 실제 설치는 미검증).
5. Kokkos GPU / `Kokkos::View`, open-cuda / open-directx 위 커널.
6. 소유자의 `docs/DESIGN.md` 작성, `docs/LANGUAGES.md` 최신성 확인.
7. Linux arm64·macOS CI에서 새 PHP 샘플 실행.
## 시험
`bash tests/run-all.sh`(WSL), `bash tests/run-php.sh`(PHP 통합), `cargo test --release`.
## 재개용 메시지
"F:\pl2pl 재개. PORTING.md의 1번부터 진행. 변환 못 하는 부분은 조용히 버리지 말고 TODO로 남길 것. 실제 실행으로 확인한 뒤 완료 보고. push는 확인 후."
