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

A partir do pacote release gerado em `dist/`:

```bash
cd dist
sha256sum -c velha-classica-1.0.0-x86_64-unknown-linux-gnu.tar.gz.sha256
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

## Como jogar

Compile uma vez e use o binário. A escolha de 1P/2P e da dificuldade é feita
dentro do jogo.

```bash
cargo build --release
BIN=./target/release/velha-classica
```

### 1 jogador

```bash
# Abra o jogo e escolha: 1 Jogador -> Fácil, Médio ou Impossível
$BIN

# Inglês e semente fixa (útil para comparar partidas)
$BIN --lang en --seed 42
```

No modo 1P você é **X** e começa. A IA é **O**; no Impossível ela usa minimax e
nunca perde. Durante o turno dela aparece “IA pensando…”.

### 2 jogadores

```bash
# Abra o jogo e escolha: 2 Jogadores
$BIN
```

X começa a primeira rodada; as revanches alternam o iniciante. O placar é da
sessão e fica em memória.

## Modos de exibição

### Por terminal

| Terminal | Comando | Protocolo esperado |
| --- | --- | --- |
| Kitty | `kitty ./target/release/velha-classica` | Kitty (Tier A) |
| foot | `foot ./target/release/velha-classica` | Sixel (Tier A) |
| WezTerm | `wezterm start --cwd "$PWD" -- ./target/release/velha-classica` | iTerm2 (Tier A) |
| GNOME Terminal | `gnome-terminal --working-directory="$PWD" -- ./target/release/velha-classica` | half-blocks (Tier B) |
| Terminal 16 cores | `$BIN --color 16` | textual (Tier C) |

A detecção automática usa `KITTY_WINDOW_ID`, `TERM` e `TERM_PROGRAM`. Se o
terminal não for reconhecido, force o protocolo:

```bash
$BIN --image-protocol kitty
$BIN --image-protocol sixel
$BIN --image-protocol iterm2
$BIN --image-protocol halfblocks
$BIN --no-images              # tabuleiro textual
$BIN --image-protocol none    # equivalente a --no-images
```

Imagens automáticas exigem truecolor. Em terminal de 256 ou 16 cores o jogo cai
para o tabuleiro textual automaticamente.

### Cores, animações e áudio

```bash
$BIN --color truecolor   # força truecolor
$BIN --color 256         # 256 cores
$BIN --color 16          # 16 cores
$BIN --color never       # sem cor
NO_COLOR=1 $BIN          # respeita a variável de ambiente

$BIN --no-animations     # reduz movimento e o atraso da IA para 250 ms
$BIN --no-audio          # desliga o áudio; M também alterna o mudo
```

### Captura para QA

```bash
$BIN --capture-frame captura.png
```

Gera um PNG determinístico 1344 × 756 com a composição de arte (madeira,
tabuleiro, painel, selo e vinheta), sem abrir terminal interativo, áudio ou
framebuffer. É útil para revisar arte e regressões visuais.

### Ajuda e versão

```bash
$BIN --help
$BIN --version
```

> `--backend fb` retorna código de saída 3 de propósito: o backend de
> framebuffer só será implementado na v1.1. Use `--backend tui` ou o padrão
> `auto`.

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

### Suite automatizada

```bash
cargo test
cargo test --no-default-features
cargo test --release
```

Resultado esperado nesta versão: **164 testes passando** em cada configuração,
incluindo snapshots de TUI.

### Portões de qualidade

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo clippy --workspace --all-targets --no-default-features -- -D warnings
cargo xtask assets --check
cargo build --release
cargo build --release --no-default-features
```

Assets podem ser regenerados a partir dos SVGs com:

```bash
cargo xtask assets
cargo xtask assets --check
```

### Checklist manual

Execute em um terminal local, de preferência com pelo menos 80 × 24.

1. **1P Fácil**: jogue algumas partidas; confirme que a IA é vencível.
2. **1P Médio**: confirme bloqueios e que ela ainda pode ser vencida com garfos.
3. **1P Impossível**: jogue várias partidas; confirme “IA pensando…” e que a IA
   nunca perde (empate é o melhor resultado).
4. **2P**: complete uma vitória e um empate; confira placar, linha vencedora,
   selo e revanche (`R`), observando a alternância do iniciante.
5. **Pausa**: abra com `Esc` durante o turno da IA; ela não deve jogar até você
   continuar. Teste reiniciar rodada e voltar ao menu.
6. **Imagens**: abra em Kitty, foot, WezTerm e GNOME Terminal; confirme o
   tabuleiro em imagem/half-blocks e a legibilidade em 80 × 24.
7. **Resize**: altere entre Wide, Compact e Minimal sem perder o estado.
8. **Idioma/som**: `F2` alterna pt-BR/inglês; `M` alterna mudo e o indicador
   aparece no cabeçalho/rodapé.
9. **Áudio**: ouça navegação, X/O, jogada inválida, vitória e empate; confirme
   volume baixo e ausência de clipping.
10. **Fallbacks**: teste `--no-images`, `--color 256`, `--color 16`,
    `--no-animations` e `--no-audio`.
11. **Reprodutibilidade**: rode duas vezes com `--seed 42` e compare as jogadas
    da IA.
12. **Captura**: rode `--capture-frame captura.png` e abra o PNG gerado.
13. **Sem assets**: copie o binário para outro diretório e execute; ele deve
    funcionar sem a pasta `assets/`.
14. **Saída**: `Q`/`Ctrl+C` (e fechar a janela) não deve deixar o terminal em
    raw mode ou alternate screen.

### Validado nesta release

A matriz manual de v1.0.0 foi executada em Kitty, foot, WezTerm e GNOME Terminal,
incluindo 80 × 24, Wide, pausa, resultado/selo, `F2`, `M` e áudio real via
PipeWire/ALSA. Konsole não foi testado; o fallback half-blocks deve ser usado.

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
- Licença da arte: CC BY 4.0 (texto completo em `LICENSE-ASSETS`).

## Limitações desta versão

- Sem backend de framebuffer (v1.1).
- A detecção de protocolo de imagem é conservadora; use `--image-protocol` se
  necessário.
- A captura de frame cobre a composição de arte; o texto do canvas chega com o
  framebuffer.
- Konsole não foi validado nesta release; use half-blocks/textual se o protocolo
  automático não funcionar.
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
