> Esta versão é uma tradução feita por IA e ainda não foi revisada por falantes nativos. Vale o original em japonês, na raiz do repositório.

# pl2pl
O pl2pl converte código entre linguagens de programação, frameworks e sistemas operacionais. Está na versão 0.1.0 e continua em desenvolvimento ativo. Há instaladores para Windows (x64/ARM64), Linux (amd64/arm64) e macOS (arm64/x64). A conversão é feita no aplicativo de desktop instalado (uma ferramenta de linha de comando); o site traz apenas os manuais e os downloads.
O conversor é cauteloso de propósito. **O que ele não consegue converter nunca é descartado em silêncio: fica no resultado como um `TODO` e aparece no relatório junto com o motivo.**

## O que funciona hoje
| Comando | O que faz |
|---|---|
| `pl2pl cobol` | Converte COBOL da IBM com JCL para GnuCOBOL no Linux com bash. Verificado compilando e executando o resultado com o GnuCOBOL. |
| `pl2pl omp` | Acrescenta `#pragma omp parallel for` aos laços de C/C++ que são independentes entre si, inclusive com redução de somas. O resultado é igual ao do original. |
| `pl2pl php` | Converte Laravel / Symfony para FastAPI (detalhes abaixo). Testado de ponta a ponta com FastAPI, SQLite e Jinja2 de verdade. |
| `pl2pl web` | Reconhece Java, COBOL e Flask/Django e repassa o trabalho ao java_to, cobol_to, 2fastapi ou 2poem. Verificado com arquivos reais. |
| `pl2pl cport` | Transforma C de UNIX em um cabeçalho de compatibilidade, com substituições mecânicas e notas de portabilidade. Verificado com gcc. |
| `pl2pl pyhpc` | Acrescenta Numba `@njit` às funções numéricas em Python; com `--mpi`, gera também um esqueleto de mpi4py. Mesmo resultado no Numba. |
| `pl2pl kokkos` | Reescreve laços independentes como Kokkos (`parallel_for` / `parallel_reduce`). Compilado e executado com Kokkos 4.5.01 (Serial). |
| `pl2pl gpu` | Protótipo: transforma laços em `.cu` no estilo CUDA ou em HLSL. Só a saída gerada foi verificada; a execução em open-cuda / open-directx ainda não foi testada. |
| `pl2pl llm setup` / `llm start` | Inicia o aruaru-llm incluído e instala um modelo Qwen2.5 compatível com a memória livre. Só a escolha do modelo e o `--dry-run` foram verificados, não uma instalação real. |
| `pl2pl doctor` | Mostra informações da CPU por meio do open-cpu, que já vem embutido. |

## Conversão de PHP (`pl2pl php`)
- **Rotas**: Laravel `Route::get/post/put/patch/delete/any/resource/apiResource/view`, grupos `prefix` e Symfony `#[Route]`.
- **Corpos de função**: `return` simples, corpos com várias instruções, `response()->json($x, 201)` e `noContent()`.
- **Eloquent → SQLAlchemy** (`db.py`, `models.py`): as colunas vêm das migrations ou de `$fillable`; o restante vai para uma coluna JSON `extra`. `DATABASE_URL` escolhe o banco de dados (SQLite por padrão).
- **Query builder**: `where / orWhere / whereIn / orderBy / latest / oldest / limit / take / offset / skip`, `paginate / simplePaginate / cursorPaginate` (lê `?page=N`, devolve JSON no formato do Laravel e `links()` funciona no Blade), `get / first / firstOrFail / count / exists / delete` e `$request->query('k', valor padrão)`.
- **Relações**: `hasMany / hasOne / belongsTo / belongsToMany` (a tabela pivô é gerada), `with`, inclusive aninhado (`with('posts.comments')`), `whereHas / has / whereDoesntHave / doesntHave` e `attach / detach / sync`.
- **Blade → Jinja2**: saídas, `@if`, `@foreach`, `@forelse`, `@for`, `@extends/@section/@yield/@include` e mais.
- **Middleware → `Depends`**: `auth` (apenas uma verificação Bearer simples — **troque por uma autenticação de verdade**), `guest`, `throttle:N,M` e middleware próprio no formato «rejeitar se uma condição for atendida; senão `return $next($request)`».

## Planejado (ainda não feito)
- PHP: relações `morph`, componentes, pilhas e diretivas de autorização do Blade, e middleware que altera o corpo da resposta depois de `$next` (adicionar cabeçalhos já funciona).
- Incorporar COBOL_to e Java_to ao pl2pl quando estiverem maduros e apagar os repositórios originais. É uma orientação do proprietário; como apagar não tem volta, será confirmado de novo depois da incorporação.
- Uma representação intermediária comum (lado do servidor, banco de dados/SQL, interface) para que N origens e M destinos precisem de N+M peças em vez de N×M.
- Integração com o aruaru-llm. Explicações e tradução de linguagem natural continuam separadas dos conversores, que nunca dependem de um LLM.
- Kokkos em GPU e `Kokkos::View`, além de executar kernels em open-cuda / open-directx.
- Executar os novos exemplos de PHP no CI em Linux arm64 e macOS e acrescentar mais pares de linguagens de `docs/LANGUAGES.md` (nem todas as combinações serão feitas).

## Vale saber
Na dúvida, o pl2pl opta pela cautela. Se uma construção puder ter dependências ou efeitos colaterais, ela **fica sem converter e o motivo é informado**, e não se promete nenhum desempenho. Até agora só foram testados exemplos pequenos escritos à mão, não programas reais grandes.
Para saber mais: `docs/DESIGN.md`, `docs/LANGUAGES.md`, `docs/COMBINATIONS.md`.
