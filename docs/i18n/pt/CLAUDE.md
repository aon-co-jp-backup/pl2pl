> Esta é uma tradução feita por IA, ainda não revisada por falantes nativos. A versão de referência é o original em japonês, na raiz do repositório.

# pl2pl – notas de desenvolvimento
- **Papel**: um conversor entre linguagens, frameworks e sistemas operacionais (CLI em Rust, `src/`). As linguagens de origem estão em `docs/LANGUAGES.md` e as combinações em `docs/COMBINATIONS.md`. O proprietário escreve o projeto aos poucos em `docs/DESIGN.md`. **Nunca invente especificações em nome dele.**
- **Direção**: conversão de qualquer linguagem ou framework conhecido para qualquer outro. Não tudo de uma vez: primeiro o que foi escolhido. O site só oferece manuais e downloads; a conversão é feita no aplicativo de PC.
- **Regras**: o que não puder ser convertido fica como `TODO` e é informado (nunca apagar em silêncio). Julgar com cautela (se houver dependência ou efeito colateral possível, não converter). Executar de verdade antes de dizer «pronto». Sem novos REST. Responder somente em japonês.
- **PHP**: `src/php.rs`, `src/blade.rs`, `src/middleware.rs`. Teste de ponta a ponta: `tests/run-php.sh` (FastAPI real, SQLite, Jinja2); testes unitários: `cargo test --release`. No WSL: `wsl -d Ubuntu -- bash <arquivo>`.
- **Atualização automática**: `pl2pl watch` (`src/watch.rs`, `registry/ecosystem.toml`). Entradas sem comando de verificação só aparecem como «não verificado» e nunca são atualizadas sozinhas. Os avisos passam por `--notify-file` do version-control.
- **Armadilhas**: escrever arquivos com japonês usando Python `-X utf8`; scripts de shell em LF; o Git Bash reescreve caminhos `/mnt/...`; para textos cheios de aspas, usar a ferramenta Write em vez de heredoc.
- **COBOL_to / Java_to**: já incorporados em `converters/`. Apagar os repositórios originais não tem volta: pedir confirmação final antes.
- **Sistema de tradução** (linguagem natural): separado dos conversores. Os conversores não dependem de um LLM.
- **Idiomas da documentação**: originais em japonês, mais nove idiomas em `docs/i18n/` (traduções por IA, sem revisão de nativos). Quando um original mudar, atualizar as traduções. Histórico anterior: `docs/HISTORY-ja.md`.
