# AGENTS.md — Memória Operacional do Projeto

> **Este arquivo é a memória persistente entre sessões de IA.**
> Leia-o inteiro no início de cada sessão antes de qualquer alteração.
> Ele deve ser atualizado ao final de cada tarefa relevante.
>
> **Última atualização:** 2026-10-25 (v1.1.0 validada no console e empacotada)
> **Status atual:** v1.1.0 implementada, validada manualmente no console local
> (1920 × 1080, 32 bpp: 1P, áudio e saída pelo menu) e empacotada em `dist/`.
> 185 testes e portões verdes. Restam spot-checks de 2P, `Ctrl+C`/SIGTERM,
> troca de VT e formatos 24/16 bpp.
> **Branch:** `main`
> **Último commit:** `bb6d680 docs: registra encerramento de sessão e estado final da v1.0.0`
> **Remote:** nenhum (repositório apenas local).

## 1. Retomada rápida

Se você é uma IA abrindo uma nova sessão, faça nesta ordem:

1. Leia este arquivo inteiro.
2. Execute `git status --short --branch` e `git log --oneline -5`.
3. Verifique se há alterações não commitadas; não sobrescreva trabalho existente.
4. Leia, conforme a tarefa:
   - [README.md](README.md) — visão geral e índice;
   - [docs/01-game-design.md](docs/01-game-design.md) — regras, IA, fluxos;
   - [docs/02-technical-spec.md](docs/02-technical-spec.md) — arquitetura;
   - [docs/03-visual-design.md](docs/03-visual-design.md) — layout e estética;
   - [docs/05-test-plan.md](docs/05-test-plan.md) — testes e critérios;
   - [docs/07-decisions-and-risks.md](docs/07-decisions-and-risks.md) — ADRs.
5. Consulte a seção **Status atual** e a **Próxima tarefa** abaixo.
6. Ao concluir qualquer tarefa, atualize este arquivo (seções 2, 9 e 10).

A Fase 1 criou o projeto Rust, o núcleo puro, o catálogo i18n inicial e o
placeholder TUI. A Fase 2 entregou a partida 2P jogável, menus, pausa, fim de
rodada, placar e snapshots. A Fase 3 entregou a IA em três níveis, o modo 1P,
o atraso de pensamento e `--seed`. A Fase 4 entregou assets originais, pipeline
`xtask`, cena em `tiny-skia`, `ratatui-image`, animações, áudio ALSA dinâmico,
fontes OFL e flags de acessibilidade. A Fase 5 fechou licenças, documentação,
medições, binário release e pacote `.tar.gz`. A Fase 6 implementou o backend
fbdev (`src/platform/fb.rs`), o renderer de framebuffer com texto (`fontdue`) e
a captura com texto; a validação manual no console segue pendente.

## 2. Status atual

| Item | Estado |
| --- | --- |
| Fase 0 — Planejamento | Concluída |
| Fase 1 — Fundação | Concluída |
| Especificações | Criadas e aprovadas pelo usuário em escopo |
| Repositório Git | Inicializado em `main`; último commit `878bc24` |
| Remote | Nenhum; não configurar sem pedido explícito |
| Código Rust | Presente; `cargo test` com 185 testes verdes |
| `Cargo.toml` / `Cargo.lock` | crate `velha-classica` 1.1.0; workspace com `xtask` |
| Toolchain | rustup stable 1.99.1; MSRV declarada 1.88 |
| Dependências | `clap`, `crossterm`, `ratatui`, `ratatui-image`, `image`, `tiny-skia`, `fontdue`, `libc`, `thiserror`, `libloading` (opcional) |
| CLI | idioma, backend, `--fb-device`, seed, cor, protocolo de imagem, flags e captura |
| Núcleo | `board`, `rules`, `score` e `state::game` com testes |
| TUI | 1P/2P; IA; imagem com fallback half-blocks/legado; animações |
| Layout | Wide/Compact/Minimal; resize preserva o estado |
| Snapshots | 10 arquivos em `tests/snapshots/` (comparador próprio, sem `insta`); versão v1.1.0 |
| Restauração | Guards de raw mode/alternate screen/cursor + `KD_TEXT`/mmap + panic hook e sinais |
| Assets | 16 SVGs, 16 PNGs e manifesto; `xtask assets --check` verde |
| Fontes | EB Garamond + Inter (OFL 1.1), estáticas e embutidas |
| Áudio | Sintetizado em runtime; ALSA via `libloading`; silencioso sem device |
| Captura | `--capture-frame` gera PNG 1344 × 756 com arte e texto (estado de resultado) |
| Release | `dist/velha-classica-1.1.0-x86_64-unknown-linux-gnu.tar.gz` + `.sha256` (v1.0.0 preservada em `dist/`) |
| SHA-256 do release | `cced01c99d20044dd773aa76fce830277484c5509e512343d761f922df5930cd` (v1.1.0) |
| Licenças | código MIT; arte CC BY 4.0; fontes OFL 1.1 |
| Framebuffer | Validado no console: 1920 × 1080, 32 bpp, 1P, áudio e saída pelo menu; 2P/Ctrl+C/SIGTERM/24/16 bpp pendentes |
| Bloqueios | Nenhum; grupo `video` e VT validados (Q-003/Q-004). Konsole não testado |
| Working tree | Alterações da Fase 6 e release v1.1.0 não commitadas (código, docs, snapshots, `dist/` e este arquivo) |

