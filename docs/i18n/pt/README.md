> Esta é uma tradução feita por IA, ainda não revisada por falantes nativos. A versão de referência é o original em japonês, na raiz do repositório.

# pl2pl

**O pl2pl transforma código escrito em uma linguagem ou framework em outra linguagem ou framework.**
Por exemplo, um site feito em PHP (Laravel) pode virar um site em Python (FastAPI). Funciona no Windows, no Linux e no Mac (v0.1.0, em desenvolvimento).

- A conversão é feita em um aplicativo que você instala no seu computador (uma ferramenta de linha de comando). O site só tem manuais e downloads.
- O que não puder ser convertido não é apagado em silêncio: o pl2pl deixa um `TODO` e explica o motivo em um relatório.
- O objetivo é converter entre as linguagens e frameworks mais conhecidos, no maior número possível de combinações.

## O que já funciona
| Comando | O que faz |
|---|---|
| `pl2pl php` | PHP (Laravel / Symfony) → FastAPI. Detalhes: [PHP.md](../../PHP.md) (em japonês) |
| `pl2pl web` | Examina uma pasta, reconhece sozinho Java, COBOL, PHP e Flask/Django, e envia cada parte ao conversor certo |
| `pl2pl cobol` | COBOL + JCL → GnuCOBOL (Linux) |
| `pl2pl omp` / `kokkos` / `gpu` | Reescreve laços de C/C++ para OpenMP, Kokkos ou GPU (gpu é um protótipo) |
| `pl2pl cport` | Faz o C de UNIX funcionar tanto no Windows quanto no Linux |
| `pl2pl pyhpc` | Acelera cálculos em Python com Numba |
| `pl2pl watch` | Encontra versões novas de linguagens e frameworks e se atualiza sozinho (veja abaixo) |
| `pl2pl llm setup` | Instala o assistente de IA (aruaru-llm) de acordo com o seu computador (a instalação real ainda não foi verificada) |

## Acompanhar as versões novas, automaticamente
Execute `pl2pl watch` toda noite e toda manhã.
1. Ele procura versões novas de linguagens e frameworks (a lista está em `registry/ecosystem.toml`).
2. Testa com a versão nova. Se os testes passarem, atualiza sozinho a «versão compatível».
3. Se falharem, o version-control (com ajuda de IA) tenta consertar.
4. Se ainda assim não resolver, avisa os e-mails e o LINE cadastrados.

Os scripts para agendar são `scripts/pl2pl-watch.cmd` (Windows) e `scripts/pl2pl-watch.sh` (Linux/Mac).
Atenção: para laravel, symfony, flask, django e outros, por enquanto ele só avisa que «saiu uma versão nova». Os testes reais com as versões novas ainda serão feitos.

## Exemplos
```
pl2pl php ./myapp --out ./out
pl2pl web ./project --target fastapi --out ./out
```

## Convém saber
- O pl2pl é cuidadoso. O que parece arriscado fica sem alteração, e o motivo é informado. A velocidade não é garantida.
- Só foi testado com exemplos pequenos, ainda não com programas grandes e reais.

## Próximos passos
- PHP: componentes Blade escritos como classes.
- Uma representação intermediária comum (IR) para acrescentar de uma vez muitos pares de linguagens.
- Verificar o envio real de e-mails e a correção automática por IA.
- Execução na GPU (open-cuda / open-directx).

## Saiba mais
[LANGUAGES.md](../../LANGUAGES.md) (lista de linguagens) / [COMBINATIONS.md](../../COMBINATIONS.md) (combinações) / [DESIGN.md](../../DESIGN.md) (projeto, escrito pelo proprietário) / [components/README.md](../../../components/README.md) (peças incluídas). Estes documentos estão em japonês.
