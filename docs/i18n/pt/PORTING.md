> Esta é uma tradução feita por IA, ainda não revisada por falantes nativos. A versão de referência é o original em japonês, na raiz do repositório.

# pl2pl – passagem de bastão
(As notas detalhadas anteriores estão em `docs/HISTORY-ja.md`, em japonês.)

## Situação atual (2026-11-19)
- A versão 0.1.0 está publicada, com instaladores para Windows, Linux e Mac e testes na CI.
- PHP → FastAPI está testado de ponta a ponta com FastAPI real (49 verificações): rotas, Eloquent, construtor de consultas, Blade, middleware, relações.
- O `pl2pl web` agora também reconhece PHP e o encaminha.
- Novidade: o `pl2pl watch` encontra versões novas, testa, conserta e, se nada der certo, avisa. O version-control ganhou a opção `--notify-file`.
- COBOL_to e Java_to estão incorporados em `converters/`.

## O que falta (por prioridade)
1. **Verificar**: a chegada de um e-mail real (`version-control --test-notify`), a correção automática via `version-control --loop` e um teste com o fastapi mais recente (a execução noturna da CI vai conferir).
2. Experimentar o `watch` com versões novas reais (laravel, symfony, spring-boot, flask, django e poem ainda contam como «não verificados»).
3. Registrar as execuções da noite e da manhã (`schtasks /Create` etc.; ainda não feito, pois é a configuração pessoal do proprietário).
4. Protótipo da representação intermediária comum (IR), base para acrescentar muitos pares de linguagens.
5. PHP: `@can`, componentes Blade como classes, `$attributes`, `@php`, `morphToMany`.
6. Integração com o aruaru-llm (a instalação real com `llm setup` não foi verificada). Kokkos na GPU; kernels no open-cuda / open-directx.
7. Apagar os COBOL_to e Java_to originais (**sem volta: pedir antes a confirmação final**).
8. O proprietário escreve `docs/DESIGN.md`. Conferir se `docs/LANGUAGES.md` não está desatualizado.

## Testes
`bash tests/run-all.sh` (WSL) / `bash tests/run-php.sh` (PHP de ponta a ponta) / `cargo test --release`

## Mensagem para retomar
«Retomar F:\pl2pl. Seguir «O que falta» em PORTING.md de cima para baixo. Nunca apagar em silêncio o que não puder ser convertido: deixar um TODO. Verificar executando de verdade antes de dizer que terminou. Responder somente em japonês. Fazer push a cada etapa.»
