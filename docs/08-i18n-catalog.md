# 08 — Catálogo i18n

## 1. Regras

- Idioma padrão: **pt-BR**.
- `F2` alterna para inglês e volta; a troca é imediata e não altera o estado do
  jogo.
- `--lang pt|en` define o idioma inicial da sessão.
- Nenhuma string de UI deve ficar fora deste catálogo.
- A escolha não é persistida entre execuções.
- Placeholders usam `{nome}` e são formatados por métodos tipados, nunca por
  substituição solta de texto.
- O catálogo é implementado como uma estrutura com campos obrigatórios; a
  ausência de uma tradução deve falhar em tempo de compilação ou em teste.
- Textos de CLI (`--help`) ficam em inglês na v1.0 para manter o `clap` simples;
  a UI do jogo é bilíngue.
- Erros fatais podem exibir a mensagem localizada seguida de um detalhe técnico
  curto em inglês.

## 2. Convenção de chaves

Formato: `área.elemento[.estado]`.

Exemplos:

- `menu.one_player`
- `difficulty.impossible.desc`
- `game.turn.ai`
- `result.win.you`
- `footer.language`
- `error.fb_permission`

## 3. Limites de layout

| Campo | Limite sugerido (uma linha) | Observação |
| --- | --- | --- |
| Item de menu | 18 caracteres | Pode ficar em uma linha no Wide |
| Título de tela | 28 caracteres | Cabeçalho |
| Descrição de dificuldade | 52 caracteres | Pode quebrar em 2 linhas |
| Status principal | 34 caracteres | Uma linha no Compact |
| Ação de rodapé | 26 caracteres | Pode abreviar no Minimal |
| Botão de resultado | 22 caracteres | Painel lateral |
| Erro | 80 caracteres | Pode quebrar em 2–3 linhas |
| Rótulo de placar | 14 caracteres | Coluna numérica separada |
| Selo | 12 caracteres | Caixa alta |

Os limites são guias; o layout deve quebrar linha ou reduzir corpo antes de
truncar. Testar todas as strings nos breakpoints Wide, Compact e Minimal.

## 4. Catálogo

### 4.1 Aplicação

| Chave | pt-BR | English | Limite | Notas |
| --- | --- | --- | --- | --- |
| `app.title` | Velha Clássica | Velha Clássica | 16 | Marca; não traduzir |
| `app.subtitle` | Jogo da Velha | Tic-Tac-Toe | 16 | Subtítulo |
| `app.tagline` | Mesa de Madeira | Wood & Paper | 18 | Assinatura visual |
| `app.version` | v{version} | v{version} | 12 | Ex.: v1.0.0 |

### 4.2 Menu principal

| Chave | pt-BR | English | Limite | Notas |
| --- | --- | --- | --- | --- |
| `menu.one_player` | 1 Jogador | 1 Player | 12 | |
| `menu.two_players` | 2 Jogadores | 2 Players | 14 | |
| `menu.quit` | Sair | Quit | 8 | |
| `menu.choose_difficulty` | Escolha a dificuldade | Choose difficulty | 24 | Título da tela |
| `menu.back` | Voltar | Back | 8 | |

### 4.3 Dificuldade

| Chave | pt-BR | English | Limite | Notas |
| --- | --- | --- | --- | --- |
| `difficulty.easy` | Fácil | Easy | 10 | |
| `difficulty.easy.desc` | Joga sem muita estratégia. | Plays without much strategy. | 36 | |
| `difficulty.medium` | Médio | Medium | 10 | |
| `difficulty.medium.desc` | Aproveita erros e bloqueia ameaças. | Punishes mistakes and blocks threats. | 42 | |
| `difficulty.impossible` | Impossível | Impossible | 12 | |
| `difficulty.impossible.desc` | Nunca perde. Empate já é vitória. | Never loses. A draw is a win. | 38 | |
| `difficulty.human_starts` | Você começa com X. | You start as X. | 22 | Apoio na tela de dificuldade |
| `difficulty.placeholder` | A IA chega na Fase 3. Jogue em 2 Jogadores. | AI arrives in Phase 3. Play with 2 Players. | 48 | Temporário até a Fase 3 |
| `difficulty.footer` | Enter iniciar · Esc voltar | Enter start · Esc back | 30 | Pode ser montado com chaves de rodapé |

### 4.4 Partida

