# Notas de release — v1.1.1

**Data:** 2026-10-28  
**Versão:** 1.1.1  
**Tema:** correção do double buffer no framebuffer e instalador

## Destaques

- Corrige a tela em branco em VTs cujo driver usa `yoffset`/double buffer e a
  aplicação desenhava no buffer não visível.
- O buffer de destino agora é escolhido a partir do `yoffset` atual, e não de um
  contador fixo iniciado em zero.
- Validação de `yoffset <= yres_virtual - yres`, `line_length`/stride e
  `smem_len` antes de mapear e desenhar.
- Double buffer só é habilitado quando `ypanstep > 0`,
  `yres_virtual >= 2 * yres`, `yres % ypanstep == 0` e a memória cobre os dois
  buffers.
- Se `FBIOPAN_DISPLAY` falhar, o pan é desabilitado e o frame é escrito
  diretamente no buffer visível, sem encerrar o jogo.
- Novo `install.sh`: extrai o pacote (ou usa o diretório já extraído), instala o
  binário e os documentos, e adiciona o usuário atual ao grupo `video`. O `sudo`
  pede a senha de forma interativa quando necessário.
- Nenhuma mudança nas regras, na IA, nos assets ou na interface do jogo.

## Instalação pelo script

Baixe o `.tar.gz`, o `.sha256` e o `install.sh` na mesma pasta e execute:

```bash
bash install.sh
```

Também funciona com o caminho explícito do pacote ou após extrair o `.tar.gz`:

```bash
bash install.sh velha-classica-1.1.1-x86_64-unknown-linux-gnu.tar.gz
# ou
tar xzf velha-classica-1.1.1-x86_64-unknown-linux-gnu.tar.gz
cd velha-classica-1.1.1-x86_64-unknown-linux-gnu
./install.sh
```

O destino padrão é `/usr/local`; use `--prefix` para outro local. O script
adiciona o usuário ao grupo `video` e informa que é necessário sair e entrar
novamente (ou usar `newgrp video`) para o acesso ao `/dev/fb0` valer.

## Pacote

- Nome: `velha-classica-1.1.1-x86_64-unknown-linux-gnu.tar.gz`.
- Conteúdo: binário, `install.sh`, README, licenças (MIT/CC BY 4.0/OFL), notas e
  créditos.
- O SHA-256 acompanha o pacote em `dist/*.sha256` e nos assets da release.

## Qualidade

- 192 testes verdes (unitários, snapshots de TUI e testes de framebuffer).
- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `--no-default-features`, `cargo xtask assets --check` e builds release ok.
- Novos testes cobrem escolha de buffer, validação de `yoffset`, stride,
  `smem_len`, double buffer e fallback para o buffer visível.

## Validação e pendências

- A correção foi validada por testes automatizados e revisão de código.
- Falta executar no segundo computador: confirmar a primeira tela sem `Ctrl+C`,
  registrar `fbset -i`/`strace -e ioctl` e repetir os spot-checks de 2P, sinais,
  troca de VT e formatos 24/16 bpp.
- Ver `docs/12-v1.1-release-checklist.md`.

## Compatibilidade

- Requer Linux com fbdev para o backend fb; a TUI continua padrão.
- Assets, fontes e API pública permanecem compatíveis com a v1.1.0.