### Próxima tarefa recomendada

**Fechar os spot-checks manuais da v1.1.0 (opcionais, sem bloquear o pacote).**

Passos:

1. No VT, rodar `--backend fb` e jogar uma partida 2P até o fim.
2. Testar `Ctrl+C` e `kill -TERM $(pgrep velha-classica)`.
3. Se possível, testar troca de VT e formatos 24/16 bpp em outra máquina/VM.
4. Registrar o resultado em
   [docs/12-v1.1-release-checklist.md](docs/12-v1.1-release-checklist.md).
5. Commit apenas com pedido explícito do usuário.

Escopo da Fase 6 (ver [docs/06-roadmap.md](docs/06-roadmap.md)):

- [x] Validar `/dev/fb0` na máquina local: `amdgpudrmfb`, 1920 × 1080, 32 bpp
  (Q-003).
- [x] Confirmar permissão de console/grupo `video` (Q-004).
- [x] Abrir/mapear framebuffer e suportar 32/24/16 bpp.
- [x] `KDSETMODE KD_GRAPHICS` e restauração para `KD_TEXT`.
- [x] Entrada de teclado no TTY via crossterm (`/dev/tty`).
- [x] Composição 1344 × 756 com texto via `fontdue` e fontes embutidas.
- [x] `--backend fb`, `--fb-device` e captura com texto.
- [x] Guards de restauração e handlers de SIGINT/SIGTERM/SIGHUP + panic hook.
- [x] Testes manuais no console real (1P, áudio, saída); spot-checks pendentes.

Portões: `cargo fmt --check`,
`cargo clippy --workspace --all-targets -- -D warnings`, `cargo test`,
`cargo xtask assets --check`, `cargo build --release` e
`cargo build --release --no-default-features`.

## 3. Resumo do produto

- **Nome:** Velha Clássica.
- **Subtítulo:** Jogo da Velha · Mesa de Madeira / Tic-Tac-Toe · Wood & Paper.
- **Gênero:** jogo da velha clássico, local, para terminal Linux.
- **Tema:** madeira escura, papel envelhecido, tinta azul-petróleo/vinho e latão.
- **Modos:** 1 jogador vs IA e 2 jogadores no mesmo teclado.
- **IA:** Fácil, Médio e Impossível (minimax; nunca perde).
- **Idiomas:** pt-BR padrão; `F2` alterna para inglês.
- **Áudio:** efeitos sintetizados, opcionais; `M` alterna mudo.
- **Entrada:** somente teclado; mouse fora do escopo.
- **Placar:** sessão em memória; sem persistência.
- **Layout:** alvo de 70% de 1920 × 1080 = 1344 × 756 px; adaptativo no terminal.
- **Backends:** TUI (`ratatui` + `crossterm`) na v1.0; framebuffer Linux
  (`/dev/fb0`) na v1.1.
- **Imagens:** SVGs originais → PNGs embutidos; fallback half-blocks e legado.
- **Fontes:** EB Garamond + Inter (OFL 1.1), embutidas; usadas no framebuffer (v1.1).
- **Licença:** código MIT; arte original CC BY 4.0; fontes OFL 1.1.