| Chave | pt-BR | English | Limite | Notas |
| --- | --- | --- | --- | --- |
| `game.turn.you` | Sua vez | Your turn | 16 | Humano |
| `game.turn.ai` | Vez da IA | AI's turn | 16 | Modo 1P |
| `game.turn.player1` | Vez do Jogador 1 | Player 1's turn | 20 | Modo 2P |
| `game.turn.player2` | Vez do Jogador 2 | Player 2's turn | 20 | Modo 2P |
| `game.you` | Você | You | 8 | Rótulo de placar |
| `game.ai` | IA | AI | 4 | Rótulo de placar |
| `game.player1` | Jogador 1 | Player 1 | 12 | Rótulo de placar |
| `game.player2` | Jogador 2 | Player 2 | 12 | Rótulo de placar |
| `game.draws` | Empates | Draws | 10 | Rótulo de placar |
| `game.session_score` | Placar da sessão | Session score | 18 | Cabeçalho do placar |
| `game.you_play_x` | Você joga com X | You play as X | 18 | Apoio no 1P |
| `game.occupied` | Casa ocupada | Square occupied | 18 | Feedback de erro |
| `game.mode.one` | 1 Jogador | 1 Player | 12 | Cabeçalho |
| `game.mode.two` | 2 Jogadores | 2 Players | 14 | Cabeçalho |
| `game.difficulty` | Dificuldade: {difficulty} | Difficulty: {difficulty} | 30 | Cabeçalho no 1P |
| `game.round` | Rodada {n} | Round {n} | 14 | Opcional no cabeçalho |
| `game.mark.x` | X | X | 1 | Não traduzir |
| `game.mark.o` | O | O | 1 | Não traduzir |
| `game.ai_thinking` | IA pensando | AI thinking | 16 | Animação de pontos é separada |

### 4.5 Resultado da rodada

| Chave | pt-BR | English | Limite | Notas |
| --- | --- | --- | --- | --- |
| `result.win.you` | Você venceu! | You win! | 16 | 1P |
| `result.win.ai` | A IA venceu! | The AI wins! | 18 | 1P |
| `result.win.player1` | Jogador 1 venceu! | Player 1 wins! | 22 | 2P |
| `result.win.player2` | Jogador 2 venceu! | Player 2 wins! | 22 | 2P |
| `result.draw` | Empate! | Draw! | 10 | |
| `result.stamp.win` | VITÓRIA | WIN | 10 | Selo em caixa alta |
| `result.stamp.draw` | EMPATE | DRAW | 10 | Selo em caixa alta |

### 4.6 Ações

| Chave | pt-BR | English | Limite | Notas |
| --- | --- | --- | --- | --- |
| `action.resume` | Continuar | Resume | 12 | Pausa |
| `action.restart` | Reiniciar partida | Restart round | 20 | Pausa |
| `action.rematch` | Revanche | Rematch | 12 | Fim de rodada |
| `action.main_menu` | Menu principal | Main menu | 16 | |
| `action.quit_game` | Sair do jogo | Quit game | 16 | |
| `action.back` | Voltar | Back | 8 | |

### 4.7 Pausa

| Chave | pt-BR | English | Limite | Notas |
| --- | --- | --- | --- | --- |
| `pause.title` | Pausado | Paused | 10 | Título do overlay |

### 4.8 Rodapé e atalhos

| Chave | pt-BR | English | Limite | Notas |
| --- | --- | --- | --- | --- |
| `footer.move` | Setas/WASD mover | Arrows/WASD move | 20 | Partida |
| `footer.select_square` | Enter marcar | Enter mark | 16 | Partida |
| `footer.select_menu` | Enter selecionar | Enter select | 18 | Menus |
| `footer.navigate` | Setas/WASD navegar | Arrows/WASD navigate | 22 | Menus |
| `footer.pause` | Esc pausar | Esc pause | 12 | Partida |
| `footer.back` | Esc voltar | Esc back | 12 | Menus |
| `footer.rematch` | R revanche | R rematch | 12 | Fim de rodada |
| `footer.menu` | Esc menu | Esc menu | 12 | Fim de rodada |
| `footer.language` | F2 idioma | F2 language | 14 | Global |
| `footer.sound` | M som | M sound | 10 | Global |
| `footer.quit` | Q sair | Q quit | 10 | Home |

### 4.9 Status e indicadores

| Chave | pt-BR | English | Limite | Notas |
| --- | --- | --- | --- | --- |
| `status.language` | Idioma: {language} | Language: {language} | 22 | Cabeçalho |
| `language.pt_br` | Português | Portuguese | 12 | Nome do idioma |
| `language.en` | Inglês | English | 10 | Nome do idioma |
| `status.sound_on` | Som ligado | Sound on | 12 | Indicador |
| `status.sound_off` | Som desligado | Sound off | 14 | Indicador |
| `status.sound_unavailable` | Som indisponível | Sound unavailable | 20 | Sem dispositivo |
| `status.thinking` | IA pensando{dots} | AI thinking{dots} | 20 | `{dots}` é animado |
| `status.muted` | Sem som | Muted | 10 | Alternativo |

### 4.10 Erros e avisos

