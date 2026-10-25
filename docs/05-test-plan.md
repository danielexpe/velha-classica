# 05 — Plano de Testes

## 1. Estratégia

O projeto usa cinco camadas de verificação:

1. **Testes unitários** do núcleo puro (tabuleiro, regras, IA, placar, layout,
   i18n, tema).
2. **Testes de integração** da máquina de estados e da CLI.
3. **Snapshots de TUI** com `ratatui::backend::TestBackend`.
4. **Captura de frames** determinísticos do renderer compartilhado/framebuffer
   para QA visual.
5. **Matriz manual** de terminais, console local, áudio, idiomas e tamanhos.

Testes automatizados não substituem a matriz manual de TTY/framebuffer, porque
protocolos de imagem, permissões e modos de console não são reproduzíveis em CI
sem hardware.

## 2. Ferramentas

- `cargo test` — unitários e integração.
- Comparador próprio de snapshots de buffer da TUI (`tests/snapshots/`).
- Propriedades determinísticas com RNG semeado (sem `proptest`).
- `cargo clippy --workspace --all-targets -- -D warnings`.
- `cargo fmt --check`.
- `cargo xtask assets --check`.
- `--capture-frame` — PNG determinístico do canvas 1344 × 756 com arte e texto.
- `--seed` — reprodutibilidade da IA.
- `hyperfine`, `/usr/bin/time -v`, `top`/`htop` — desempenho manual.
- Comparação perceptual de PNGs para capturas visuais (ferramenta a definir na
  implementação; não usar diff byte a byte entre plataformas).

## 3. Testes unitários do núcleo

### 3.1 Tabuleiro

- `new_board_is_empty`
- `place_accepts_empty_cell`
- `place_rejects_occupied_cell`
- `place_rejects_out_of_bounds`
- `available_cells_lists_only_empty`
- `board_index_row_col_roundtrip`
- `full_board_has_no_available_cells`

### 3.2 Regras

- `detects_all_eight_winning_lines` — parametrizado para cada linha.
- `does_not_report_win_with_two_marks`
- `does_not_report_win_for_mixed_marks`
- `detects_draw_on_full_board_without_winner`
- `winning_line_contains_winning_mark`
- `outcome_is_in_progress_on_empty_board`
- `placing_after_game_over_is_rejected`
- `turn_alternates_after_valid_move`
- `invalid_move_does_not_change_turn`

### 3.3 Placar

- `win_increments_correct_counter_once`
- `draw_increments_draws_only`
- `rematch_preserves_score`
- `leaving_to_menu_resets_score`
- `changing_difficulty_resets_score`
- `double_finish_does_not_double_count`

### 3.4 Iniciante e revanche

- `one_player_human_always_starts`
- `two_player_first_round_x_starts`
- `two_player_rematch_alternates_starter`
- `symbols_do_not_change_between_rounds`
- `rematch_resets_board_and_selection_to_center`

## 4. Testes da IA

### 4.1 Fácil

- `easy_returns_legal_move`
- `easy_never_returns_occupied_cell` (10.000 tabuleiros aleatórios)
- `easy_is_beatable_over_many_games` (amostragem estatística, não determinística
  rígida; roda com semente fixa e verifica taxa de vitória humana > 0)

### 4.2 Médio

- `medium_takes_immediate_win`
- `medium_blocks_immediate_loss`
- `medium_prefers_center_when_free`
- `medium_returns_legal_move`
- `medium_can_be_beaten_by_a_fork_with_fixed_seed`

### 4.3 Impossível

- `impossible_returns_legal_move`
- `impossible_never_loses_exhaustive` — percorre todas as sequências legais do
  humano (X começando) e todas as respostas da IA; afirma que X nunca vence.
- `impossible_draws_against_perfect_human` — humano também joga ótimo.
- `impossible_random_tie_break_still_never_loses` — repete com várias sementes.
- `impossible_move_is_optimal` — compara com referência independente de minimax
  em tabuleiros de teste.
- `impossible_move_completes_within_5ms`

### 4.4 Determinismo

- `seeded_ai_is_reproducible`
- `easy_and_medium_legal_across_random_boards` (proptest)

## 5. Testes de estado e fluxo