## 4. Decisões congeladas (não reabrir sem o usuário)

- Linux local é a plataforma oficial; SSH é melhor esforço.
- Rust stable, edição 2021, `ratatui` + `crossterm`.
- Dois backends: TUI na v1.0; framebuffer fbdev na v1.1.
- v1.1 usa `/dev/fb0`; DRM/KMS fica no backlog.
- Tema clássico madeira/papel; nada de neon, CRT, glitch ou emojis.
- Modos 1P/2P; IA em três níveis.
- No 1P, humano é X e começa. No 2P, X começa a primeira; revanches alternam o
  iniciante mantendo os símbolos.
- Placar apenas em memória, zerado ao voltar ao menu/trocar modo/dificuldade.
- Interface 100% por teclado; sem mouse.
- Áudio sintetizado, sem arquivos de áudio, sem música, sem bloquear se faltar
  dispositivo.
- Sem tutorial dedicado na v1.0; controles no rodapé/pausa.
- Sem persistência de configurações ou histórico na v1.0.
- Assets originais SVG → PNG, com pipeline `xtask`; PNGs e fontes embutidos.
- Layout de 70% centralizado; fundo de madeira ocupa a tela inteira.
- `--backend auto` é conservador: prefere TUI; no servidor usar `--backend fb`.
- Fallback de imagem obrigatório; o jogo nunca depende de protocolo de imagem.
- IA Impossível usa minimax com desempate aleatório entre jogadas ótimas.
- RNG próprio SplitMix64 em `util/rng.rs`; `--seed` para reprodutibilidade.
- Áudio via ALSA dinâmico (`libasound.so.2` + `libloading`), não `rodio`.
- Detecção de imagem por ambiente + `--image-protocol`; sem consultar stdio.
- Fontes estáticas instanciadas das variáveis OFL; usadas no framebuffer.
- `--capture-frame` gera canvas 1344 × 756 com arte e texto.
- Framebuffer mapeia `/dev/fb0` com `mmap` de `libc`; `memmap2` dispensado (D-026).
- Renderer fb em duas camadas: fundo pré-renderizado + canvas escalado e
  centralizado com bilinear e caminho rápido 1:1 (D-027).
- `--backend auto` nunca escolhe framebuffer; fb é explícito (D-028).
- Texto do framebuffer via `fontdue` com cache de glifos.
- Nenhuma dependência de rede em runtime.

## 5. Documentos e fontes de verdade

| Documento | Autoridade |
| --- | --- |
| [README.md](README.md) | Guia do usuário: execução, controles, testes e limitações |
| [docs/01-game-design.md](docs/01-game-design.md) | Regras, IA, fluxos, controles, aceite funcional |
| [docs/02-technical-spec.md](docs/02-technical-spec.md) | Arquitetura, crates, backends, CLI, desempenho |
| [docs/03-visual-design.md](docs/03-visual-design.md) | Paleta, tipografia, layout, tier de imagem, animações |
| [docs/04-assets-and-licenses.md](docs/04-assets-and-licenses.md) | Pipeline de assets, manifesto, fontes, licenças |
| [docs/05-test-plan.md](docs/05-test-plan.md) | Testes automatizados, snapshots, matriz manual |
| [docs/06-roadmap.md](docs/06-roadmap.md) | Fases, entregáveis e critérios de saída |
| [docs/07-decisions-and-risks.md](docs/07-decisions-and-risks.md) | ADRs, premissas, questões abertas, riscos |
| [docs/08-i18n-catalog.md](docs/08-i18n-catalog.md) | Strings pt-BR/inglês e limites de layout |
| [docs/11-release-notes-v1.1.md](docs/11-release-notes-v1.1.md) | Notas da v1.1 |
| [docs/12-v1.1-release-checklist.md](docs/12-v1.1-release-checklist.md) | Checklist e matriz manual do framebuffer |

Em caso de conflito: primeiro este arquivo para **estado operacional**; depois
`docs/` para **especificação**. Se a especificação mudar, atualize o documento
correspondente e registre a decisão em `docs/07-decisions-and-risks.md`.

## 6. Pendências e questões abertas

Não bloqueiam a v1.0, mas precisam ser resolvidas antes das fases indicadas:

