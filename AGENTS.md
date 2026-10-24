# AGENTS.md — Memória Operacional do Projeto

> **Este arquivo é a memória persistente entre sessões de IA.**
> Leia-o inteiro no início de cada sessão antes de qualquer alteração.
> Ele deve ser atualizado ao final de cada tarefa relevante.
>
> **Última atualização:** 2026-10-24 (após README do usuário)
> **Status atual:** Fases 0 e 1 concluídas. Projeto Rust compilando, núcleo
> testado e placeholder TUI funcional. Próxima: Fase 2 (TUI jogável).
> **Branch:** `main`
> **Último commit:** `335be01 chore: inicializa projeto com especificações do jogo`
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
placeholder TUI. Ainda não há partida jogável, arte final nem framebuffer.

## 2. Status atual

| Item | Estado |
| --- | --- |
| Fase 0 — Planejamento | Concluída |
| Fase 1 — Fundação | Concluída |
| Especificações | Criadas e aprovadas pelo usuário em escopo |
| Repositório Git | Inicializado em `main`, commit `335be01` |
| Remote | Nenhum; não configurar sem pedido explícito |
| Código Rust | Presente; `cargo test` com 67 testes verdes |
| `Cargo.toml` / `Cargo.lock` | Criados; crate `velha-classica` 0.1.0 |
| Toolchain | rustup stable 1.99.1; MSRV declarada 1.88 |
| Dependências | `clap`, `crossterm`, `ratatui`, `thiserror` |
| CLI | `--help`, `--version`, `--lang`, `--backend` funcionam |
| Núcleo | `board`, `rules`, `score` e `state::game` com testes |
| TUI | Placeholder Wide/Compact/Minimal; `F2` alterna pt-BR/inglês |
| Restauração | Guards de raw mode/alternate screen/cursor + panic hook |
| Assets PNG/fontes | Planejados, ainda não produzidos |
| Framebuffer | Planejado para v1.1; não iniciado |
| Bloqueios | Nenhum para a Fase 2 |
| Working tree | Código e docs da Fase 1 não commitados; `AGENTS.md` não rastreado |

### Próxima tarefa recomendada

**Iniciar a Fase 2 — TUI jogável**, conforme
[docs/06-roadmap.md](docs/06-roadmap.md), sem começar arte final nem backend
framebuffer.

Escopo principal da Fase 2:

- [ ] Home, dificuldade (placeholder), partida 2P, pausa e fim de rodada.
- [ ] Tabuleiro navegável por teclado e marcação de casas.
- [ ] Revanche com alternância de iniciante no 2P.
- [ ] Placar da sessão e feedback de jogada inválida.
- [ ] Layouts Wide/Compact/Minimal usando o `layout.rs` existente.
- [ ] Testes de fluxo e snapshots iniciais com `TestBackend`.
- [ ] `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
      `cargo test` verdes.

Critérios de saída da Fase 2 estão em
[docs/06-roadmap.md](docs/06-roadmap.md#5-fase-2--tui-jogável).

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

Não bloqueiam a Fase 1, mas precisam ser resolvidas antes das fases indicadas:

| ID | Pendência | Prazo | Observação |
| --- | --- | --- | --- |
| Q-001 | Licença final (MIT + CC BY 4.0?) | Antes da v1.0 | Proposta atual |
| Q-002 | Aprovar EB Garamond + Inter | Antes da Fase 4 | Alternativa: outras OFL |
| Q-003 | Hardware do Ubuntu Server: `/dev/fb0`, resolução, bpp | Antes da Fase 6 | Validar cedo |
| Q-004 | Permissão de console/grupo `video` no servidor | Antes da Fase 6 | Necessário para fbdev |
| Q-005 | Volume padrão do áudio | Fase 4 | Sugestão: 0,32 |
| Q-006 | Manter a marca em inglês? | Fase 4 | Sugestão: sim |
| Q-007 | Persistência futura? | Pós-v1.1 | Fora do escopo atual |
| Q-008 | Atraso da IA Impossível? | Fase 3 | Sugestão: 450–800 ms |
| Q-009 | Mínimo de 80 × 24 é aceitável? | Fase 2 | Sugestão: sim |
| Q-010 | Capturas de frame versionadas? | Fase 4 | Sugestão: só referências pequenas |

O usuário aprovou as especificações em escopo. As pendências acima são
operacionais/legais e não bloquearam a Fase 1.

## 7. Comandos e qualidade

```bash
# Estado do repositório
git status --short --branch
git log --oneline -5

# Desenvolvimento
cargo run
cargo run -- --lang en
cargo test
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

## 11. Fases

| Fase | Estado | Próximo marco |
| --- | --- | --- |
| 0 — Planejamento | Concluída | Aprovação do usuário |
| 1 — Fundação | Concluída | Placeholder TUI + núcleo testado |
| 2 — TUI jogável | Próxima | Partida 2P completa |
| 3 — IA | Pendente | Impossível nunca perde |
| 4 — Arte/som/i18n | Pendente | Tier A/B/C + áudio + traduções |
| 5 — v1.0 desktop | Pendente | Release desktop |
| 6 — v1.1 framebuffer | Pendente | Console local do servidor |
| 7 — Backlog | Não iniciada | Itens pós-v1.1 |

## 12. Notas para a próxima sessão

- Na nova sessão, comece lendo este arquivo e confirmando o `git status`.
- A Fase 1 está concluída; **não a reimplemente** e não comece arte final,
  áudio nem framebuffer. O próximo passo é a Fase 2 (TUI jogável).
- A TUI atual é apenas um placeholder: `Q`/`Esc`/`Ctrl+C` saem, `F2` alterna o
  idioma e `M` alterna o som.
- `--backend fb` retorna código 3 de propósito até a v1.1.
- Se o usuário pedir para commitar, verifique `git diff`/`git status`, faça um
  commit coerente e atualize o hash na seção 2 e no registro.
- Se houver dúvida sobre uma decisão, consulte `docs/07-decisions-and-risks.md`
  antes de perguntar; se ainda não estiver registrada, pergunte e registre.
