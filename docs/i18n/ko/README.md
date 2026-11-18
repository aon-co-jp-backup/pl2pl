> AI가 번역했으며 원어민 검증 전입니다. 일본어 원문(저장소 루트)이 기준입니다.

# pl2pl
**프로그래밍 언어·프레임워크·운영체제 간 변환 도구**(v0.1.0 공개, 개발 중). Windows(x64/ARM64), Linux(amd64/arm64), macOS(arm64/x64)용 설치 프로그램이 있습니다. 변환은 설치형 PC 앱(CLI)에서 하며, 웹 버전은 설명서와 다운로드만 제공합니다.
변환은 보수적으로 합니다. **변환할 수 없는 부분은 조용히 버리지 않고 `TODO`로 남기며, 이유와 함께 보고서에 나열합니다.**

## 현재 동작하는 기능
| 명령 | 설명 |
|---|---|
| `pl2pl cobol` | COBOL(IBM)+JCL → GnuCOBOL(Linux)+bash. GnuCOBOL로 실제 빌드·실행해 검증. |
| `pl2pl omp` | 독립적인 C/C++ 루프에 `#pragma omp parallel for` 추가(합계 리덕션 포함). 원본과 같은 결과. |
| `pl2pl php` | Laravel / Symfony → FastAPI(아래 참고). 실제 FastAPI·SQLite·Jinja2로 통합 시험. |
| `pl2pl web` | Java / COBOL / Flask·Django를 판별해 java_to, cobol_to, 2fastapi, 2poem으로 전달. 실제 파일로 검증. |
| `pl2pl cport` | UNIX의 C → 호환 헤더+기계적 치환+이식 지침. gcc로 검증. |
| `pl2pl pyhpc` | 수치 Python 함수에 Numba `@njit` 추가, `--mpi`로 mpi4py 뼈대 생성. Numba 실행 결과 동일. |
| `pl2pl kokkos` | 독립 루프 → Kokkos(`parallel_for` / `parallel_reduce`). Kokkos 4.5.01(Serial)로 빌드·실행. |
| `pl2pl gpu` | (시험작) 루프 → CUDA 형식 `.cu` / HLSL. 출력만 확인, open-cuda / open-directx 실행은 미검증. |
| `pl2pl llm setup` / `llm start` | 내장 aruaru-llm을 시작하고 여유 메모리에 맞는 Qwen2.5 모델을 자동 설치. 선택 로직과 --dry-run만 검증. |
| `pl2pl doctor` | open-cpu(내장)로 CPU 정보 표시. |

## PHP 변환(`pl2pl php`)
- **라우트**: Laravel `Route::get/post/put/patch/delete/any/resource/apiResource/view`, `prefix` 그룹, Symfony `#[Route]`.
- **본문**: 단순 `return`, 여러 문장, `response()->json($x, 201)`, `noContent()`.
- **Eloquent → SQLAlchemy**(`db.py`, `models.py`): 열은 마이그레이션이나 `$fillable`에서 가져오고, 나머지는 `extra`(JSON) 열에 저장. `DATABASE_URL`로 DB 지정(기본 SQLite).
- **쿼리 빌더**: `where / orWhere / whereIn / orderBy / latest / oldest / limit / take / offset / skip`, `paginate / simplePaginate`(`?page=N`, Laravel과 같은 JSON, Blade `links()`), `get / first / firstOrFail / count / exists / delete`, `$request->query('k', 기본값)`.
- **관계**: `hasMany / hasOne / belongsTo / belongsToMany`(중간 테이블 자동 생성), `with`, `whereHas / has / whereDoesntHave / doesntHave`, `attach / detach / sync`.
- **Blade → Jinja2**: 출력, `@if`, `@foreach`, `@forelse`, `@for`, `@extends/@section/@yield/@include` 등.
- **미들웨어 → `Depends`**: `auth`(간단한 Bearer 판정, **실제 인증으로 교체 필요**), `guest`, `throttle:N,M`, 사용자 정의 "조건이면 거부, 아니면 `return $next($request)`".

## 완성 예정(미완료)
- PHP: 중첩 `with('a.b')`, `morph` 관계, `cursorPaginate`, Blade 컴포넌트·스택·권한 지시문, `$next` 이후 응답을 가공하는 미들웨어.
- COBOL_to와 Java_to가 충분히 발전하면 pl2pl로 통합하고 원 저장소를 삭제(소유자 지시, 삭제는 되돌릴 수 없으므로 통합 후 다시 확인).
- 공통 중간 표현(서버 측, DB/SQL, 화면 측)으로 N개 입력과 M개 출력을 N×M이 아닌 N+M개 부품으로 처리.
- aruaru-llm 연동(설명·자연어 번역은 변환기와 분리, 변환기는 LLM에 의존하지 않음).
- Kokkos GPU 실행과 `Kokkos::View`, open-cuda / open-directx 위 커널 실행.
- Linux arm64·macOS CI에서 새 PHP 샘플 실행, `docs/LANGUAGES.md`에서 더 많은 언어 조합 선정(모든 조합을 만들지는 않음).

## 주의
판정은 보수적입니다. 의존성이나 부작용 우려가 있으면 **변환하지 않고 이유를 보고**하며 성능은 보장하지 않습니다. 시험한 것은 작은 자작 샘플이며 실제 대형 프로그램은 시험하지 않았습니다.
문서: `docs/DESIGN.md`, `docs/LANGUAGES.md`, `docs/COMBINATIONS.md`.