| ID | Pendência | Prazo | Observação |
| --- | --- | --- | --- |
| Q-001 | Licença final (MIT + CC BY 4.0?) | Antes da v1.0 | Resolvida: MIT + CC BY 4.0 |
| Q-002 | Aprovar EB Garamond + Inter | Fase 4 | Resolvida: aprovadas |
| Q-003 | Hardware do Ubuntu Server: `/dev/fb0`, resolução, bpp | Antes da Fase 6 | Resolvida: `amdgpudrmfb`, 1920 × 1080, 32 bpp |
| Q-004 | Permissão de console/grupo `video` no servidor | Antes da Fase 6 | Resolvida: grupo `video` e VT validados em 2026-10-25 |
| Q-005 | Volume padrão do áudio | Fase 4 | Resolvida: 0,32 |
| Q-006 | Manter a marca em inglês? | Fase 4 | Resolvida: sim |
| Q-007 | Persistência futura? | Pós-v1.1 | Fora do escopo atual |
| Q-008 | Atraso da IA Impossível? | Fase 3 | Resolvida: 450–800 ms |
| Q-009 | Mínimo de 80 × 24 é aceitável? | Fase 2 | Resolvida: sim; testado em 80 × 24 |
| Q-010 | Capturas de frame versionadas? | Fase 4 | Resolvida: gerar localmente; não versionar |

O usuário aprovou as especificações em escopo. As pendências acima são
operacionais/legais e não bloquearam as Fases 1, 2 e 3.

## 7. Comandos e qualidade

```bash
# Estado do repositório
git status --short --branch
git log --oneline -5

# Desenvolvimento
cargo run
cargo run -- --lang en
cargo test
UPDATE_SNAPSHOTS=1 cargo test   # regenera tests/snapshots/
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo build --release

# Servidor (v1.1)
cargo build --release --no-default-features
./target/release/velha-classica --backend fb
./target/release/velha-classica --backend fb --fb-device /dev/fb0

# Assets
cargo xtask assets
cargo xtask assets --check

# Captura de arte e texto para QA
cargo run -- --capture-frame captura.png
```

Toolchain local: rustup em `~/.cargo` (stable 1.99.1, componentes `rustfmt` e
`clippy`). Em shell novo, carregar com `. "$HOME/.cargo/env"`.

