> 이 문서는 AI가 번역한 것으로, 아직 원어민의 검수를 거치지 않았습니다. 저장소 루트의 일본어 원문이 기준입니다.

# pl2pl 인수인계
## 현재 상황(2026-11-19)
v0.1.0이 공개되었고, Windows·Linux·macOS 설치 프로그램과 CI의 종단 간 시험이 있습니다. PHP 변환은 라우트, 여러 문장 본문, Eloquent(SQLAlchemy), 페이지네이션을 포함한 쿼리 빌더, 모델 관계(중첩 `with` 포함), Blade(Jinja2), 미들웨어(`Depends`)까지 다루며 모두 종단 간 시험을 마쳤습니다. Numba, Kokkos(Serial), `pl2pl web`은 실제 실행으로 확인했습니다.
## 남은 일(우선순위 순)
1. PHP: `morph`, Blade 컴포넌트·스택·권한, `$next` 이후에 동작하는 미들웨어.
2. COBOL_to / Java_to를 pl2pl에 합친 뒤 원래 저장소 삭제(먼저 확인할 것, 되돌릴 수 없음, 다른 세션이 작업 중).
3. 공통 중간 표현 시제품 만들기.
4. aruaru-llm 연동(`llm setup`의 실제 설치는 아직 미확인).
5. Kokkos GPU / `Kokkos::View`, open-cuda / open-directx의 커널.
6. 소유자가 `docs/DESIGN.md`를 작성하고, `docs/LANGUAGES.md`가 최신인지 확인.
7. Linux arm64와 macOS CI에서 새 PHP 샘플 실행.
## 시험
`bash tests/run-all.sh`(WSL), `bash tests/run-php.sh`(PHP 종단 간), `cargo test --release`.
## 작업을 다시 시작할 때 쓸 메시지
“F:\pl2pl 작업을 이어서 하자. PORTING.md의 1번부터 시작. 변환하지 못한 부분은 조용히 빼지 말고 TODO로 남길 것. 실제로 실행해서 확인한 뒤에 완료라고 보고할 것. push하기 전에는 먼저 물어볼 것.”
