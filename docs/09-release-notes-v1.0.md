# Velha Clássica v1.0.0 — notas de release

**Data:** 2026-10-24  
**Plataforma:** Linux local (x86_64), terminal TUI  
**Licenças:** código MIT; arte original CC BY 4.0; fontes OFL 1.1

## Destaques

- Modo 1 jogador com IA Fácil, Médio e Impossível (minimax; nunca perde).
- Modo 2 jogadores no mesmo teclado, com revanche e placar de sessão.
- Tabuleiro em imagem original (SVG → PNG) via Kitty, Sixel, iTerm2 ou
  half-blocks, com fallback textual de 16 cores.
- Animações curtas de menu, cursor, marca, linha vencedora e selo.
- Áudio sintetizado em runtime, com mudo (`M`); silencioso se não houver
  dispositivo.
- Interface completa em pt-BR e inglês (`F2`).
- Assets e fontes embutidos: o binário funciona copiado, sem a pasta `assets/`.

## Instalação

A partir do código-fonte:

```bash
cargo build --release
install -m755 target/release/velha-classica ~/.local/bin/
velha-classica
```

A partir do pacote `velha-classica-1.0.0-x86_64-unknown-linux-gnu.tar.gz`:

```bash
tar xzf velha-classica-1.0.0-x86_64-unknown-linux-gnu.tar.gz
cd velha-classica-1.0.0-x86_64-unknown-linux-gnu
install -m755 velha-classica ~/.local/bin/
```

Requisitos: terminal local de no mínimo 80 × 24; Rust 1.88+ apenas para compilar.
Áudio usa `libasound.so.2` em runtime e não exige headers de desenvolvimento.

## Controles

| Contexto | Tecla | Ação |
| --- | --- | --- |
| Menus | `↑`/`↓`, `W`/`S` | Mover seleção |
| Menus | `Enter`, `Espaço` | Confirmar |
| Partida | Setas/WASD | Mover cursor |
| Partida | `Enter`, `Espaço` | Marcar |
| Partida | `Esc` | Pausar |
| Fim da rodada | `R` | Revanche |
| Global | `F2` | Idioma |
| Global | `M` | Som |
| Global | `Ctrl+C` | Sair |

## Compatibilidade de imagem

A detecção automática é conservadora e usa variáveis do terminal
(`KITTY_WINDOW_ID`, `TERM`, `TERM_PROGRAM`). Use `--image-protocol` para forçar
`kitty`, `sixel`, `iterm2` ou `halfblocks`, ou `--no-images` para o tabuleiro
textual. Imagens automáticas exigem truecolor.

## Desempenho medido

Medições locais em PTY (não substituem a matriz de hardware real):

- Startup até a Home: ~5,5 ms.
- RSS: ~18 MB com imagem; ~8 MB no fallback textual.
- CPU ocioso: 0,4%–0,8%.
- Binário release: ~3,3 MB.
- IA Impossível: abaixo de 5 ms no teste automatizado.

## Limitações conhecidas

- Sem backend de framebuffer; console local `/dev/fb0` fica para a v1.1.
- A captura `--capture-frame` cobre a composição de arte; o texto do canvas
  chega com o framebuffer.
- A matriz manual foi executada em Kitty 0.49.2, foot 1.16.2, WezTerm
  20240203, GNOME Terminal e áudio PipeWire/ALSA reais.
- Konsole não foi testado; o fallback half-blocks/textual deve ser usado.
- Em Kitty/Sixel, as animações reenviam a cena do tabuleiro; o desempenho foi
  aceitável na validação local.
- Sem persistência de idioma, placar ou configurações.

## Verificação

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test
cargo xtask assets --check
cargo build --release
```

O checksum SHA-256 do pacote acompanha o arquivo `.sha256` gerado no release.
