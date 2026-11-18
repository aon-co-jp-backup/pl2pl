> Esta versão é uma tradução feita por IA e ainda não foi revisada por falantes nativos. Vale o original em japonês, na raiz do repositório.

# pl2pl CLAUDE
- **Papel**: um conversor entre linguagens, frameworks e sistemas operacionais (programa de linha de comando em Rust, em `src/`). As origens possíveis estão em `docs/LANGUAGES.md` e as combinações em `docs/COMBINATIONS.md`. O proprietário vai escrevendo aos poucos a especificação em `docs/DESIGN.md`; não invente especificação em nome dele.
- **Regras**: o que não puder ser convertido fica como `TODO` e vai para o relatório — nunca descarte em silêncio. Execute de verdade antes de dizer que uma tarefa está concluída. Seja cauteloso: havendo possíveis dependências ou efeitos colaterais, não converta.
- **PHP**: `src/php.rs`, `src/blade.rs`, `src/middleware.rs`. Teste de ponta a ponta: `tests/run-php.sh` (FastAPI, SQLite e Jinja2 de verdade). Testes unitários: `cargo test --release`. A partir do WSL: `wsl -d Ubuntu -- bash <arquivo de script>`.
- **Armadilhas**: no Windows, grave arquivos com texto em japonês usando Python com `-X utf8`; mantenha os scripts de shell com quebras de linha LF; o Git Bash reescreve caminhos `/mnt/...`.
- **COBOL_to / Java_to**: outras sessões estão desenvolvendo. Incorpore depois e só apague os originais após uma confirmação final — não há como desfazer.
- **Sistema de tradução** (linguagem natural): continua separado dos conversores.
- **Idiomas da documentação**: japonês na raiz e mais nove idiomas em `docs/i18n/` (traduções feitas por IA, ainda sem revisão de falantes nativos). Quando o original em japonês mudar, atualize também as traduções.
