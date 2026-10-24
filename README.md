# Velha Clássica

> Jogo da velha para terminal Linux com estética de mesa de madeira e papel.
> **Status: planejamento (v0.1) — ainda não há código-fonte nesta etapa.**

Velha Clássica é uma versão própria de tic-tac-toe escrita em Rust. O jogo foi
pensado para ser executado localmente no Zorin OS, dentro de um terminal
colorido, e também no console local de um Ubuntu Server sem interface gráfica.
O layout ocupa aproximadamente 70% de uma tela 1920 × 1080 (1344 × 756 px) e é
adaptativo para janelas menores.

A experiência combina:

- partidas contra uma IA com três níveis ou entre dois jogadores no mesmo teclado;
- visual artesanal "madeira e papel", com PNGs originais gerados a partir de SVGs;
- animações discretas, som sintetizado e interface bilíngue (pt-BR/inglês);
- backend de terminal com `ratatui` e backend de framebuffer para o console local
  do servidor (previsto para a v1.1).

## Decisões principais

| Tema | Decisão |
| --- | --- |
| Linguagem | Rust stable, edição 2021 |
| Interface de terminal | `ratatui` + `crossterm` |
| Segundo backend | Framebuffer Linux (`/dev/fb0`), v1.1 |
| Modos de jogo | 1 jogador vs IA e 2 jogadores local |
| IA | Fácil, Médio e Impossível (minimax) |
| Estilo | Clássico madeira e papel, com tipografia serifada e detalhes em latão |
| Imagens | Assets originais SVG → PNG; fallback Unicode/half-blocks |
| Áudio | Efeitos sintetizados; desativação silenciosa sem dispositivo |
| Idiomas | UI em pt-BR por padrão; `F2` alterna para inglês |
| Placar | Sessão em memória; sem persistência entre execuções |
| Entrada | Somente teclado; mouse fora do escopo |
| Área de jogo | Alvo de 70% da tela; mínimo de 80 × 24 células no terminal |

## Plataformas-alvo

- **Desktop — Zorin OS:** terminal local, backend TUI. Recomenda-se Kitty ou
  WezTerm para PNGs em alta resolução; terminais sem protocolo de imagem usam
  half-blocks truecolor. GNOME Terminal funciona no modo de compatibilidade.
- **Servidor — Ubuntu Server:** console local com monitor e teclado, sem X/Wayland,
  usando backend de framebuffer a partir da v1.1. Não é necessário SSH.
- **Fora do escopo:** Windows, macOS, navegador, multiplayer em rede, mobile e
  execução remota como caso principal.

## Escopo das versões

### v1.0 — Desktop

- Home, seleção de dificuldade, partida, pausa e fim de rodada.
- 1 jogador contra IA e 2 jogadores local.
- IA fácil, média e impossível.
- Placar da sessão e revanche.
- Tema madeira/papel, animações, sons e UI bilíngue.
- PNGs via Kitty/Sixel/half-blocks, com fallback legado em 16 cores.
- Layout adaptativo com alvo de 70% em 1920 × 1080.

### v1.1 — Console local do servidor

- Backend de framebuffer Linux com a mesma identidade visual e a mesma lógica.
- Renderização por software, escala centralizada em 70% e textura de madeira.
- Entrada de teclado no TTY, restauração do console ao sair e fallback de áudio.
- Baseline em `/dev/fb0`; DRM/KMS fica como evolução posterior.

## Documentação

| Documento | Conteúdo |
| --- | --- |
| [01 — Game Design](docs/01-game-design.md) | Regras, modos, IA, fluxos, controles e critérios de aceite |
| [02 — Especificação Técnica](docs/02-technical-spec.md) | Arquitetura, crates, backends, framebuffer, build e desempenho |
| [03 — Design Visual](docs/03-visual-design.md) | Paleta, tipografia, layout 70%, wireframes e animações |
| [04 — Assets e Licenças](docs/04-assets-and-licenses.md) | Pipeline SVG→PNG, manifesto, fontes e licenciamento |
| [05 — Plano de Testes](docs/05-test-plan.md) | Testes unitários, snapshots, matriz manual e desempenho |
| [06 — Roadmap](docs/06-roadmap.md) | Fases, entregáveis, critérios de saída e backlog |
| [07 — Decisões e Riscos](docs/07-decisions-and-risks.md) | ADRs, premissas, questões abertas e riscos |
| [08 — Catálogo i18n](docs/08-i18n-catalog.md) | Todas as strings pt-BR/inglês e limites de layout |

## Estrutura planejada do projeto

```text
tic_tac_toe/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── docs/
│   └── *.md
├── assets/
│   ├── src/                 # SVGs originais
│   ├── generated/           # PNGs versionados
│   ├── fonts/               # fontes OFL embutidas
│   └── manifest.sha256
├── src/
│   ├── main.rs
│   ├── cli.rs
│   ├── app.rs
│   ├── core/                # board, rules, ai, score
│   ├── state/               # telas, transições, configurações
│   ├── i18n/
│   ├── render/              # TUI, framebuffer, cena, layout
│   ├── input.rs
│   ├── audio.rs
│   ├── assets.rs
│   ├── theme.rs
│   └── util/
├── tests/
└── xtask/                   # geração e verificação de assets
```

## Comandos previstos

> Ainda não executáveis nesta fase de planejamento.

```bash
# Desktop
cargo run --release

# Testes e qualidade
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check

# Servidor, sem áudio (v1.1)
cargo build --release --no-default-features
./target/release/velha-classica --backend fb

# Assets
cargo xtask assets
cargo xtask assets --check
```

## Licenciamento (proposta)

- Código: MIT.
- Arte original: CC BY 4.0, com crédito ao projeto.
- Fontes: OFL 1.1 (`EB Garamond` e `Inter`), com avisos preservados.
- Crates: licenças originais, listadas em `THIRD_PARTY_NOTICES.md` na implementação.

A licença final deve ser confirmada antes da v1.0; consulte as questões abertas em
[07 — Decisões e Riscos](docs/07-decisions-and-risks.md).

## Próximo passo

Revisar e aprovar as especificações desta pasta. Depois da aprovação, iniciar a
Fase 1 do [roadmap](docs/06-roadmap.md): esqueleto do projeto, núcleo de regras,
testes e primeira tela no terminal.
