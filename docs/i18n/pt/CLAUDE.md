> Traduzido por IA, ainda não revisado por falantes nativos. Vale o original em japonês (raiz do repositório).

# Notas de desenvolvimento do pl2pl
- Papel: conversor entre linguagens/frameworks/SOs (CLI em Rust, `src/`). Origens candidatas: `docs/LANGUAGES.md`; combinações: `docs/COMBINATIONS.md`. O proprietário escreve aos poucos a especificação em `docs/DESIGN.md`; não inventar especificação em nome dele.
- Regras: o que não puder ser convertido fica como `TODO` e aparece no relatório; nunca descartar em silêncio. Verificar executando de verdade antes de dar por concluído. Decisões conservadoras (não converter diante de possível dependência ou efeito colateral).
- PHP: `src/php.rs`, `src/blade.rs`, `src/middleware.rs`; teste de ponta a ponta `tests/run-php.sh` (FastAPI, SQLite e Jinja2 reais), testes unitários `cargo test --release`. A partir do WSL: `wsl -d Ubuntu -- bash <arquivo de script>`.
- Armadilhas: no Windows, gravar arquivos com japonês usando Python `-X utf8`; manter scripts de shell em LF; o Git Bash reescreve caminhos `/mnt/...`.
- COBOL_to e Java_to são desenvolvidos por outra sessão; incorporar depois e apagar os originais só após a confirmação final (irreversível).
- O sistema de tradução (linguagem natural) é separado dos conversores.
- Idiomas da documentação: japonês (raiz) e 9 idiomas em `docs/i18n/` (traduções por IA, ainda não revisadas por falantes nativos).