- `home_enter_one_player_goes_to_difficulty`
- `difficulty_enter_starts_game_with_selected_level`
- `home_enter_two_players_starts_game`
- `esc_on_home_quits`
- `esc_during_play_opens_pause`
- `pause_continue_returns_to_same_state`
- `pause_restart_resets_round_but_keeps_score`
- `pause_main_menu_resets_session`
- `pause_quit_exits`
- `round_over_rematch_keeps_score`
- `round_over_esc_goes_to_menu`
- `f2_toggles_language_without_changing_game`
- `m_toggles_sound_without_changing_game`
- `invalid_move_sets_status_for_900ms`
- `ai_turn_ignores_board_input`
- `ai_pending_is_cancelled_or_frozen_by_pause`
- `resize_during_play_preserves_state`
- `terminal_too_small_blocks_game`
- `screen_transitions_are_exhaustive` (proptest com sequências de ações)

## 6. Testes de i18n

- `all_keys_exist_in_both_languages`
- `no_empty_translation`
- `no_hardcoded_ui_strings_outside_catalog` (revisão + grep no CI local)
- `placeholders_are_well_formed`
- `formatted_turn_text_works_for_both_players`
- `pt_br_and_en_fit_max_lengths`
- `accented_glyphs_exist_in_embedded_fonts`
- `language_toggle_updates_all_visible_screens`
- `unknown_locale_falls_back_to_pt_br`

## 7. Testes de layout

### 7.1 Cálculo da área de jogo

Com `round` e clamps 80–140 × 24–45:

| Terminal | Área de jogo | Layout |
| --- | --- | --- |
| 192 × 54 | 134 × 38 | Wide |
| 160 × 45 | 112 × 32 | Compact |
| 220 × 60 | 140 × 42 | Wide |
| 120 × 34 | 84 × 24 | Minimal |
| 100 × 32 | 80 × 24 | Minimal |
| 96 × 28 | 80 × 24 | Minimal |
| 80 × 24 | 80 × 24 | Minimal |
| 79 × 23 | bloqueado | Aviso |

Testes:

- `layout_uses_70_percent_with_clamps`
- `layout_centers_content`
- `layout_never_overlaps_board_and_side_panel`
- `board_is_visually_square_within_5_percent`
- `footer_and_header_fit_in_each_breakpoint`
- `minimum_size_warning_below_80x24`
- `long_english_strings_do_not_overflow`
- `framebuffer_content_is_70_percent_and_16_9`
- `framebuffer_scales_at_1080p_1440p_768p`

### 7.2 Framebuffer

| Tela | Conteúdo esperado |
| --- | --- |
| 1920 × 1080 | 1344 × 756 em (288, 162) |
| 2560 × 1440 | 1792 × 1008 em (384, 216) |
| 1366 × 768 | 956 × 538 (escala ~0,711, centralizado) |
| 1280 × 720 | 896 × 504 em (192, 108) |

- O fundo deve cobrir 100% da tela.
- A área de jogo deve preservar 16:9.
- Nenhum elemento pode invadir a margem de segurança.
- Texto deve manter tamanho mínimo legível após a escala (≥ 16 px efetivos no
  rodapé).

## 8. Snapshots e capturas visuais

### 8.1 TUI

Usar `TestBackend` com terminais de 192 × 54 (área Wide 134 × 38), 160 × 46
(área Compact 112 × 32) e 80 × 24 (área Minimal), nos idiomas pt-BR e inglês,
para as telas:

- Home;
- Dificuldade;
- Partida 1P com tabuleiro parcial;
- Partida 2P;
- Pausa;
- Fim de rodada com vitória;
- Fim de rodada com empate;
- Aviso de terminal pequeno;
- Modo sem cor (se suportado pelo backend de teste).

Os snapshots devem ser aprovados manualmente. A Fase 2 usa um comparador leve
próprio em `tests/snapshots/*.txt` (sem `insta`), conforme D-023; regenere com
`UPDATE_SNAPSHOTS=1 cargo test`.

### 8.2 Capturas do canvas

`--capture-frame` deve gerar frames determinísticos para:

- Home em pt-BR e inglês;
- Dificuldade;
- Partida com X/O e cursor;
- Fim de rodada com linha vencedora;
- Pausa;
- Tema em cores e em `NO_COLOR` (se aplicável ao canvas).

Verificações:

- resolução 1344 × 756;
- posições de tabuleiro e painel;
- contraste;
- ausência de texto cortado;
- ausência de emojis;
- consistência entre execuções com o mesmo estado e tempo de animação zero.

