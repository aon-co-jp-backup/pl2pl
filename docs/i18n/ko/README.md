> 이 문서는 AI가 번역한 것으로, 아직 원어민의 검수를 거치지 않았습니다. 저장소 루트의 일본어 원문이 기준입니다.

# pl2pl
pl2pl은 프로그래밍 언어, 프레임워크, 운영체제 사이에서 코드를 변환해 주는 도구입니다. 현재 v0.1.0이며 계속 개발 중입니다. Windows(x64/ARM64), Linux(amd64/arm64), macOS(arm64/x64)용 설치 프로그램이 있습니다. 변환은 설치한 데스크톱 앱(CLI)에서 이루어지고, 웹사이트에서는 설명서와 다운로드만 제공합니다.
변환은 신중하게 합니다. **변환하지 못한 부분은 조용히 빼 버리지 않고 결과물에 `TODO`로 남기며, 그 이유와 함께 보고서에 적습니다.**

## 지금 쓸 수 있는 기능
| 명령 | 기능 |
|---|---|
| `pl2pl cobol` | IBM COBOL과 JCL을 Linux의 GnuCOBOL과 bash로 변환합니다. GnuCOBOL로 실제 빌드하고 실행해 확인했습니다. |
| `pl2pl omp` | 서로 독립적인 C/C++ 반복문에 `#pragma omp parallel for`를 붙입니다(합계 리덕션 포함). 원본과 같은 결과가 나옵니다. |
| `pl2pl php` | Laravel / Symfony를 FastAPI로 변환합니다(자세한 내용은 아래). 실제 FastAPI, SQLite, Jinja2로 종단 간 시험을 마쳤습니다. |
| `pl2pl web` | Java, COBOL, Flask/Django를 자동으로 알아보고 java_to, cobol_to, 2fastapi, 2poem 중 알맞은 도구에 넘깁니다. 실제 파일로 확인했습니다. |
| `pl2pl cport` | UNIX용 C 코드를 호환 헤더, 기계적 치환, 이식 안내로 바꿉니다. gcc로 확인했습니다. |
| `pl2pl pyhpc` | 수치 계산 Python 함수에 Numba `@njit`을 붙이고, `--mpi`를 주면 mpi4py 뼈대도 만듭니다. Numba에서 같은 결과가 나옵니다. |
| `pl2pl kokkos` | 독립적인 반복문을 Kokkos(`parallel_for` / `parallel_reduce`)로 바꿉니다. Kokkos 4.5.01(Serial)로 빌드하고 실행했습니다. |
| `pl2pl gpu` | 시험 기능입니다. 반복문을 CUDA 형식의 `.cu`나 HLSL로 바꿉니다. 생성된 출력만 확인했고, open-cuda / open-directx에서의 실행은 아직 해 보지 않았습니다. |
| `pl2pl llm setup` / `llm start` | 내장된 aruaru-llm을 띄우고, 남은 메모리에 맞는 Qwen2.5 모델을 자동으로 설치합니다. 모델 선택 로직과 `--dry-run`만 확인했으며 실제 설치는 아직 확인하지 못했습니다. |
| `pl2pl doctor` | 내장된 open-cpu로 CPU 정보를 보여 줍니다. |

## PHP 변환(`pl2pl php`)
- **라우트**: Laravel의 `Route::get/post/put/patch/delete/any/resource/apiResource/view`, `prefix` 그룹, Symfony의 `#[Route]`.
- **본문**: 단순한 `return`, 여러 문장, `response()->json($x, 201)`, `noContent()`.
- **Eloquent → SQLAlchemy**(`db.py`, `models.py`): 컬럼은 마이그레이션이나 `$fillable`에서 가져오고, 나머지는 JSON 형식의 `extra` 컬럼에 담습니다. `DATABASE_URL`로 데이터베이스를 지정합니다(기본값은 SQLite).
- **쿼리 빌더**: `where / orWhere / whereIn / orderBy / latest / oldest / limit / take / offset / skip`, `paginate / simplePaginate / cursorPaginate`(`?page=N`을 읽고 Laravel과 같은 구조의 JSON을 돌려주며, Blade에서는 `links()`를 쓸 수 있습니다), `get / first / firstOrFail / count / exists / delete`, 그리고 `$request->query('k', 기본값)`.
- **모델 관계**: `hasMany / hasOne / belongsTo / belongsToMany / morphMany / morphOne / morphTo`(중간 테이블 자동 생성), `with`(중첩된 `with('posts.comments')` 포함), `whereHas / has / whereDoesntHave / doesntHave`, `attach / detach / sync`.
- **Blade → Jinja2**: 출력 표현식, `@if`, `@foreach`, `@forelse`, `@for`, `@extends/@section/@yield/@include` 등.
- **미들웨어 → `Depends`**: `auth`(간단한 Bearer 확인일 뿐이므로 **실제 인증으로 바꿔 쓰세요**), `guest`, `throttle:N,M`, 그리고 “조건에 맞으면 거부하고, 아니면 `return $next($request)`” 형태의 사용자 정의 미들웨어.

## 앞으로의 계획(아직 안 된 것)
- PHP: `@can` 같은 권한 지시문과 클래스 기반 Blade 컴포넌트(익명 `<x-...>` 컴포넌트는 이미 지원), `$next` 이후에 응답 본문을 바꾸는 미들웨어(헤더 추가는 이미 지원).
- COBOL_to와 Java_to는 이제 `converters/`에 들어와 있습니다(이력을 포함한 스냅숏, 시험 통과). 남은 일: `pl2pl web`이 내장된 사본을 쓰도록 바꾸기, 원본에서 이후에 생긴 변경 따라잡기, 그리고 원래 저장소 삭제(소유자 지시이며, 삭제는 되돌릴 수 없으므로 먼저 확인합니다).
- 공통 중간 표현(서버 쪽, DB/SQL, 화면 쪽)을 만들어, 입력 N개와 출력 M개를 N×M이 아니라 N+M개의 부품으로 처리합니다.
- aruaru-llm 연동. 설명이나 자연어 번역은 변환기와 분리하며, 변환기 자체는 LLM에 의존하지 않습니다.
- Kokkos의 GPU 실행과 `Kokkos::View`, 그리고 open-cuda / open-directx에서 커널 실행.
- `docs/LANGUAGES.md`에서 고른 언어 조합 추가(모든 조합을 만들지는 않습니다).

## 알아 두실 점
pl2pl은 보수적으로 판단합니다. 의존 관계나 부작용이 있을 수 있는 구문은 **변환하지 않고 이유를 보고서에 적으며**, 성능은 보장하지 않습니다. 지금까지는 직접 작성한 작은 샘플만 시험했고, 실제 대규모 프로그램은 아직 시험하지 못했습니다.
더 읽을거리: `docs/DESIGN.md`, `docs/LANGUAGES.md`, `docs/COMBINATIONS.md`.
