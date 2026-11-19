> 이 문서는 AI 번역이며, 아직 원어민의 검수를 받지 않았습니다. 저장소 루트의 일본어 원문이 기준입니다.

# pl2pl

**pl2pl은 어떤 프로그래밍 언어나 프레임워크로 만든 코드를 다른 것으로 바꿔 주는 도구입니다.**
예를 들어 PHP(Laravel)로 만든 사이트를 Python(FastAPI) 사이트로 바꿀 수 있습니다. Windows, Linux, Mac에서 쓸 수 있습니다(v0.1.0, 개발 중).

- 변환은 내 컴퓨터에 설치하는 앱(명령줄 도구)에서 합니다. 웹사이트에는 설명서와 다운로드만 있습니다.
- 바꿀 수 없는 부분은 조용히 지우지 않고 `TODO`로 남기며, 이유를 보고서에 적습니다.
- 목표는 널리 알려진 언어와 프레임워크끼리 가능한 한 많은 조합으로 바꿀 수 있게 하는 것입니다.

## 지금 할 수 있는 일
| 명령 | 하는 일 |
|---|---|
| `pl2pl php` | PHP(Laravel / Symfony) → FastAPI. 자세한 내용은 [PHP.md](../../PHP.md)(일본어) |
| `pl2pl web` | 폴더를 살펴 Java, COBOL, PHP, Flask/Django를 스스로 알아보고 알맞은 변환기로 보냄 |
| `pl2pl cobol` | COBOL + JCL → GnuCOBOL(Linux) |
| `pl2pl omp` / `kokkos` / `gpu` | C/C++ 반복문을 OpenMP, Kokkos, GPU용으로 고침(gpu는 시험 단계) |
| `pl2pl cport` | UNIX용 C 코드를 Windows와 Linux 모두에서 돌아가게 함 |
| `pl2pl pyhpc` | Numba로 Python 계산을 빠르게 함 |
| `pl2pl watch` | 언어와 프레임워크의 새 버전을 찾아 자동으로 따라감(아래 참고) |
| `pl2pl llm setup` | 컴퓨터에 맞춰 AI 도우미(aruaru-llm)를 설치(실제 설치는 아직 확인 전) |

## 새 버전을 자동으로 따라가기
`pl2pl watch`를 매일 밤과 아침에 실행합니다.
1. 언어와 프레임워크의 새 버전이 있는지 확인합니다(목록은 `registry/ecosystem.toml`).
2. 새 버전으로 테스트해서 통과하면 "지원하는 버전"을 스스로 바꿉니다.
3. 통과하지 못하면 version-control(AI의 도움)이 고쳐 봅니다.
4. 그래도 안 되면 등록해 둔 이메일과 LINE으로 알려 줍니다.

예약 실행용 스크립트는 `scripts/pl2pl-watch.cmd`(Windows)와 `scripts/pl2pl-watch.sh`(Linux/Mac)입니다.
주의: laravel, symfony, flask, django 등은 아직 "새 버전이 나왔어요"라고 알려 주기만 합니다. 진짜 새 버전으로 하는 테스트는 앞으로 만들 예정입니다.

## 사용 예
```
pl2pl php ./myapp --out ./out
pl2pl web ./project --target fastapi --out ./out
```

## 알아 두세요
- pl2pl은 신중합니다. 위험해 보이는 부분은 바꾸지 않고 이유를 알려 줍니다. 속도는 보장하지 않습니다.
- 작은 예제로만 시험했고, 크고 실제적인 프로그램에서는 아직 시험하지 못했습니다.

## 앞으로
- PHP: 클래스로 만든 Blade 컴포넌트.
- 공통 중간 표현(IR)을 만들어 언어 조합을 한꺼번에 늘리기.
- 실제 이메일 알림과 AI 자동 수정 확인.
- GPU에서 실행 확인(open-cuda / open-directx).

## 더 알아보기
[LANGUAGES.md](../../LANGUAGES.md)(언어 목록) / [COMBINATIONS.md](../../COMBINATIONS.md)(조합) / [DESIGN.md](../../DESIGN.md)(설계, 소유자가 작성 중) / [components/README.md](../../../components/README.md)(포함된 부품). 이 문서들은 일본어입니다.
