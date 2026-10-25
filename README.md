# Velha Clássica

Jogo da velha para terminal Linux com visual de mesa de madeira e papel.

> **Versão 1.0.0:** primeira versão desktop para terminal Linux, com 1P/2P, IA
> em três níveis, arte original em imagem, animações, áudio sintetizado e
> interface pt-BR/inglês.

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
- Tabuleiro em imagem com fallback half-blocks e legado de 16 cores.
- Animações de cursor, marca, linha vencedora, selo e menus (desligáveis).
- Áudio sintetizado com mudo; silencioso se não houver dispositivo.
- Captura determinística de arte para QA com `--capture-frame`.
- CLI com ajuda, versão, idioma, backend, semente (`--seed`), protocolo de
  imagem, cor e flags de acessibilidade.
- Restauração do terminal ao sair, inclusive com `Esc`, `Q` e `Ctrl+C`.

## Requisitos

- Linux com um terminal local (Zorin OS, Ubuntu e similares).
- Terminal de no mínimo **80 × 24** caracteres.
- Terminal com cores é recomendado; há fallback para 256 e 16 cores.
- Áudio usa `libasound.so.2` em runtime; não é preciso instalar headers de
  desenvolvimento. Sem dispositivo, o jogo segue em silêncio.
- Para compilar: **Rust 1.88 ou superior** (edição 2021).

## Instalação

A partir do pacote release (quando disponível):

```bash
tar xzf velha-classica-1.0.0-x86_64-unknown-linux-gnu.tar.gz
cd velha-classica-1.0.0-x86_64-unknown-linux-gnu
install -m755 velha-classica ~/.local/bin/
```

Ou compile e instale a partir do código:

```bash
cargo build --release
install -m755 target/release/velha-classica ~/.local/bin/
```

O binário é autossuficiente: assets e fontes são embutidos e ele funciona copiado
para outro diretório, sem a pasta `assets/`.

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

# Acessibilidade e compatibilidade
cargo run -- --no-images --no-animations
cargo run -- --no-audio
cargo run -- --color 256
cargo run -- --image-protocol halfblocks
```

Gerar uma captura de arte determinística para QA:

```bash
cargo run -- --capture-frame captura.png
```

Outras opções:

```bash
cargo run -- --help
cargo run -- --version
cargo run -- --backend fb
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
| Global | `M` | Liga/desliga o som |
| Home | `Q` | Sair |
| Pausa | `Q` | Sair do jogo |
| Global | `Ctrl+C` | Sair e restaurar o terminal |

## Como testar

Rodar a suíte automatizada:

```bash
cargo test
```

Resultado esperado nesta versão: **164 testes passando**, incluindo snapshots de TUI.

Rodar os portões de qualidade:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test
cargo xtask assets --check
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
7. Pressione `F2` e verifique os textos em inglês; pressione `M` e confirme o
   mudo no cabeçalho/rodapé.
8. Pressione `Q` ou `Ctrl+C` e confirme que o cursor e a tela normal voltaram.
9. Rode `cargo xtask assets --check` e confirme que os assets estão atualizados.

## Problemas comuns

**“stdin and stdout must be a local terminal”**
Execute o jogo em um terminal real, não via pipe, redirecionamento ou CI.

**“Terminal muito pequeno. Mínimo: 80×24.”**
Aumente a janela do terminal até pelo menos 80 colunas por 24 linhas.

**“the framebuffer backend is not available in this version”**
Esperado ao usar `--backend fb`. Use `--backend tui` ou deixe o padrão `auto`.

**As cores parecem estranhas**
O jogo detecta truecolor, 256 e 16 cores automaticamente. Force com
`--color truecolor`, `--color 256`, `--color 16` ou `--color never`.

**O tabuleiro aparece em blocos ou sem imagem**
A detecção automática usa variáveis do terminal. Imagens automáticas exigem
truecolor; force o protocolo com `--image-protocol kitty`, `sixel`, `iterm2` ou
`halfblocks`, ou use `--no-images` para o tabuleiro textual.

**Não há som**
O áudio é opcional e silencioso quando não há dispositivo ou `libasound.so.2`.
`M` alterna o mudo.

## Assets, fontes e licenças

- SVGs originais em `assets/src/`; PNGs versionados em `assets/generated/`.
- Regeneração: `cargo xtask assets`; verificação: `cargo xtask assets --check`.
- Fontes EB Garamond e Inter (OFL 1.1) em `assets/fonts/`, com as licenças.
- Créditos de terceiros em [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
- Proposta de licença da arte: CC BY 4.0 (confirmar antes da v1.0).

## Limitações desta versão

- Sem backend de framebuffer (v1.1).
- A detecção de protocolo de imagem é conservadora; use `--image-protocol` se
  necessário.
- A captura de frame cobre a composição de arte; o texto do canvas chega com o
  framebuffer.
- Sem persistência de idioma, placar ou configurações.

## Documentação do projeto

As especificações completas estão em [`docs/`](docs/):

- [Game design](docs/01-game-design.md)
- [Especificação técnica](docs/02-technical-spec.md)
- [Design visual](docs/03-visual-design.md)
- [Testes](docs/05-test-plan.md)
- [Roadmap](docs/06-roadmap.md)
- [Catálogo i18n](docs/08-i18n-catalog.md)
- [Notas de release v1.0.0](docs/09-release-notes-v1.0.md)
- [Checklist de release](docs/10-v1.0-release-checklist.md)

## Licença

- Código: [MIT](LICENSE).
- Arte original (SVGs, PNGs e identidade visual): [CC BY 4.0](LICENSE-ASSETS),
  crédito "Velha Clássica — arte original".
- Fontes: SIL Open Font License 1.1; textos em `assets/fonts/OFL-*.txt`.
- Créditos de terceiros: [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
