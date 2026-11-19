> 안내: 저장소 루트의 일본어 README가 원문입니다. 이 문서는 쉬운 말로 쓴 AI 번역이며, 아직 원어민의 검수를 받지 않았습니다.

# pl2pl

## pl2pl이 뭐예요?
프로그램은 여러 가지 "언어"(프로그래밍 언어)로 쓰여 있습니다.
**pl2pl은 어떤 언어로 쓴 프로그램을 다른 언어로 바꿔 쓰는 도구입니다.**
예를 들어 PHP(Laravel)로 만든 사이트를 Python(FastAPI) 사이트로 바꿀 수 있습니다.
Windows, Linux, Mac에서 쓸 수 있습니다(v0.1.0, 아직 만드는 중입니다).

- 쓰려면 내 컴퓨터에 설치합니다. 웹사이트에는 설명과 다운로드만 있습니다 → <https://easy-web.tokyo/pl2pl/>
- 바꿀 수 없는 부분은 몰래 지우지 않습니다. `TODO`(나중에 할 일)로 남기고 이유를 보고서에 적습니다.
- 목표는 널리 알려진 언어끼리 가능한 한 많은 조합으로 바꿔 쓸 수 있게 하는 것입니다.

## 지금 할 수 있는 일
| 명령 | 하는 일 |
|---|---|
| `pl2pl php` | PHP(Laravel / Symfony) → FastAPI. 자세한 내용은 [PHP.md](../../PHP.md)(일본어) |
| `pl2pl web` | 폴더 안을 살펴 Java, COBOL, PHP, Flask/Django를 알아보고 알맞은 도구에 맡김 |
| `pl2pl ir` | (시험 단계) PHP, Python, Java의 "경로(라우트)"를 같은 모양으로 만들어 JSON, OpenAPI, FastAPI 뼈대로 출력 |
| `pl2pl watch` | 언어와 도구의 새 버전을 찾아 스스로 따라감 |
| `pl2pl cobol` | COBOL + JCL → GnuCOBOL(Linux) |
| `pl2pl omp` / `kokkos` / `gpu` | C/C++ 반복문을 더 빠르게 돌아가게 함(gpu는 시험 단계) |
| `pl2pl cport` | UNIX용 C 코드를 Windows와 Linux 모두에서 돌아가게 함 |
| `pl2pl pyhpc` | Numba로 Python 계산을 빠르게 함 |
| `pl2pl llm setup` | AI 도우미(aruaru-llm)를 설치합니다. 실제 설치는 아직 확인하지 못했습니다 |

## 새 버전을 스스로 따라가기
`pl2pl watch`를 매일 밤과 아침에 실행합니다.
1. 언어와 도구의 새 버전이 나왔는지 확인합니다(목록은 `registry/ecosystem.toml`).
2. 새 버전으로 테스트합니다. 통과하면 "지원하는 버전"을 스스로 새로 고칩니다.
3. 통과하지 못하면 version-control(AI의 도움)이 고쳐 봅니다.
4. 그래도 안 되면 저장해 둔 이메일 주소와 LINE으로 알려 줍니다.

참고: laravel, symfony, flask, django 등은 지금은 "새 버전이 나왔어요"라고 알려 주기만 합니다.
GitHub에서는 매일 밤과 아침에 최신 버전으로 테스트가 자동으로 돌아갑니다.

## 사용법
```
pl2pl php ./myapp --out ./out
pl2pl web ./project --target fastapi --out ./out
pl2pl ir ./project --out ./out --format openapi
```

## 알아 두세요
- pl2pl은 신중합니다. 위험해 보이는 부분은 그대로 두고 이유를 알려 줍니다. 속도는 약속할 수 없습니다.
- 작은 예제로만 시험했습니다. 크고 실제적인 프로그램에서는 아직 해 보지 않았습니다.

## 앞으로
- 자료형과 함수의 내용도 공통 모양에 담고, FastAPI 변환도 그것을 거치게 하기.
- PHP: 클래스로 만든 Blade 부품.
- 실제 이메일 알림과 AI 자동 수정 확인.
- GPU에서 돌아가는지 확인(open-cuda / open-directx).

## 더 알아보기
[LANGUAGES.md](../../LANGUAGES.md)(언어 목록) / [COMBINATIONS.md](../../COMBINATIONS.md)(조합) / [DESIGN.md](../../DESIGN.md)(설계, 소유자가 작성 중) / [components/README.md](../../../components/README.md)(함께 설치되는 부품). 이 파일들은 일본어입니다.
