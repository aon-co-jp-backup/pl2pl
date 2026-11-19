> Nota: o CLAUDE.md em japonês na raiz do repositório é o original. Esta versão é uma tradução feita por IA, em palavras simples, e ainda não foi revisada por falantes nativos.

# pl2pl – notas de desenvolvimento
- **Papel**: uma ferramenta que reescreve código entre linguagens, frameworks e sistemas operacionais (feita em Rust, em `src/`). As linguagens de origem estão em `docs/LANGUAGES.md` e as combinações em `docs/COMBINATIONS.md`. O proprietário escreve o projeto em `docs/DESIGN.md`, aos poucos. **Nunca invente a especificação em nome dele.**
- **Direção**: reescrever «de muitos para muitos» entre linguagens conhecidas. Não construir tudo de uma vez; começar pelo que foi escolhido. O site só tem guias e downloads. A reescrita acontece no aplicativo de PC.
- **Regras**: o que não puder ser reescrito fica como `TODO` e é informado (nunca esconder). Ser cuidadoso (se houver dependência ou efeito colateral possível, não mudar nada). Executar de verdade antes de dizer «pronto». Sem novos REST. Responder somente em japonês.
- **Forma comum (IR)**: `src/ir.rs`. O lado de entrada (PHP, Python, Java) e o de saída (JSON, OpenAPI, FastAPI) ficam separados, para facilitar a inclusão de novos pares. Arquivos sem rotas encontradas são informados. Conferir a saída de verdade, por exemplo com `py_compile`.
- **PHP**: `src/php.rs`, `src/blade.rs`, `src/middleware.rs`. Teste de ponta a ponta: `tests/run-php.sh` (FastAPI real, SQLite, Jinja2). Testes unitários: `cargo test --release`. No WSL: `wsl -d Ubuntu -- bash <arquivo>`.
- **Atualização automática**: `pl2pl watch` (`src/watch.rs`, `registry/ecosystem.toml`). Itens sem comando de verificação só aparecem como «não verificado» e nunca são atualizados sozinhos. Os alertas passam pelo `--notify-file` do version-control.
- **Publicação**: a página de apresentação é `webpage/index.html`. No VPS: `/root/easy-web.tokyo/pl2pl` (atualiza com `git pull`; é estática, então não precisa reiniciar), `pl2pl-web.service` na porta 8142, já adicionado ao `domains.toml`.
- **Armadilhas**: escrever arquivos com japonês usando Python `-X utf8`. Manter os scripts de shell em LF. O Git Bash reescreve caminhos `/mnt/...`. Para textos com muitas aspas ou `\`, usar as ferramentas Write / Edit, não um heredoc.
- **COBOL_to / Java_to**: já incorporados em `converters/`. Apagar os repositórios originais não tem volta: pedir antes a aprovação final.
- **Sistema de tradução** (língua humana): separado dos conversores. Os conversores não dependem de um LLM.
- **Idiomas da documentação**: os arquivos em japonês são os originais. Os arquivos em inglês foram reescritos em palavras simples. Os demais idiomas em `docs/i18n/` são traduções por IA (ainda sem revisão de nativos). Histórico anterior: `docs/HISTORY-ja.md`.
