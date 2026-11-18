> Esta versão é uma tradução feita por IA e ainda não foi revisada por falantes nativos. Vale o original em japonês, na raiz do repositório.

# pl2pl: passagem de bastão
## Como estão as coisas (2026-11-19)
A versão 0.1.0 está publicada, com instaladores para Windows, Linux e macOS e testes de ponta a ponta no CI. A conversão de PHP cobre rotas, corpos com várias instruções, Eloquent (SQLAlchemy), o query builder com paginação, relações (inclusive `with` aninhado), Blade (Jinja2) e middleware (`Depends`), tudo testado de ponta a ponta. Numba, Kokkos (Serial) e `pl2pl web` foram confirmados com execuções reais.
## Pendências, por ordem de prioridade
1. PHP: `morph`, `cursorPaginate`, componentes/pilhas/autorização do Blade, middleware que age depois de `$next`.
2. Incorporar COBOL_to / Java_to ao pl2pl e depois apagar os originais (confirmar antes; irreversível; outras sessões estão trabalhando neles).
3. Protótipo da representação intermediária comum.
4. Integração com o aruaru-llm (uma instalação real com `llm setup` ainda não foi verificada).
5. Kokkos em GPU / `Kokkos::View`; kernels em open-cuda / open-directx.
6. O proprietário escreve `docs/DESIGN.md`; conferir se `docs/LANGUAGES.md` está atualizado.
7. Executar os novos exemplos de PHP no CI em Linux arm64 e macOS.
## Testes
`bash tests/run-all.sh` (WSL), `bash tests/run-php.sh` (PHP de ponta a ponta), `cargo test --release`.
## Mensagem para retomar o trabalho
«Retome o F:\pl2pl. Comece pelo item 1 do PORTING.md. Nunca descarte em silêncio o que não puder ser convertido: deixe um TODO. Verifique executando de verdade antes de dar algo por concluído. Pergunte-me antes de fazer push.»