Comparação pixel-perfect só é obrigatória na mesma plataforma/fonte. Em
plataformas diferentes, usar comparação perceptual ou revisão visual.

## 9. Testes de backend TUI

### 9.1 Protocolos de imagem

- Kitty: PNG de alta resolução, sem artefatos.
- WezTerm (Kitty/Sixel): detecção correta e fallback quando forçado.
- Konsole/foot (Sixel): imagem legível, FPS aceitável.
- GNOME Terminal: half-blocks truecolor.
- xterm 256 cores sem truecolor: half-blocks aproximados ou legado.
- `--image-protocol none`: legado.
- `--no-images`: legado/half-blocks sem tentar protocolo.
- Timeout de detecção não trava o startup.

### 9.2 Cores

- Truecolor: paleta exata.
- 256 cores: aproximação sem perda de legibilidade.
- 16 cores: X/O distinguíveis, foco visível.
- `NO_COLOR`: sem cor, ainda jogável.
- `--color never`: idem.
- Terminal com fundo claro/escuro: o jogo não depende do fundo do terminal fora
  da área de jogo.

### 9.3 Terminal e restauração

- Entrar e sair várias vezes sem corromper o terminal.
- `Ctrl+C` durante a Home, durante a partida e durante animação.
- Fechar a janela do terminal durante o jogo.
- Matar o processo com SIGTERM.
- Simular erro de backend e verificar restauração.
- Verificar que o cursor do terminal reaparece.
- Verificar que o alternate screen é encerrado.
- Verificar que nenhum log aparece no meio da tela.

## 10. Testes do backend framebuffer (v1.1)

> Automação implementada em `src/platform/fb.rs` (formatos, ABI, conversão),
> `src/render/fb.rs` (geometria, telas, blending) e `src/render/text.rs`
> (layout, medida e quebra). Os itens abaixo marcados como manuais dependem de
> console real e constam em `docs/12-v1.1-release-checklist.md`.

### 10.1 Inicialização

- `/dev/fb0` presente e acessível.
- `/dev/fb0` ausente: erro claro, sem panic.
- Permissão negada: mensagem com instrução do grupo `video`.
- Formato 32 bpp, 24 bpp e 16 bpp.
- Formato desconhecido: erro claro.
- `KDSETMODE` permitido: console entra em gráfico.
- `KDSETMODE` negado: erro acionável.
- `--fb-device` apontando para caminho inválido.

### 10.2 Exibição

- 1920 × 1080: conteúdo centralizado, 70%, sem bordas cortadas.
- 2560 × 1440 e 1366 × 768: escala e centralização corretas.
- Fundo cobre toda a tela.
- Cores corretas nos formatos suportados (sem canal trocado).
- Sem flicker excessivo; tearing aceitável.
- Animações a 60 FPS quando possível; mínimo 30 FPS.
- Captura de frame confere com a tela.

### 10.3 Entrada e console

- Setas, WASD, Enter, Espaço, Esc, F2, M, Q.
- Nenhum eco de teclas na tela.
- Troca de VT (Ctrl+Alt+F1..F6) e retorno.
- `Ctrl+C` restaura `KD_TEXT`.
- SIGTERM restaura.
- Sair pelo menu restaura.
- Reiniciar o jogo após sair funciona.
- Console não fica com cursor piscando ou texto residual.

### 10.4 Servidor real

Matriz mínima em Ubuntu Server 24.04 x86_64:

- máquina física com `/dev/fb0`;
- resolução 1920 × 1080;
- teclado local;
- usuário no grupo `video`;
- sem X/Wayland;
- sem dispositivo de áudio;
- iniciar com `--no-default-features` e `--backend fb`;
- jogar uma partida completa 1P e uma 2P;
- verificar restauração do console ao sair.

## 11. Testes de áudio

- Síntese determinística, duração esperada, sem NaN e pico ≤ 1,0.
- Com dispositivo: todos os eventos soam, volume baixo, sem clipping.
- Sem dispositivo: jogo inicia, log discreto, sem erro visível.
- `--no-audio`: nenhum acesso ao dispositivo.
- `M`: alterna mudo e o indicador atualiza.
- Eventos rápidos (mover cursor repetidamente) não travam a UI.
- Vitória + revanche rápida não deixa som preso.
- Build `--no-default-features` não referencia áudio.

## 12. Testes de assets

