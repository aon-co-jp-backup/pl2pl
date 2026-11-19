> Nota: o PORTING.md em japonês na raiz do repositório é o original. Esta versão é uma tradução feita por IA, em palavras simples, e ainda não foi revisada por falantes nativos.

# pl2pl – passagem de bastão
(As notas detalhadas anteriores estão em `docs/HISTORY-ja.md`, em japonês.)

## Onde estamos (2026-11-19)
- A versão 0.1.0 está publicada, com instaladores para Windows, Linux e Mac e testes na CI.
- PHP → FastAPI está testado de ponta a ponta com FastAPI real: rotas, Eloquent, construtor de consultas, Blade, middleware, relações e a família `@can`.
- O `pl2pl web` agora também reconhece PHP.
- O `pl2pl watch` encontra versões novas, testa, conserta problemas e envia um alerta se nada der certo. O GitHub também testa com as versões mais recentes toda noite e toda manhã.
- O `pl2pl ir` (versão de teste) coloca as rotas de PHP, Python e Java (Spring, JAX-RS, Servlet, Spark) numa forma comum e gera JSON, OpenAPI ou um esqueleto de FastAPI. Conferimos que o Python realmente consegue ler a saída.
- A página de apresentação e download está no ar: <https://easy-web.tokyo/pl2pl/> (no VPS, estática).
- COBOL_to e Java_to estão incorporados em `converters/`.

## O que falta (o mais importante primeiro)
1. **Verificar**: a chegada de um e-mail real (`version-control --test-notify`; este PC não tem configuração de SMTP), a correção automática via version-control `--loop` e um teste com o fastapi mais recente (a execução noturna no GitHub vai conferir).
2. Colocar tipos e conteúdo das funções na IR. Fazer a saída atual do FastAPI (`pl2pl php`) passar pela IR. Se a busca de rotas deixar coisas passarem, pensar em tree-sitter.
3. Experimentar o `watch` com versões novas reais (laravel, symfony, spring-boot, flask, django e poem ainda contam como não verificados).
4. Programar num PC as execuções da noite e da manhã (ainda não feito: o comando de verificação não funciona neste PC e daria alarmes falsos; veja um exemplo no início de `scripts/pl2pl-watch.cmd`).
5. Adicionar um cartão do pl2pl na página inicial do easy-web.tokyo.
6. PHP: peças do Blade escritas como classes, `$attributes`, `@php`, `morphToMany`.
7. Integração com o aruaru-llm (a instalação real com `llm setup` não foi verificada). Kokkos na GPU; kernels no open-cuda / open-directx.
8. Apagar os COBOL_to e Java_to originais (**sem volta: pedir antes a aprovação final**).
9. O proprietário escreve `docs/DESIGN.md`. Conferir se `docs/LANGUAGES.md` não está desatualizado.

## Testes
`bash tests/run-all.sh` (WSL) / `bash tests/run-php.sh` (PHP de ponta a ponta) / `cargo test --release`

## Mensagem para retomar
«Retomar F:\pl2pl. Seguir «O que falta» em PORTING.md de cima para baixo. Nunca esconder o que não puder ser reescrito: deixar um TODO. Verificar executando de verdade antes de dizer que terminou. Responder somente em japonês. Fazer commit e push a cada etapa e atualizar também o VPS.»
