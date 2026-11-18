> Traduzido por IA, ainda não revisado por falantes nativos. Vale o original em japonês (raiz do repositório).

# Passagem de bastão do pl2pl
## Situação (2026-11-19)
A v0.1.0 está publicada (instaladores para Windows/Linux/macOS, testes de ponta a ponta no CI). A conversão de PHP cobre rotas, corpos com várias instruções, Eloquent (SQLAlchemy), query builder com paginação, relações, Blade (Jinja2) e middleware (`Depends`), tudo testado de ponta a ponta. Numba, Kokkos (Serial) e `pl2pl web` estão confirmados com execuções reais.
## Pendências (por prioridade)
1. PHP: `with` aninhado, `morph`, `cursorPaginate`, componentes/pilhas/autorização do Blade, middleware posterior a `$next`.
2. Incorporar COBOL_to / Java_to ao pl2pl e depois apagar os originais (confirmar antes; irreversível; outras sessões trabalham neles).
3. Protótipo da IR comum.
4. Integração com o aruaru-llm (instalação real do `llm setup` não verificada).
5. Kokkos GPU / `Kokkos::View`; kernels em open-cuda / open-directx.
6. O proprietário escreve `docs/DESIGN.md`; conferir se `docs/LANGUAGES.md` está atualizado.
7. Executar os novos exemplos PHP no CI em Linux arm64 e macOS.
## Testes
`bash tests/run-all.sh` (WSL), `bash tests/run-php.sh` (PHP de ponta a ponta), `cargo test --release`.
## Mensagem para retomar
«Retomar F:\pl2pl. Começar pelo item 1 do PORTING.md. Nunca descartar em silêncio o que não puder ser convertido; deixar TODO. Só dar por concluído após executar de verdade. Confirmar antes de fazer push.»