- `cargo xtask assets --check` passa.
- Todos os PNGs embutidos decodificam.
- Dimensões embutidas conferem com o manifesto.
- Fontes têm assinatura TTF/OTF e são embutidas.
- Cena do tabuleiro, assinatura de cache, selo e captura têm testes.
- Protocolo de imagem explícito resolve sem consultar o terminal.
- Binário funciona sem a pasta `assets/`.
- Nenhum asset acima do orçamento.
- Selos pt/en corretos e legíveis.
- Nenhum texto essencial apenas em imagem.
- Nenhum metadado de data/software nos PNGs.

## 13. Matriz manual de terminais

Executar em 1920 × 1080 e, no mínimo, uma vez em 1366 × 768:

| Terminal | Protocolo esperado | Resultado |
| --- | --- | --- |
| Kitty | Kitty Graphics | |
| WezTerm | Kitty/Sixel | |
| Konsole | Sixel | |
| foot | Sixel | |
| GNOME Terminal | Half-blocks | |
| xterm | Half-blocks/legado | |
| Terminal com `NO_COLOR=1` | Sem cor | |
| Terminal 80 × 24 | Minimal | |
| Terminal 79 × 23 | Aviso | |

Para cada terminal:

- iniciar, jogar, pausar, reiniciar, sair;
- redimensionar durante a partida;
- trocar idioma e som;
- verificar restauração;
- verificar legibilidade de X/O e do foco.

## 14. Desempenho

### 14.1 Medições

- Startup: tempo até a Home, com `hyperfine` ou medição interna.
- Memória: `/usr/bin/time -v` em execução curta e após 5 minutos.
- CPU ocioso: `top`/`htop` com o jogo na Home e na partida, sem animação.
- FPS: instrumentação de debug opcional (`VELHA_LOG=debug`) ou contador interno.
- Latência de entrada: percepção + testes com timestamp interno.
- IA: benchmark unitário de `choose_move`.
- Encode de imagem: benchmark do board scene em 640 × 640.
- Tamanho do binário: `ls -lh target/release/velha-classica`.

### 14.2 Metas

- Startup desktop < 400 ms; servidor < 700 ms.
- RSS < 120 MB.
- CPU ocioso TUI < 2%; FB < 8%.
- FB 60 FPS alvo, mínimo 30 FPS em 1080p.
- IA < 5 ms.
- Entrada < 50 ms.
- Binário < 15 MB.

Falhas de desempenho em hardware modesto devem ser registradas com CPU, GPU,
terminal e resolução.

## 15. Testes de robustez

- Matriz de eventos aleatórios (proptest) não deve causar panic.
- Sequências de resize durante animação.
- Alternância rápida de idioma/som.
- Abrir pausa durante o atraso da IA.
- Reiniciar durante animação de vitória.
- Sair durante encode de imagem.
- `--capture-frame` com caminho inválido.
- Assets corrompidos em build de desenvolvimento (teste injetado).
- Falha de alocação? Não é obrigatório simular, mas o código deve evitar
  alocações gigantes.

## 16. Portões de qualidade

Antes de cada release:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo xtask assets --check
cargo build --release
cargo build --release --no-default-features
```

E, manualmente:

- [ ] Matriz de terminais principais.
- [ ] Matriz de framebuffer no servidor real (v1.1) — pendente de VT e grupo `video`.
- [x] Testes automatizados do backend fb (185 no total).
- [ ] Partida completa 1P e 2P.
- [ ] IA Impossível sem derrota em 20 partidas manuais.
- [ ] Ambos os idiomas em todas as telas.
- [ ] Saída limpa em todos os caminhos.
- [ ] Nenhum terminal/console corrompido.

## 17. Definition of Done

### v1.0 — Desktop

- Testes unitários e de integração verdes.
- Snapshots aprovados.
- Matriz de terminais executada.
- Desempenho dentro das metas (ou desvios documentados).
- Assets e licenças completos.
- README e docs atualizados.
- Binário release gerado e testado fora do diretório do projeto.

### v1.1 — Framebuffer

- Tudo da v1.0 continua passando.
- Backend fbdev testado no servidor real. **(pendente)**
- Restauração de console validada. **(pendente no VT)**
- Capturas de frame aprovadas. **(geradas; aprovação pendente)**
- Instruções de permissão e diagnóstico documentadas.
- Limitações conhecidas (fbdev, VT, tearing) registradas.
