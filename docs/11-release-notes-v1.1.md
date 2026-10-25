# Notas de release — v1.1.0

**Data:** 2026-10-24  
**Versão:** 1.1.0  
**Tema:** console Linux via framebuffer (`/dev/fb0`)

## Destaques

- Novo backend `--backend fb`, com composição de 1344 × 756 escalada e
  centralizada sobre o fundo de madeira em qualquer resolução.
- Texto rasterizado com `fontdue` e as fontes EB Garamond/Inter já embutidas;
  a captura `--capture-frame` passa a incluir a camada textual.
- Suporte a 32 bpp (RGBX/BGRX/XRGB/XBGR), 24 bpp (RGB/BGR) e 16 bpp (RGB565).
- Console entra em `KD_GRAPHICS` e volta para `KD_TEXT` na saída, em pânico e nos
  sinais `SIGINT`/`SIGTERM`/`SIGHUP`.
- `--fb-device` permite apontar outro dispositivo (por exemplo `/dev/fb1`).
- Diagnóstico acionável para permissão do grupo `video`, ausência de VT local e
  formato de framebuffer não suportado.
- Mesmo jogo da v1.0: 1P/2P, IA em três níveis, pausa, placar, revanche,
  animações, áudio opcional e pt-BR/inglês.

## Implementação

- `src/platform/fb.rs`: abertura do dispositivo, `FBIOGET_*`/`FBIOPAN_DISPLAY`,
  `mmap`/`munmap` via `libc`, `KDGETMODE`/`KDSETMODE`, raw mode, guards RAII,
  handlers de sinal e restauração de pânico. `unsafe` restrito a esse módulo e
  documentado.
- `src/render/text.rs`: layout com quebra de linha, alinhamento, múltiplas
  fontes e cache de glifos.
- `src/render/fb.rs`: canvas lógico 1344 × 756, fundo pré-renderizado, escala
  bilinear com caminho rápido 1:1, painéis, menus, pausa e resultado.
- `src/app.rs`: loop do backend fb, pacing de 60 FPS em animação e 10 FPS ocioso.
- `src/capture.rs` + `App::preview`: frame determinístico de resultado com texto.
- `memmap2` não foi necessário; o mapeamento usa `libc` (ver D-026).

## Desempenho local (release, 1920 × 1080)

- Composição da cena de jogo: ~12 ms por frame.
- Escala/centralização 1:1: ~1,7 ms.
- 5 telas renderizadas em 0,21 s incluindo primeiro frame (cache frio).
- Meta de 60 FPS em animação e 30 FPS ocioso atendida ou próxima disso nesta
  máquina; medir de novo no console real.

## Qualidade

- 185 testes verdes (unitários, snapshots de TUI e novos testes de framebuffer).
- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `--no-default-features`, `cargo xtask assets --check` e builds release ok.
- Snapshots de TUI atualizados apenas para `v1.1.0`.

## Pacote

- Nome: `velha-classica-1.1.0-x86_64-unknown-linux-gnu.tar.gz`.
- Conteúdo: binário, README, licenças (MIT/CC BY 4.0/OFL), notas e créditos.
- O SHA-256 acompanha o pacote em `dist/*.sha256` e no checklist de release.

## Validação e pendências

- Em 2026-10-25 o backend foi validado no console local a 1920 × 1080, 32 bpp:
  partida 1P contra a IA, áudio e saída pelo menu com restauração do console.
- Pendentes (spot-checks): partida 2P, `Ctrl+C`, SIGTERM, troca de VT e formatos
  24/16 bpp. Ver `docs/12-v1.1-release-checklist.md`.
- Troca de VT durante o jogo pode deixar resíduos; documentado como limitação.
- Sem DRM/KMS; sem vsync garantido.

## Compatibilidade

- Requer Rust 1.88+ para compilar e Linux com fbdev para o backend fb.
- A TUI continua sendo o padrão (`--backend auto`).
- Assets, fontes e API pública do jogo permanecem compatíveis com a v1.0.
