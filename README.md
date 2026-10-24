# Velha Clássica

Jogo da velha para terminal Linux com visual de mesa de madeira e papel.

> **Fase 3:** o modo **1 jogador** está completo, com IA Fácil, Médio e
> Impossível, e o modo 2 jogadores continua disponível no mesmo teclado.

## O que já funciona

- Partida local de 2 jogadores com tabuleiro navegável por teclado.
- Partida 1 jogador contra IA em três níveis: Fácil, Médio e Impossível.
- IA Impossível com minimax: nunca perde; empate já é resultado positivo.
- Home, seleção de dificuldade, pausa e fim de rodada.
- Detecção de vitória/empate, linha vencedora e placar da sessão.
- Revanche mantendo o placar; no 2P o iniciante alterna, no 1P o humano começa.
- Layouts Wide, Compact e Minimal, com adaptação ao redimensionar.
- Interface em pt-BR e inglês, alternada com `F2`.
- Núcleo de tabuleiro, regras, placar e estado de partida implementado e testado.
- CLI com ajuda, versão, idioma, backend e semente da IA (`--seed`).
- Restauração do terminal ao sair, inclusive com `Esc`, `Q` e `Ctrl+C`.

## Requisitos

- Linux com um terminal local (Zorin OS, Ubuntu e similares).
- Terminal de no mínimo **80 × 24** caracteres.
- Terminal com cores é recomendado; há fallback para 256 e 16 cores.
- Para compilar: **Rust 1.88 ou superior** (edição 2021).

## Instalar o Rust

Se ainda não tiver Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"
```

## Como rodar

Dentro da pasta do projeto:

```bash
# Desenvolvimento
cargo run

# Versão otimizada
cargo run --release

# Começar em inglês
cargo run -- --lang en

# Forçar o backend de terminal
cargo run -- --backend tui

# Reproduzir as escolhas da IA (útil para depuração)
cargo run -- --seed 42
```

Outras opções:

```bash
cargo run -- --help
cargo run -- --version
```

> `--backend fb` ainda retorna código de saída 3 de propósito: o backend de
> framebuffer só será implementado na v1.1.

Para desativar cores:

```bash
NO_COLOR=1 cargo run
```

## Controles desta versão

| Contexto | Tecla | Ação |
| --- | --- | --- |
| Menus | `↑`/`↓`, `W`/`S` | Mover seleção |
| Menus | `Enter`, `Espaço` | Confirmar |
| Menus | `Esc` | Voltar |
| Partida | `↑`/`↓`/`←`/`→`, `WASD` | Mover cursor pelo tabuleiro |
| Partida | `Enter`, `Espaço` | Marcar a casa selecionada |
| Partida | `Esc` | Abrir a pausa |
| Pausa | `↑`/`↓`, `Enter` | Escolher ação; `Esc` continua |
| Fim da rodada | `↑`/`↓`, `Enter` | Escolher ação; `R` inicia revanche |
| Global | `F2` | Alterna pt-BR/inglês |
| Global | `M` | Liga/desliga o indicador de som (o áudio ainda não existe) |
| Home | `Q` | Sair |
| Pausa | `Q` | Sair do jogo |
| Global | `Ctrl+C` | Sair e restaurar o terminal |

## Como testar

Rodar a suíte automatizada:

```bash
cargo test
```

Resultado esperado nesta versão: **138 testes passando**, incluindo snapshots de TUI.

Rodar os portões de qualidade:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo build --release
cargo build --release --no-default-features
```

Teste manual rápido:

1. Execute `cargo run` em um terminal local.
2. Escolha **1 Jogador**, selecione **Impossível** e confirme que a IA nunca
   perde; observe o aviso "IA pensando…" durante o turno dela.
3. Teste **Fácil** e **Médio** para comparar o comportamento.
4. Escolha **2 Jogadores**, mova o cursor com as setas ou `WASD` e marque casas
   com `Enter`; complete uma rodada e confira o placar e a revanche com `R`.
5. Abra a pausa com `Esc` durante o turno da IA e confirme que ela não joga até
   você continuar; teste também reiniciar e voltar ao menu.
6. Redimensione o terminal e confirme que a partida continua no mesmo estado.
7. Pressione `F2` e verifique os textos em inglês.
8. Pressione `Q` ou `Ctrl+C` e confirme que o cursor e a tela normal voltaram.

## Problemas comuns

**“stdin and stdout must be a local terminal”**
Execute o jogo em um terminal real, não via pipe, redirecionamento ou CI.

**“Terminal muito pequeno. Mínimo: 80×24.”**
Aumente a janela do terminal até pelo menos 80 colunas por 24 linhas.

**“the framebuffer backend is not available in this version”**
Esperado ao usar `--backend fb`. Use `--backend tui` ou deixe o padrão `auto`.

**As cores parecem estranhas**
O jogo detecta truecolor, 256 e 16 cores automaticamente. Para jogar sem cor,
use `NO_COLOR=1 cargo run`.

## Limitações desta versão

- Sem imagens finais, animações ou áudio.
- Sem backend de framebuffer.
- Sem persistência de idioma, placar ou configurações.
- A arte ainda usa a composição tipográfica da TUI, sem PNGs.

## Documentação do projeto

As especificações completas estão em [`docs/`](docs/):

- [Game design](docs/01-game-design.md)
- [Especificação técnica](docs/02-technical-spec.md)
- [Design visual](docs/03-visual-design.md)
- [Testes](docs/05-test-plan.md)
- [Roadmap](docs/06-roadmap.md)
- [Catálogo i18n](docs/08-i18n-catalog.md)

## Licença

Licença de código proposta: MIT. A licença final será confirmada antes da v1.0.
