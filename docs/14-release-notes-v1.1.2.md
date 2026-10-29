# Notas de release — v1.1.2

**Data:** 2026-10-28  
**Versão:** 1.1.2  
**Tema:** desbloqueio do framebuffer no início do backend fb

## Causa raiz confirmada no segundo computador

O `testes_debug.txt` mostrou:

- `i915drmfb`, 1920 × 1080, 32 bpp, `yres_virtual == yres` (sem ypan).
- `/sys/class/graphics/fb0/blank: 4` (powerdown/DPMS desligado) antes do app.
- O app abria `/dev/fb0`, mapeava a memória, entrava em `KD_GRAPHICS` e
  desenhava, mas nunca chamava `FBIOBLANK`/`FB_BLANK_UNBLANK`.
- O teste direto com `FBIOBLANK` mudou o estado para `blank: 0`.
- O `Ctrl+C` gera entrada de teclado, que acorda o console e revela a imagem
  por instantes.

Ou seja: não era `yoffset`, permissão, Wayland nem instalção. O console estava
blanked/DPMS off e o app não o desbloqueava.

## Correção

- `src/platform/fb.rs` agora chama `FBIOBLANK` com `FB_BLANK_UNBLANK` logo após
  entrar em `KD_GRAPHICS`. Falha no ioctl é ignorada (drivers sem blanking
  continuam funcionando).
- A ordem continua: abrir/validar fb, mapear, entrar em `KD_GRAPHICS`,
  desbloquear o blank, raw mode, primeiro frame, loop, restaurar console.

## Diagnóstico

O asset `testes_debug.sh` foi mantido e melhorado:

- Executa o app real antes e depois do teste de desbloqueio.
- Mede `blank/DPMS`, `fbset`, vtconsole, DRM e ioctls via `strace`.
- Faz escrita direta com faixas RGB, testa `FBIOPAN_DISPLAY` e restaura
  `KD_TEXT`.
- Gera `testes_debug.txt` no diretório atual.

## Instalação

```bash
bash install.sh
```

O script instala em `/usr/local`, adiciona o usuário ao grupo `video` e pede a
senha do `sudo` de forma interativa quando necessário.

## Pacote

- Nome: `velha-classica-1.1.2-x86_64-unknown-linux-gnu.tar.gz`.
- Conteúdo: binário, `install.sh`, `testes_debug.sh`, README, licenças
  (MIT/CC BY 4.0/OFL), notas e créditos.
- Assets separados: pacote, `.sha256`, `install.sh` e `testes_debug.sh`.

## Qualidade

- 192 testes verdes.
- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `--no-default-features`, `cargo xtask assets --check` e builds release ok.

## Validação pendente

- Confirmar no segundo computador que a Home aparece imediatamente sem `Ctrl+C`.
- Repetir os spot-checks de 2P, `Ctrl+C`, SIGTERM, troca de VT e 24/16 bpp.
