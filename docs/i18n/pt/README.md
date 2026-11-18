> Traduzido por IA, ainda não revisado por falantes nativos. Vale o original em japonês (raiz do repositório).

# pl2pl
Um **conversor entre linguagens de programação, frameworks e sistemas operacionais** (v0.1.0 publicada, em desenvolvimento). Há instaladores para Windows (x64/ARM64), Linux (amd64/arm64) e macOS (arm64/x64). A conversão é feita no aplicativo de PC instalado (CLI); a versão web oferece apenas manuais e downloads.
A conversão é conservadora: **o que não pode ser convertido nunca é descartado em silêncio; fica como `TODO` e aparece no relatório com o motivo.**

## O que funciona hoje
| Comando | Descrição |
|---|---|
| `pl2pl cobol` | COBOL (IBM) + JCL → GnuCOBOL (Linux) + bash. Verificado compilando e executando com o GnuCOBOL. |
| `pl2pl omp` | Adiciona `#pragma omp parallel for` a laços C/C++ independentes (com redução de somas). Mesmo resultado do original. |
| `pl2pl php` | Laravel / Symfony → FastAPI (veja abaixo). Testado de ponta a ponta com FastAPI, SQLite e Jinja2 reais. |
| `pl2pl web` | Detecta Java / COBOL / Flask·Django e repassa a java_to, cobol_to, 2fastapi, 2poem. Verificado com arquivos reais. |
| `pl2pl cport` | C de UNIX → cabeçalho de compatibilidade + substituição mecânica + orientações de portabilidade. Verificado com gcc. |
| `pl2pl pyhpc` | Numba `@njit` para funções Python numéricas; `--mpi` gera um esqueleto mpi4py. Mesmo resultado no Numba. |
| `pl2pl kokkos` | Laços independentes → Kokkos (`parallel_for` / `parallel_reduce`). Compilado e executado com Kokkos 4.5.01 (Serial). |
| `pl2pl gpu` | (Protótipo) laços → `.cu` no estilo CUDA / HLSL. Somente saída; a execução em open-cuda / open-directx não foi verificada. |
| `pl2pl llm setup` / `llm start` | Inicia o aruaru-llm incluído e instala um modelo Qwen2.5 compatível com a memória livre. Apenas a lógica de escolha e o --dry-run foram verificados. |
| `pl2pl doctor` | Informações da CPU via open-cpu (embutido). |

## Conversão de PHP (`pl2pl php`)
- **Rotas**: Laravel `Route::get/post/put/patch/delete/any/resource/apiResource/view`, grupos `prefix`, Symfony `#[Route]`.
- **Corpos**: `return` simples, várias instruções, `response()->json($x, 201)`, `noContent()`.
- **Eloquent → SQLAlchemy** (`db.py`, `models.py`): colunas vêm das migrations ou de `$fillable`; os demais campos vão para uma coluna JSON `extra`; `DATABASE_URL` escolhe o banco (SQLite por padrão).
- **Query builder**: `where / orWhere / whereIn / orderBy / latest / oldest / limit / take / offset / skip`, `paginate / simplePaginate` (`?page=N`, JSON como no Laravel, `links()` no Blade), `get / first / firstOrFail / count / exists / delete`, `$request->query('k', padrão)`.
- **Relações**: `hasMany / hasOne / belongsTo / belongsToMany` (tabela pivô gerada), `with`, `whereHas / has / whereDoesntHave / doesntHave`, `attach / detach / sync`.
- **Blade → Jinja2**: saída, `@if`, `@foreach`, `@forelse`, `@for`, `@extends/@section/@yield/@include` etc.
- **Middleware → `Depends`**: `auth` (verificação Bearer simples, **substituir por autenticação real**), `guest`, `throttle:N,M`, middleware próprio «rejeitar conforme uma condição; senão `return $next($request)`».

## Previsto (ainda não feito)
- PHP: `with('a.b')` aninhado, relações `morph`, `cursorPaginate`, componentes/pilhas/diretivas de autorização do Blade, middleware que processa a resposta depois de `$next`.
- Incorporar COBOL_to e Java_to ao pl2pl quando estiverem maduros e apagar os repositórios originais (instrução do proprietário; apagar é irreversível, será confirmado de novo após a incorporação).
- Uma representação intermediária comum (servidor, BD/SQL, interface) para que N origens e M destinos exijam N+M peças e não N×M.
- Integração com o aruaru-llm (explicações e tradução de linguagem natural ficam separadas dos conversores; os conversores nunca dependem de um LLM).
- Execução em GPU com Kokkos e `Kokkos::View`; execução de kernels em open-cuda / open-directx.
- Executar os novos exemplos PHP no CI em Linux arm64 e macOS; mais pares de linguagens de `docs/LANGUAGES.md` (nem todas as combinações serão construídas).

## Observações
As decisões são conservadoras: havendo possível dependência ou efeito colateral, a construção **é deixada sem converter e o motivo é informado**; o desempenho não é garantido. Só foram testados pequenos exemplos próprios, não programas reais grandes.
Documentos: `docs/DESIGN.md`, `docs/LANGUAGES.md`, `docs/COMBINATIONS.md`.
