# AGENTS.md — Memória Operacional do Projeto

> **Este arquivo é a memória persistente entre sessões de IA.**
> Leia-o inteiro no início de cada sessão antes de qualquer alteração.
> Ele deve ser atualizado ao final de cada tarefa relevante.
>
> **Última atualização:** 2026-10-24 (após a Fase 3)
> **Status atual:** Fases 0, 1, 2 e 3 concluídas. A TUI tem partidas 1P contra
> IA (Fácil/Médio/Impossível) e 2P, pausa, revanche, placar e layouts
> adaptativos. Próxima: Fase 4 (arte, som e i18n completo).
> **Branch:** `main`
> **Último commit:** `dd591d6 feat: conclui fase 3 com IA em três níveis e modo 1P`
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
o atraso de pensamento e `--seed`. Ainda não há arte final nem framebuffer.

## 2. Status atual

| Item | Estado |
| --- | --- |
| Fase 0 — Planejamento | Concluída |
| Fase 1 — Fundação | Concluída |
| Especificações | Criadas e aprovadas pelo usuário em escopo |
| Repositório Git | Inicializado em `main`; último commit `dd591d6` |
| Remote | Nenhum; não configurar sem pedido explícito |
| Código Rust | Presente; `cargo test` com 138 testes verdes |
| `Cargo.toml` / `Cargo.lock` | Criados; crate `velha-classica` 0.1.0 |
| Toolchain | rustup stable 1.99.1; MSRV declarada 1.88 |
| Dependências | `clap`, `crossterm`, `ratatui`, `thiserror` |
| CLI | `--help`, `--version`, `--lang`, `--backend` funcionam |
| Núcleo | `board`, `rules`, `score` e `state::game` com testes |
| TUI | Jogável em 1P e 2P; dificuldade inicia partida, IA pensa sem travar |
| Layout | Wide/Compact/Minimal; resize preserva o estado |
| Snapshots | 10 arquivos em `tests/snapshots/` (comparador próprio, sem `insta`) |
| Restauração | Guards de raw mode/alternate screen/cursor + panic hook |
| Assets PNG/fontes | Planejados, ainda não produzidos |
| Framebuffer | Planejado para v1.1; não iniciado |
| Bloqueios | Nenhum para a Fase 4 |
| Working tree | Alterações da Fase 3 ainda não commitadas |

### Próxima tarefa recomendada

**Iniciar a Fase 4 — Arte, som e i18n**, conforme
[docs/06-roadmap.md](docs/06-roadmap.md), somente após o usuário confirmar.
Não começar framebuffer antes da v1.0 estável.

Escopo principal da Fase 4 (ver roadmap):

- [ ] SVGs originais e pipeline `xtask` de geração de assets.
- [ ] PNGs otimizados, manifesto e checksums.
- [ ] Fontes OFL embutidas e licenças.
- [ ] Cena do tabuleiro e fallbacks de imagem.
- [ ] Áudio sintetizado e controle de mudo.
- [ ] Revisão completa de i18n e snapshots visuais.

Portões: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
`cargo test`, `cargo build --release`.

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
- **Fontes:** EB Garamond + Inter (OFL 1.1), embutidas apenas no framebuffer.
- **Licença proposta:** código MIT; arte CC BY 4.0 (confirmar antes da v1.0).

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

Em caso de conflito: primeiro este arquivo para **estado operacional**; depois
`docs/` para **especificação**. Se a especificação mudar, atualize o documento
correspondente e registre a decisão em `docs/07-decisions-and-risks.md`.

## 6. Pendências e questões abertas

Não bloqueiam a Fase 4, mas precisam ser resolvidas antes das fases indicadas:

| ID | Pendência | Prazo | Observação |
| --- | --- | --- | --- |
| Q-001 | Licença final (MIT + CC BY 4.0?) | Antes da v1.0 | Proposta atual |
| Q-002 | Aprovar EB Garamond + Inter | Antes da Fase 4 | Alternativa: outras OFL |
| Q-003 | Hardware do Ubuntu Server: `/dev/fb0`, resolução, bpp | Antes da Fase 6 | Validar cedo |
| Q-004 | Permissão de console/grupo `video` no servidor | Antes da Fase 6 | Necessário para fbdev |
| Q-005 | Volume padrão do áudio | Fase 4 | Sugestão: 0,32 |
| Q-006 | Manter a marca em inglês? | Fase 4 | Sugestão: sim |
| Q-007 | Persistência futura? | Pós-v1.1 | Fora do escopo atual |
| Q-008 | Atraso da IA Impossível? | Fase 3 | Resolvida: 450–800 ms |
| Q-009 | Mínimo de 80 × 24 é aceitável? | Fase 2 | Resolvida: sim; testado em 80 × 24 |
| Q-010 | Capturas de frame versionadas? | Fase 4 | Sugestão: só referências pequenas |

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

# Assets (Fase 4; ainda não existe)
cargo xtask assets
cargo xtask assets --check
```

Toolchain local: rustup em `~/.cargo` (stable 1.99.1, componentes `rustfmt` e
`clippy`). Em shell novo, carregar com `. "$HOME/.cargo/env"`.

Portões antes de qualquer release:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
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

## 11. Fases

| Fase | Estado | Próximo marco |
| --- | --- | --- |
| 0 — Planejamento | Concluída | Aprovação do usuário |
| 1 — Fundação | Concluída | Placeholder TUI + núcleo testado |
| 2 — TUI jogável | Concluída | Partida 2P completa |
| 3 — IA | Concluída | Impossível nunca perde |
| 4 — Arte/som/i18n | Próxima | Tier A/B/C + áudio + traduções |
| 5 — v1.0 desktop | Pendente | Release desktop |
| 6 — v1.1 framebuffer | Pendente | Console local do servidor |
| 7 — Backlog | Não iniciada | Itens pós-v1.1 |

## 12. Notas para a próxima sessão

- Na nova sessão, comece lendo este arquivo e confirmando o `git status`.
- As Fases 1, 2 e 3 estão concluídas; **não as reimplemente**. Não comece arte
  final, áudio nem framebuffer sem confirmação do usuário.
- A TUI cobre 1P (IA Fácil/Médio/Impossível) e 2P: Home, dificuldade, partida,
  pausa, fim de rodada, placar e revanche.
- A IA é função pura em `src/ai/mod.rs`; o atraso fica no `App` e não bloqueia
  o loop de eventos. `--seed` controla o RNG e os atrasos.
- A pausa cancela o turno pendente da IA e o reagenda ao continuar.
- Snapshots de TUI ficam em `tests/snapshots/`; regenere com
  `UPDATE_SNAPSHOTS=1 cargo test` e revise o diff antes de commitar.
- `--backend fb` retorna código 3 de propósito até a v1.1.
- Se o usuário pedir para commitar, verifique `git diff`/`git status`, faça um
  commit coerente e atualize o hash na seção 2 e no registro.
- Se houver dúvida sobre uma decisão, consulte `docs/07-decisions-and-risks.md`
  antes de perguntar; se ainda não estiver registrada, pergunte e registre.