| Chave | pt-BR | English | Limite | Notas |
| --- | --- | --- | --- | --- |
| `error.terminal_too_small` | Terminal muito pequeno. Mínimo: {cols}×{rows}. | Terminal too small. Minimum: {cols}×{rows}. | 60 | Aviso central |
| `error.terminal_resize` | Aumente a janela e tente novamente. | Resize the window and try again. | 44 | Apoio |
| `error.not_tty` | Execute em um terminal local. | Run in a local terminal. | 36 | Antes da TUI |
| `error.fb_missing` | Framebuffer não encontrado em {path}. | Framebuffer not found at {path}. | 60 | Servidor |
| `error.fb_permission` | Sem permissão para acessar {path}. Adicione seu usuário ao grupo video. | No permission to access {path}. Add your user to the video group. | 86 | Pode quebrar |
| `error.fb_format` | Formato de framebuffer não suportado: {format}. | Unsupported framebuffer format: {format}. | 60 | |
| `error.fb_graphics_mode` | Não foi possível ativar o modo gráfico do console. | Could not enable console graphics mode. | 58 | |
| `error.assets` | Falha ao carregar recursos visuais. | Failed to load visual assets. | 40 | Fatal |
| `error.audio` | Áudio indisponível; o jogo continuará sem som. | Audio unavailable; the game will continue without sound. | 64 | Não fatal |
| `error.backend` | Backend indisponível: {backend}. | Backend unavailable: {backend}. | 44 | |
| `error.unexpected` | Erro inesperado. Consulte o log. | Unexpected error. Check the log. | 40 | |
| `error.invalid_cli` | Opção inválida: {option}. | Invalid option: {option}. | 48 | CLI |
| `error.capture_frame` | Não foi possível gravar o frame em {path}. | Could not write frame to {path}. | 58 | QA |

### 4.11 Placeholder da Fase 1 (removido)

As chaves `placeholder.title` e `placeholder.body` foram removidas ao concluir a
Fase 2. A tela de dificuldade usa `difficulty.placeholder` até a Fase 3.

## 5. Placeholders

| Placeholder | Tipo | Usado em |
| --- | --- | --- |
| `{version}` | string semântica | `app.version` |
| `{language}` | nome localizado | `status.language` |
| `{difficulty}` | nome localizado | `game.difficulty` |
| `{n}` | inteiro | `game.round` |
| `{dots}` | string animada (`""`, `.`, `..`, `...`) | `status.thinking` |
| `{cols}` | inteiro | `error.terminal_too_small` |
| `{rows}` | inteiro | `error.terminal_too_small` |
| `{path}` | caminho | erros de framebuffer/captura |
| `{format}` | descrição do formato | `error.fb_format` |
| `{backend}` | `tui`/`fb` | `error.backend` |
| `{option}` | argumento | `error.invalid_cli` |

Regras:

- Formatação por métodos, por exemplo:
  - `turn_text(Player::You)` → "Sua vez" / "Your turn";
  - `win_text(Player::Ai)` → "A IA venceu!" / "The AI wins!";
  - `difficulty_text(Difficulty::Impossible)` → "Impossível" / "Impossible".
- Não interpolar strings traduzidas dentro de outras sem revisão gramatical.
- Se um placeholder faltar, o teste de i18n falha.
- Não usar pluralização dinâmica na v1.0; os rótulos de placar são fixos e os
  números ficam em coluna separada.

## 6. Notas de tradução

- "Velha Clássica" é marca e permanece igual.
- "Jogo da Velha" (pt-BR) ↔ "Tic-Tac-Toe" (en).
- "IA" ↔ "AI".
- "Você" ↔ "You"; em frases, preferir "Sua vez" a "Vez de Você".
- "Empate" ↔ "Draw"; "Empates" ↔ "Draws".
- "Revanche" ↔ "Rematch".
- "Partida" pode significar uma rodada; usar "Reiniciar partida" para a rodada
  atual, não para a sessão.
- "Menu principal" ↔ "Main menu".
- "Som ligado/desligado" ↔ "Sound on/off".
- "Impossível" descreve o nível; não usar "Imbatível" para não prometer algo
  além do que o minimax garante.
- Manter o tom calmo, curto e sem exclamações excessivas; "Você venceu!" e
  "Empate!" são as exceções de celebração.

## 7. Testes de i18n

- Todas as chaves existem nos dois idiomas e não estão vazias.
- Nenhuma string de UI fora do catálogo.
- Placeholders válidos e usados.
- Comprimentos dentro dos limites ou com quebra de linha aprovada.
- Acentuação portuguesa renderiza nas fontes embutidas e no terminal.
- `F2` atualiza Home, Dificuldade, Partida, Pausa e Resultado imediatamente.
- O idioma não altera o estado da partida nem o placar.
- Textos em inglês não estouram o painel lateral no layout Wide.
- Textos em português não estouram o layout Compact.
- Selos de vitória/empate correspondem ao idioma ativo.
- Erros de framebuffer exibem a mensagem localizada e o detalhe técnico.

## 8. Comportamento de fallback

- Idioma inicial inválido → pt-BR.
- Tradução ausente → falha de teste/build; em runtime de desenvolvimento, usar
  pt-BR e registrar log.
- Fonte sem glifo → o framebuffer deve falhar de forma clara no startup; a TUI
  usa a fonte do terminal e deve recomendar uma fonte com suporte a latim.
- Texto muito longo → quebrar linha; se ainda não couber, reduzir corpo até o
  mínimo legível e, por último, usar reticências.