Portões antes de qualquer release:

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo clippy --workspace --all-targets --no-default-features -- -D warnings`
- `cargo test`
- `cargo xtask assets --check`
- `cargo build --release`
- `cargo build --release --no-default-features`
- Matriz manual de terminais e framebuffer.

## 8. Regras de trabalho para o agente

1. **Leia este arquivo antes de agir.** Não recomece o planejamento do zero.
2. **Não use a internet** para dependências de runtime; o jogo deve funcionar
   offline. Downloads de crates/fontes, se necessários, são decisão explícita.
3. **Não faça commit, push, amend ou PR sem pedido explícito** do usuário.
4. **Não configure remote** sem pedido explícito; o projeto é local.
5. **Não armazene segredos** neste arquivo nem no repositório.
6. **Não reabra decisões congeladas** sem o usuário; se necessário, proponha e
   registre um novo ADR.
7. **Mantenha o escopo da fase.** Não comece framebuffer antes da v1.0 estável;
   não adicione mouse, temas, persistência ou tutorial sem aprovação.
8. **Preserve o terminal/console.** Qualquer backend deve restaurar o estado em
   saída, erro, pânico e sinais.
9. **Código e identificadores em inglês; UI em pt-BR/inglês.**
10. **Sem comentários desnecessários** no código; siga o estilo do projeto.
11. **Teste antes de declarar concluído.** Use os portões da seção 7.
12. **Atualize este arquivo** após cada tarefa relevante, mesmo que parcial.
13. **Registre bloqueios** em vez de inventar progresso.
14. **Se o contexto for perdido**, reconstrua a partir deste arquivo + git +
    docs; não peça ao usuário para repetir decisões já registradas.

## 9. Protocolo de atualização deste arquivo

Ao final de cada tarefa, atualize:

1. **Última atualização** no topo (data e, se útil, hora).
2. **Status atual** (tabela da seção 2).
3. **Próxima tarefa recomendada** (substituir ou marcar itens).
4. **Registro de atividades** (seção 10), com data, resumo e arquivos alterados.
5. **Pendências** (seção 6), se alguma foi resolvida ou criada.
6. **Decisões** (seção 4 e doc 07), se algo mudou.

Regras de escrita:

- Seja conciso; não cole transcrições longas.
- Use checkboxes para progresso.
- Registre o hash do commit quando houver.
- Se uma tarefa falhou, registre o erro e o próximo passo.
- Não apague entradas antigas do registro; apenas acrescente.
- Se o arquivo crescer demais, arquive o histórico em `docs/` e mantenha aqui o
  resumo operacional.

## 10. Registro de atividades

| Data | Atividade | Resultado | Arquivos |
| --- | --- | --- | --- |
| 2026-10-24 | Perguntas de planejamento respondidas pelo usuário | Escopo, tema, IA, layout, idioma e backend definidos | — |
| 2026-10-24 | Especificações criadas | 8 documentos + README | `README.md`, `docs/*.md` |
| 2026-10-24 | Repositório Git inicializado | Branch `main`, commit inicial `335be01`, sem remote | `.gitignore`, docs |
| 2026-10-24 | Memória operacional criada | Este arquivo; ainda não commitado | `AGENTS.md` |
| 2026-10-24 | Fase 1 — Fundação implementada | Crate Rust, núcleo testado, i18n, tema/layout, guards, CLI e placeholder TUI; 67 testes e portões verdes | `Cargo.toml`, `Cargo.lock`, `src/**`, `README.md`, `docs/06-roadmap.md`, `docs/08-i18n-catalog.md` |
| 2026-10-24 | README reescrito para usuário final | Como instalar Rust, rodar, controlar, testar, limitações e diagnóstico | `README.md` |
| 2026-10-24 | Commit local da Fase 1 | `ea9ad52 feat: conclui fundação com núcleo testado e placeholder TUI`; 26 arquivos | `README.md`, `AGENTS.md`, `Cargo.toml`, `Cargo.lock`, `src/**`, `docs/06-roadmap.md`, `docs/08-i18n-catalog.md` |
| 2026-10-24 | Fase 2 — TUI jogável implementada | Home, dificuldade placeholder, partida 2P, pausa, fim de rodada, placar, revanche, layouts e 10 snapshots; 108 testes e portões verdes | `src/app.rs`, `src/input.rs`, `src/i18n/mod.rs`, `src/state/game.rs`, `src/render/tui.rs`, `tests/snapshots/**`, `README.md`, `docs/06-roadmap.md`, `docs/07-decisions-and-risks.md`, `docs/08-i18n-catalog.md`, `AGENTS.md` |
| 2026-10-24 | Commit local da Fase 2 | `9ea8530 feat: conclui fase 2 com partida 2P e TUI jogável`; 21 arquivos | `src/**`, `tests/snapshots/**`, `README.md`, `docs/**`, `AGENTS.md` |
| 2026-10-24 | Fase 3 — IA implementada | `src/ai` com Fácil/Médio/Impossível, RNG próprio, modo 1P, atraso 450–800 ms, `--seed`, pausa segura e 138 testes verdes | `src/ai/mod.rs`, `src/util/rng.rs`, `src/app.rs`, `src/cli.rs`, `src/main.rs`, `src/i18n/mod.rs`, `src/render/tui.rs`, `README.md`, `docs/01-game-design.md`, `docs/06-roadmap.md`, `docs/08-i18n-catalog.md`, `AGENTS.md` |
| 2026-10-24 | Commit local da Fase 3 | `dd591d6 feat: conclui fase 3 com IA em três níveis e modo 1P`; 15 arquivos | `src/ai/**`, `src/util/rng.rs`, `src/**`, `README.md`, `docs/**`, `AGENTS.md` |
| 2026-10-24 | Smoke 1P em PTY | Home → dificuldade → partida → IA pensando → pausa/retomada → saída limpa com restauração do terminal | — |
| 2026-10-24 | Fase 4 — arte, som e i18n | SVGs + pipeline `xtask`, PNGs/manifesto, fontes OFL, cena `tiny-skia`, `ratatui-image`, animações, áudio ALSA dinâmico, flags e captura; 164 testes verdes | `assets/**`, `xtask/**`, `src/assets.rs`, `src/audio.rs`, `src/capture.rs`, `src/render/{scene,image}.rs`, `src/app.rs`, `src/cli.rs`, `README.md`, `docs/**`, `THIRD_PARTY_NOTICES.md`, `AGENTS.md` |
| 2026-10-24 | Commit local da Fase 4 | `d973584 feat: conclui fase 4 com arte, áudio e acessibilidade`; 64 arquivos | `assets/**`, `xtask/**`, `src/**`, `README.md`, `docs/**`, `THIRD_PARTY_NOTICES.md`, `AGENTS.md` |
| 2026-10-24 | Fase 5 — v1.0 desktop | Licenças MIT/CC BY 4.0/OFL, versão 1.0.0, README final, notas e checklist, medições, pacote `tar.gz` + `.sha256` e smoke sem assets; 164 testes verdes | `LICENSE`, `LICENSE-ASSETS`, `Cargo.toml`, `Cargo.lock`, `README.md`, `docs/06-roadmap.md`, `docs/07-decisions-and-risks.md`, `docs/09-release-notes-v1.0.md`, `docs/10-v1.0-release-checklist.md`, `tests/snapshots/**`, `AGENTS.md` |
| 2026-10-24 | Commit local da Fase 5 | `9a369c1 feat: conclui fase 5 e empacota v1.0.0`; 23 arquivos | `LICENSE`, `LICENSE-ASSETS`, `Cargo.toml`, `Cargo.lock`, `README.md`, `docs/**`, `src/render/image.rs`, `tests/snapshots/**`, `AGENTS.md` |
| 2026-10-24 | Validação manual v1.0.0 | Kitty 0.49.2, foot 1.16.2, WezTerm 20240203, GNOME Terminal e áudio PipeWire/ALSA; 80 × 24, Wide, pausa, selo, F2/M e áudio OK. Correções: overlay de pausa, contraste de menu, posição do selo e indicador de som | `src/render/tui.rs`, `src/render/scene.rs`, `tests/snapshots/**`, `README.md`, `docs/06-roadmap.md`, `docs/09-release-notes-v1.0.md`, `docs/10-v1.0-release-checklist.md`, `AGENTS.md` |
| 2026-10-24 | Commit da validação v1.0.0 | `5da4af0 fix: valida v1.0 e corrige detalhes nos terminais reais`; 10 arquivos | `README.md`, `AGENTS.md`, `docs/**`, `src/render/{tui,scene}.rs`, `tests/snapshots/**` |
| 2026-10-24 | Registro do hash da validação | `878bc24 docs: registra hash do commit da validação v1.0.0` | `AGENTS.md` |
| 2026-10-24 | Encerramento de sessão | AGENTS.md atualizado com estado final, release `f3b3e9d…`, terminais instalados e próximo passo (Fase 6); arquivo deixado não commitado por não haver pedido explícito de commit | `AGENTS.md` |
| 2026-10-24 | Fase 6 — backend framebuffer | `src/platform/fb.rs` (device, ioctls, mmap via `libc`, console, sinais), `src/render/{text,fb}.rs`, `--fb-device`, captura com texto, versão 1.1.0; 185 testes e portões verdes | `Cargo.toml`, `Cargo.lock`, `src/platform/**`, `src/render/{fb,text}.rs`, `src/{app,capture,cli,lib,main,assets}.rs`, `tests/snapshots/**`, `README.md`, `docs/{02,05,06,07}.md`, `docs/11-release-notes-v1.1.md`, `docs/12-v1.1-release-checklist.md`, `AGENTS.md` |
| 2026-10-24 | Validação de `/dev/fb0` | `amdgpudrmfb`, 1920 × 1080, 32 bpp; leitura/escrita bloqueadas para `daniel` (fora do grupo `video`); sessão Wayland, teste exige VT | — |
| 2026-10-24 | Capturas de QA do fb | Revisadas Home, dificuldade, jogo, pausa e resultado em pt-BR/en; correção do card de pausa; captura `--capture-frame` agora inclui texto | `src/render/fb.rs`, `src/capture.rs`, `src/app.rs` |
| 2026-10-25 | Validação manual v1.1.0 no VT | 1920 × 1080, 32 bpp: partida 1P contra a IA, áudio no console e saída pelo menu com restauração; console sem corrupção | `docs/12-v1.1-release-checklist.md`, `AGENTS.md` |
| 2026-10-25 | Empacotamento v1.1.0 | `dist/velha-classica-1.1.0-...tar.gz` + `.sha256`; smoke em `/tmp` sem assets (`--version` e PNG 1344 × 756) | `dist/velha-classica-1.1.0-*`, `docs/06-roadmap.md`, `docs/11-release-notes-v1.1.md`, `AGENTS.md` |

## 11. Fases

| Fase | Estado | Próximo marco |
| --- | --- | --- |
| 0 — Planejamento | Concluída | Aprovação do usuário |
| 1 — Fundação | Concluída | Placeholder TUI + núcleo testado |
| 2 — TUI jogável | Concluída | Partida 2P completa |
| 3 — IA | Concluída | Impossível nunca perde |
| 4 — Arte/som/i18n | Concluída | Tier A/B/C + áudio + traduções |
| 5 — v1.0 desktop | Concluída | Release validado |
| 6 — v1.1 framebuffer | Concluída (spot-checks opcionais) | Release v1.1.0 empacotado |
| 7 — Backlog | Não iniciada | Itens pós-v1.1 |

## 12. Notas para a próxima sessão

- Na nova sessão, comece lendo este arquivo e confirmando o `git status`.
- As Fases 1 a 6 estão implementadas; **não as reimplemente**. A v1.0.0 foi
  validada manualmente em Kitty, foot, WezTerm, GNOME Terminal e áudio real.
- Backend fb: `src/platform/fb.rs` (device/console/sinais), `src/render/fb.rs`
  (telas e composição), `src/render/text.rs` (fontdue). `unsafe` só no módulo de
  plataforma, com comentário.
- Entrada do fb: crossterm lê `/dev/tty` automaticamente; não depende de stdin.
- `--backend auto` continua TUI; use `--backend fb` no VT.
- Validação feita em 2026-10-25 no VT: 1P, áudio e saída pelo menu OK. Faltam
  spot-checks de 2P, `Ctrl+C`, SIGTERM, troca de VT e 24/16 bpp (ver docs/12).
- Desempenho local (release, 1080p): cena ~12 ms/frame, escala 1:1 ~1,7 ms;
  frame de animação a 16 ms. Remeça no console antes de concluir.
- A TUI cobre 1P (IA Fácil/Médio/Impossível) e 2P, com imagem e fallbacks.
- A IA é função pura em `src/ai/mod.rs`; o atraso fica no `App` e não bloqueia
  o loop. `--seed` controla o RNG e os atrasos.
- A pausa cancela o turno pendente da IA e o reagenda ao continuar.
- Assets: `cargo xtask assets`; `--check` é portão. Não edite PNG à mão.
- Imagem: detecção por ambiente + `--image-protocol`; não consultar stdio.
- Áudio: ALSA dinâmico; sem device é silêncio. `--no-default-features` remove.
- A captura `--capture-frame` usa `App::preview` e inclui texto (resultado 2P).
- Release v1.1.0: `dist/velha-classica-1.1.0-...tar.gz`, SHA-256
  `cced01c99d20044dd773aa76fce830277484c5509e512343d761f922df5930cd`.
  A v1.0.0 permanece em `dist/` (SHA-256
  `f3b3e9d75e98d41f703ac20cb90f6953b32d0a0bd423ea9629486a019ca34caa`).
- Terminais usados na validação manual, instalados em `~/.local`:
  `~/.local/kitty.app` (`kitty` 0.49.2), `~/.local/foot.app` (`foot` 1.16.2) e
  `~/.local/wezterm.app` (`wezterm` 20240203). GNOME Terminal também validado.
- Snapshots de TUI ficam em `tests/snapshots/`; regenere com
  `UPDATE_SNAPSHOTS=1 cargo test` e revise o diff antes de commitar.
- Ao retomar, rode `git status`; Fase 6 + release v1.1.0 estão não commitados.
  Se o usuário pedir, faça um commit coerente e atualize o hash na seção 2.
- Se houver dúvida sobre uma decisão, consulte `docs/07-decisions-and-risks.md`
  antes de perguntar; se ainda não estiver registrada, pergunte e registre.
