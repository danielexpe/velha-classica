# 03 — Design Visual e Layout

## 1. Conceito

**"Uma mesa antiga, um tabuleiro de papel e tinta."**

A direção visual é clássica, quente e tátil. O jogo não deve parecer um painel
cyberpunk nem um fliperama. A madeira escura funciona como mesa; o papel
envelhecido é o tabuleiro e o cartão de placar; o latão aparece em detalhes
funcionais (foco, linha vencedora, divisores). X e O são carimbos de tinta.

### Palavras-chave

- artesanal, silencioso, preciso, acolhedor, legível, atemporal.

### Referências de sensação

- tabuleiro de madeira com acabamento fosco;
- papel algodão levemente amarelado;
- tinta nanquim azul-petróleo e vinho;
- fita/ponteira de latão;
- tipografia serifada de livro, com interface sans-serif discreta.

## 2. Princípios

1. **O tabuleiro é o herói.** A interface ao redor é moldura, não competição.
2. **Textura com moderação.** Grão e fibras devem ser percebidos, não dominar.
3. **Contraste antes de charme.** Nenhum ornamento pode reduzir a legibilidade.
4. **Movimento curto.** Animações confirmam ações; não bloqueiam o jogador.
5. **Mesma família visual nos dois backends.** O terminal é uma versão estilizada
   da mesa; o framebuffer tem a textura completa.
6. **Degradação honesta.** Sem imagem, o jogo continua bonito e legível; sem cor,
   continua jogável.

## 3. Canvas e área de jogo

### 3.1 Canvas de referência

- Tela-alvo: **1920 × 1080 px**.
- Área de jogo: **1344 × 756 px** = 70% de cada dimensão, 16:9.
- Posição em 1920 × 1080: `x = 288`, `y = 162`, centralizada.
- Fundo de madeira preenche a tela inteira; a composição de 70% fica por cima.
- Em outras resoluções, escalar o canvas de referência para 70% da tela atual,
  preservando 16:9 e centralizando.
- Escala mínima recomendada: 0,75. Abaixo disso, reduzir detalhes e priorizar
  texto.

### 3.2 Composição no framebuffer (canvas 1344 × 756)

| Elemento | Retângulo (x, y, w, h) | Observações |
| --- | --- | --- |
| Fundo | 0, 0, 1344, 756 | Madeira + grão + vinheta |
| Cabeçalho | 48, 28, 1248, 112 | Título, subtítulo, status |
| Divisor | 48, 140, 1248, 2 | Latão com falhas sutis |
| Tabuleiro | 154, 170, 520, 520 | Papel, grade 468 × 468, padding 26 |
| Painel lateral | 730, 170, 460, 520 | Papel, status, placar, atalhos |
| Rodapé | 48, 708, 1248, 24 | Atalhos e versão |

Cálculos do tabuleiro:

- Papel: 520 × 520, cantos levemente irregulares e sombra suave.
- Grade: 468 × 468, centralizada.
- Célula: 148 × 148.
- Espaço entre células: 12 px.
- Marca: ~112 px de altura visual.
- Linha da grade: 3 px, traço irregular, cor de tinta secundária.

Painel lateral:

- Padding interno: 28 px.
- Bloco de status: parte superior.
- Divisor de latão.
- Placar em três linhas.
- Atalhos contextuais na base.

### 3.3 Layout no terminal

No terminal, a área de jogo é 70% das células, centralizada e limitada:

- `cols = clamp(round(term_cols * 0.70), 80, 140)`.
- `rows = clamp(round(term_rows * 0.70), 24, 45)`.
- Com célula de 10 × 20 px em 1920 × 1080: terminal ≈ 192 × 54;
  área de jogo ≈ **134 × 38**.
- Se o terminal for menor que 80 × 24, mostrar apenas a mensagem de tamanho
  mínimo.
- O fundo fora da área de jogo usa `surface.wood.deep`.

Cabeçalho e rodapé:

| Breakpoint | Cabeçalho | Rodapé | Layout principal |
| --- | --- | --- | --- |
| Wide (≥120 × 34) | 4–6 linhas | 2 linhas | Tabuleiro à esquerda, painel à direita |
| Compact (≥96 × 28) | 2–3 linhas | 1–2 linhas | Tabuleiro central, status/placar abaixo |
| Minimal (≥80 × 24) | 1 linha | 1 linha | Tabuleiro, status e atalhos essenciais |

- O tabuleiro é quadrado: `lado = min(largura_disponível, altura_disponível × 2)`.
- A cena é sempre renderizada em proporção 1:1 e encaixada no espaço.
- O painel lateral só existe no layout Wide.
- Textos de status têm prioridade sobre ornamentos quando o espaço aperta.

### 3.4 Breakpoints e comportamento

| Faixa | Colunas × linhas | Comportamento |
| --- | --- | --- |
| Wide | ≥ 120 × 34 | Composição completa, painel lateral, título com subtítulo |
| Compact | 96–119 × 28–33 | Título curto, status acima/abaixo do tabuleiro, sem painel lateral |
| Minimal | 80–95 × 24–27 | Título de uma linha, status condensado, atalhos essenciais |
| Bloqueado | < 80 ou < 24 | Aviso centralizado com o tamanho atual e o mínimo |

## 4. Paleta

### 4.1 Tokens

| Token | Hex | Uso |
| --- | --- | --- |
| `surface.wood.deep` | `#1E140E` | Fundo externo, mesa |
| `surface.wood` | `#2B1C12` | Fundo principal da mesa |
| `surface.wood.mid` | `#4A2F1D` | Grão, sombras de madeira |
| `surface.wood.light` | `#6B4728` | Realces de grão |
| `surface.paper` | `#EAD9B7` | Papel do tabuleiro e painéis |
| `surface.paper.light` | `#F7E9CD` | Realce de papel, hover |
| `surface.paper.shadow` | `#CDB88F` | Sombra/borda de papel |
| `text.on.paper` | `#2A2118` | Texto principal em papel |
| `text.on.paper.muted` | `#6E5A43` | Texto secundário em papel |
| `text.on.wood` | `#F2E5CC` | Texto principal em madeira |
| `text.on.wood.muted` | `#B9A78E` | Texto secundário em madeira |
| `mark.x` | `#284B5A` | Tinta do X |
| `mark.o` | `#8E3B2F` | Tinta do O |
| `brass` | `#C7A23A` | Divisores, foco, linha vencedora |
| `brass.bright` | `#E6C766` | Foco ativo, brilho de latão |
| `success` | `#6F8A56` | Confirmações discretas |
| `danger` | `#A94438` | Erro, jogada inválida |
| `focus` | `#E6C766` | Foco de menu, brackets |
| `shadow` | `#120B06` | Sombras profundas |
| `overlay` | `#0B0704` | Sobreposição de pausa (82%) |

### 4.2 Degradação ANSI

| Token | 256 cores | 16 cores |
| --- | --- | --- |
| `surface.wood.deep` | 234 | Preto |
| `surface.wood` | 235 | Preto |
| `surface.wood.mid` | 94 | Preto/cinza escuro |
| `surface.wood.light` | 137 | Cinza |
| `surface.paper` | 230 | Branco |
| `surface.paper.light` | 231 | Branco brilhante |
| `surface.paper.shadow` | 180 | Cinza |
| `text.on.paper` | 235 | Preto |
| `text.on.paper.muted` | 240 | Cinza escuro |
| `text.on.wood` | 230 | Branco |
| `text.on.wood.muted` | 180 | Cinza claro |
| `mark.x` | 24 | Ciano brilhante |
| `mark.o` | 124 | Vermelho brilhante |
| `brass` | 178 | Amarelo |
| `brass.bright` | 220 | Amarelo brilhante |
| `success` | 64 | Verde |
| `danger` | 160 | Vermelho |
| `focus` | 220 | Amarelo brilhante |
| `shadow` | 232 | Preto |
| `overlay` | 233 | Preto |

- `NO_COLOR` ou `--color never`: remover cor, manter formas, negrito, bordas e
  símbolos. X e O continuam distinguíveis pelo glifo/forma.
- Nenhum estado depende só de cor.

### 4.3 Contraste

- Texto normal em papel: `#2A2118` sobre `#EAD9B7` (alto contraste).
- Texto normal em madeira: `#F2E5CC` sobre `#2B1C12` (alto contraste).
- Texto secundário em papel: `#6E5A43` sobre `#EAD9B7` (≥ 4,5:1).
- X e O têm contraste ≥ 4,5:1 sobre o papel e formas distintas.
- Latão sobre madeira é usado em textos grandes, ícones e linhas; para texto
  pequeno, preferir `text.on.wood`.
- `success` não é usado para texto pequeno sobre papel.

## 5. Tipografia

### 5.1 Framebuffer

Fontes embutidas (OFL 1.1):

- **EB Garamond SemiBold** — título, números do placar, selo.
- **EB Garamond Regular** — subtítulo e textos editoriais.
- **Inter Regular / SemiBold** — interface, status, atalhos e corpo.

Tamanhos no canvas 1344 × 756:

| Elemento | Fonte | Tamanho | Peso |
| --- | --- | --- | --- |
| Título | EB Garamond | 64 px | SemiBold |
| Subtítulo | Inter | 22 px | Regular, tracking 2 px |
| Status principal | Inter | 30 px | SemiBold |
| Texto de apoio | Inter | 22 px | Regular |
| Números do placar | EB Garamond | 40 px | SemiBold |
| Botões/menu | Inter | 26 px | SemiBold |
| Rodapé | Inter | 16 px | Regular |
| Selo de resultado | EB Garamond | 56 px | SemiBold, tracking 6 px |
| Metadados do cabeçalho | Inter | 18 px | Regular |

- Escalar proporcionalmente ao fator da tela.
- Não usar menos de 16 px efetivos no rodapé.
- Textos longos quebram linha; não truncar sem reticências.
- Números do placar usam algarismos tabulares.

### 5.2 Terminal

- Usa a fonte monoespaçada do terminal; não embute fonte na TUI.
- Recomendação: JetBrains Mono, Fira Code, Ubuntu Mono ou DejaVu Sans Mono,
  14–16 pt, com suporte a latim acentuado e box drawing.
- Título: `✦ V E L H A   C L Á S S I C A ✦` em latão, com espaçamento por
  caracteres.
- Subtítulo: `Jogo da Velha · Mesa de Madeira` em texto secundário.
- Cabeçalhos de seção em caixa alta e negrito.
- Evitar depender de Nerd Fonts, emojis ou glifos exóticos.
- Glifos permitidos: `✦ · ▸ ─ │ ┌ ┐ └ ┘ ├ ┤ ┬ ┴ ┼ ╭ ╮ ╰ ╯ ✕ ◯ █ ▀ ▄`.
- Se o terminal não renderizar um glifo, a interface deve continuar legível.

## 6. Componentes

### 6.1 Fundo de madeira

- Gradiente radial sutil do centro para as bordas.
- Grão horizontal, com linhas longas e baixa opacidade.
- Vinheta escura nas bordas.
- Sem ruído de alta frequência que atrapalhe o texto.
- No terminal: cor sólida `surface.wood.deep` + faixas discretas opcionais.

### 6.2 Papel

- Base `surface.paper`, com fibras e manchas muito sutis.
- Borda levemente irregular, sombra suave e um fio de latão em um dos lados.
- Cantos levemente arredondados (4–8 px), sem cartão "flutuante" genérico.
- No terminal: fundo papel, borda em `surface.paper.shadow` e detalhe em latão.

### 6.3 Grade do tabuleiro

- Traço de tinta marrom-escuro, 3 px, com pontas ligeiramente irregulares.
- Pequenas falhas e variação de opacidade, como desenho à mão.
- Interseções sem "bolinhas"; o cruzamento é natural.
- Não usar linhas neon ou sombras fortes.

### 6.4 Marcas X e O

- X: tinta azul-petróleo `mark.x`, traço de pincel/carimbo, leve rotação (-2° a
  +2°) e bordas com sangramento.
- O: tinta vinho `mark.o`, anel irregular, levemente oval, com falhas de tinta.
- Ambas têm textura interna sutil e alpha nas bordas.
- A marca fantasma (pré-visualização) usa a mesma forma com 22–28% de opacidade.
- Ao colocar: escala 0,72 → 1,04 → 1,00 em 220 ms, com leve overshoot; o
  sangramento aparece nos primeiros 80 ms.
- No tier legado: `X` e `O` em negrito, centralizados, com cor e forma distintas;
  sem tentar simular textura.

### 6.5 Cursor

- Quatro cantos de latão ao redor da célula selecionada.
- Pulso de opacidade 0,55 → 1,00 em 1,4 s, apenas no turno do humano.
- A marca fantasma aparece dentro dos cantos.
- Em movimento inválido, os cantos piscam em `danger` uma vez.
- No terminal legado: usar `▸` ou molduras `┌ ┐ └ ┘` em volta da célula.

### 6.6 Linha vencedora

- Traço de latão/grafite com 8 px, pontas arredondadas.
- Desenhada progressivamente em 500 ms, da primeira à última célula.
- Depois, brilho suave por 1,6 s, sem piscar.
- As três células vencedoras recebem um leve realce de papel.
- O status textual também informa o vencedor; a linha não é o único indicador.

### 6.7 Selo de resultado

- Selo circular/retangular com borda de latão e texto em serifa.
- Textos localizados: "VITÓRIA" / "WIN", "EMPATE" / "DRAW".
- Entra com escala 0,85 → 1,00 e rotação -3° → 0° em 320 ms.
- No terminal: usar caixa com borda dupla e texto em caixa alta.

### 6.8 Menus

- Itens em Inter SemiBold.
- Item focado: marcador de latão à esquerda, fundo papel claro e texto escuro.
- Itens não focados: texto sobre madeira ou papel, sem caixa.
- Transição de foco em 120 ms.
- Sem ícones de mouse; atalhos de teclado no rodapé.

### 6.9 Painel lateral

- Cartão de papel com:
  - status do turno em destaque;
  - símbolo do humano/IA;
  - placar da sessão;
  - atalhos contextuais.
- Separadores de latão finos.
- No layout compacto, vira uma faixa abaixo do tabuleiro.
- No minimal, apenas status + placar em uma linha.

### 6.10 Sobreposições

- Pausa: fundo `overlay` a 82%, cartão de papel centralizado, sem esconder
  completamente o tabuleiro.
- Fim de rodada: não usar overlay de tela cheia; manter o tabuleiro e a linha
  vencedora visíveis. O cartão de resultado aparece sobre a mesa, ao lado ou
  abaixo do tabuleiro.

## 7. Wireframes

Os wireframes são conceituais e usam menos colunas que a tela real. Eles mostram
hierarquia e posição, não medidas exatas.

### 7.1 Home

```text
                 ✦  V E L H A   C L Á S S I C A  ✦
                    Jogo da Velha · Mesa de Madeira
                 ─────────────────────────────────────

                            ▸  1 Jogador
                               2 Jogadores
                               Sair

                 F2 Idioma      M Som             v1.0.0
```

### 7.2 Seleção de dificuldade

```text
                 ✦  V E L H A   C L Á S S I C A  ✦
                        Escolha a dificuldade

                 ▸  Fácil
                    Joga sem muita estratégia.

                    Médio
                    Aproveita erros e bloqueia ameaças.

                    Impossível
                    Nunca perde. Empate já é vitória.

                 Esc Voltar                        F2 Idioma
```

### 7.3 Partida — Wide

```text
┌────────────────────────────────────────────────────────────────────────────────┐
│  ✦ VELHA CLÁSSICA ✦                              Português · Som ligado  v1.0 │
│  Jogo da Velha · Mesa de Madeira                                               │
│  ────────────────────────────────────────────────────────────────────────────  │
│                                                                                │
│      ╭──────────────────────────────╮      ╭───────────────────────────────╮   │
│      │                              │      │  VEZ DE VOCÊ                  │   │
│      │       ┌─────┬─────┬─────┐    │      │  Você joga com X              │   │
│      │       │     │     │     │    │      │                               │   │
│      │       ├─────┼─────┼─────┤    │      │  PLACAR DA SESSÃO             │   │
│      │       │     │  X  │     │    │      │  Você       1                 │   │
│      │       ├─────┼─────┼─────┤    │      │  IA         0                 │   │
│      │       │     │     │     │    │      │  Empates    0                 │   │
│      │       └─────┴─────┴─────┘    │      │                               │   │
│      │                              │      │  ───────────────────────────  │   │
│      ╰──────────────────────────────╯      │  Setas/WASD  mover            │   │
│                                            │  Enter       marcar           │   │
│                                            │  Esc         pausar           │   │
│                                            ╰───────────────────────────────╯   │
│                                                                                │
│  F2 Idioma    M Som    Esc Pausar                                   v1.0.0     │
└────────────────────────────────────────────────────────────────────────────────┘
```

Observações:

- O espaço do tabuleiro é preenchido pela cena PNG ou half-blocks; o wireframe
  mostra apenas a grade.
- O painel lateral usa papel; o fundo externo é madeira.
- No layout Wide, o tabuleiro e o painel ficam alinhados pelo topo.

### 7.4 Partida — Compact

```text
┌──────────────────────────────────────────────────────────────────────────────┐
│ VELHA CLÁSSICA                    Português · Som ligado              v1.0   │
├──────────────────────────────────────────────────────────────────────────────┤
│                    VEZ DE VOCÊ · VOCÊ JOGA COM X                             │
│                                                                              │
│     ╭────────────────────────────────────────────────────────────────────╮   │
│     │                                                                    │   │
│     │                         [ TABULEIRO ]                              │   │
│     │                                                                    │   │
│     ╰────────────────────────────────────────────────────────────────────╯   │
│                                                                              │
│     Você 1     ·     IA 0     ·     Empates 0                                │
│                                                                              │
│     Setas/WASD mover · Enter marcar · Esc pausar · F2 idioma · M som        │
└──────────────────────────────────────────────────────────────────────────────┘
```

### 7.5 Pausa

```text
                  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
                  ░                                   ░
                  ░        ╭─────────────────╮        ░
                  ░        │     PAUSADO     │        ░
                  ░        │                 │        ░
                  ░        │  ▸ Continuar    │        ░
                  ░        │    Reiniciar    │        ░
                  ░        │    Menu         │        ░
                  ░        │    Sair         │        ░
                  ░        ╰─────────────────╯        ░
                  ░                                   ░
                  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
```

### 7.6 Fim da rodada

```text
┌────────────────────────────────────────────────────────────────────────────────┐
│  ✦ VELHA CLÁSSICA ✦                              Português · Som ligado  v1.0 │
│  ────────────────────────────────────────────────────────────────────────────  │
│                                                                                │
│      ╭──────────────────────────────╮      ╭───────────────────────────────╮   │
│      │                              │      │        ✦ VITÓRIA ✦            │   │
│      │       ┌─────┬─────┬─────┐    │      │                               │   │
│      │       │  X  │  X  │  X  │    │      │  Você venceu!                 │   │
│      │       ├─────┼─────┼─────┤    │      │                               │   │
│      │       │  O  │  O  │     │    │      │  Você       2                 │   │
│      │       ├─────┼─────┼─────┤    │      │  IA         0                 │   │
│      │       │     │     │     │    │      │  Empates    0                 │   │
│      │       └─────┴─────┴─────┘    │      │  ───────────────────────────  │   │
│      │         linha vencedora      │      │  ▸ Revanche                   │   │
│      ╰──────────────────────────────╯      │    Menu principal             │   │
│                                            │    Sair do jogo               │   │
│                                            ╰───────────────────────────────╯   │
│  F2 Idioma    M Som                                              v1.0.0         │
└────────────────────────────────────────────────────────────────────────────────┘
```

## 8. Tiers de renderização

### 8.1 Tier A — PNG via protocolo de terminal

- Kitty Graphics Protocol (recomendado), Sixel ou iTerm2.
- Cena do tabuleiro renderizada em 640 × 640, exibida em escala.
- Textura de papel, grão, sombras, marcas e linha vencedora completos.
- UI em texto do terminal, usando a paleta e os glifos definidos.
- Melhor opção para o desktop.

### 8.2 Tier B — Half-blocks truecolor

- Mesma cena, reduzida para a grade de células com `▀` e cores 24-bit.
- Renderização em LOD baixo:
  - grade mais grossa;
  - sem grão fino nem sombra complexa;
  - marcas com contraste reforçado;
  - papel e madeira em cores chapadas com um leve gradiente.
- Em 60 × 30 células, o tabuleiro tem cerca de 60 × 60 pixels lógicos; a
  legibilidade vem da simplificação.
- Funciona em GNOME Terminal, Konsole, xterm com truecolor e similares.

### 8.3 Tier C — Legado 16 cores

- Sem PNG.
- Tabuleiro desenhado com box drawing (`┌ ┬ ┐ ├ ┼ ┤ └ ┴ ┘`), papel branco/cinza e
  grade escura.
- X e O em negrito, centralizados, com cores distintas; um deles pode usar fundo
  invertido para reforçar a diferença.
- Prioriza compatibilidade e legibilidade; não tenta reproduzir textura.
- Deve continuar jogável em 80 × 24.

### 8.4 Seleção de tier

```text
Imagem disponível? ── sim ──> Tier A (Kitty/Sixel/iTerm2)
        │
        não
        ▼
Truecolor disponível? ── sim ──> Tier B (half-blocks)
        │
        não
        ▼
Tier C (16 cores / legado)
```

`--no-images` força Tier B/C. `--image-protocol none` é equivalente a desligar
PNG. O jogo nunca falha por falta de protocolo de imagem.

## 9. Animações

### 9.1 Tabela

| Evento | Duração | Easing | Tier A/B | Tier C | Reduzido |
| --- | --- | --- | --- | --- | --- |
| Foco de menu | 120 ms | linear | Cor e marcador | Cor | Imediato |
| Movimento do cursor | 90 ms | ease-out | Brackets deslizam | Salto | Imediato |
| Pulso do cursor | 1,4 s | seno | Alpha 0,55–1,0 | Brilho discreto | Desligado |
| Marca fantasma | 160 ms | ease-out | Fade-in | Sem fantasma | Imediato |
| Colocar X/O | 220 ms | ease-out-back | Escala + tinta | Flash de cor | Imediato |
| Linha vencedora | 500 ms | ease-out | Traço progressivo | Linha aparece | Imediato |
| Brilho da vitória | 1,6 s | seno | Pulso suave | Sem pulso | Desligado |
| Selo de resultado | 320 ms | ease-out | Escala + rotação | Aparece | Imediato |
| Cartão de resultado | 260 ms | ease-out | Fade + slide | Aparece | Imediato |
| IA pensando | 500 ms/ciclo | linear | Três pontos | Três pontos | Estático |
| Jogada inválida | 180 ms | ease-out | Tremor 4 px | 1 flash | 1 flash |
| Troca de tela | 200 ms | ease-out | Crossfade | Corte | Corte |

### 9.2 Regras

- Nada pisca mais de 3 vezes por segundo.
- Animações não bloqueiam entrada; a ação é aplicada imediatamente e a animação
  apenas representa o resultado.
- A linha vencedora só começa depois que o resultado é computado.
- O som acompanha o início da animação, não o fim.
- `--no-animations` mantém feedback por cor/estado e reduz o atraso da IA para
  250 ms.
- No TUI, animações de tabuleiro só rodam enquanto forem necessárias; depois a
  cena fica estática para economizar CPU.
- Em Sixel, limitar a 15 FPS para animações de cena; Kitty pode usar 30 FPS.
- Em half-blocks, 20–30 FPS é suficiente.

## 10. Som no design

O som é parte da experiência tátil, mas nunca é obrigatório. Volume baixo,
ataques curtos e sem música.

| Evento | Som | Duração | Caráter |
| --- | --- | --- | --- |
| Mover cursor | Tick senoidal ~1,2 kHz | 25 ms | Sequinho de madeira |
| Colocar X | Ruído + seno ~180 Hz | 80 ms | Batida seca |
| Colocar O | Ruído filtrado | 120 ms | Papel/tecido |
| Inválido | Seno ~110 Hz | 70 ms | Baque abafado |
| Selecionar | Blip ~660 Hz | 40 ms | Confirmação discreta |
| Vitória | Arpejo C5–E5–G5–C6 | 600 ms | Sino de latão |
| Empate | G4–C4 | 400 ms | Resolução neutra |
| Pausa | Silêncio | — | — |

- Volume padrão: 0,30–0,35.
- `M` alterna mudo; o estado aparece no cabeçalho/rodapé.
- Sem dispositivo de áudio: silêncio, sem erro bloqueante.
- Sem música de fundo na v1.0.

## 11. Microcopy e hierarquia

- Títulos de tela em caixa alta/serifada; corpo em sans-serif.
- Status do turno é a informação mais importante durante a partida.
- Placar é secundário, mas sempre visível no layout Wide.
- Atalhos ficam no rodapé, em texto menor e tom secundário.
- Mensagens de erro são curtas e não usam jargão técnico.
- "Impossível" é um nível, não uma provocação; a descrição explica que empate é
  uma conquista.

## 12. Acessibilidade visual

- Contraste mínimo 4,5:1 para texto pequeno; 3:1 para texto grande.
- X e O têm silhuetas distintas; a cor é reforço.
- Foco de menu tem marcador de forma além da cor.
- Linha vencedora tem indicação textual no status.
- `--no-animations` e `NO_COLOR` documentados no README.
- Não usar texto dentro de PNG para informações essenciais; textos de UI são
  renderizados pelo terminal ou por fonte, garantindo leitura e i18n.
- Selos localizados em PNG são apenas reforço visual; o status textual sempre
  informa o resultado.

## 13. Regras de produção de arte

- Todos os assets são originais e gerados a partir de SVG.
- Paleta restrita aos tokens; variações de opacidade são permitidas.
- Madeira: grão horizontal, sem veios em espiral.
- Papel: fibras quase invisíveis, sem aparência de pergaminho fantasia.
- Tinta: bordas irregulares, sem gloss.
- Latão: fosco, não cromado; brilho contido.
- Sombras: difusas, deslocadas para baixo, sem preto puro.
- Nada de neon, glitch, scanlines ou emojis.
- Texturas não podem reduzir o contraste do texto.
- Assets não contêm texto, exceto selos localizados.

## 14. Checklist de QA visual

- [ ] Em 1920 × 1080, a composição ocupa ~70% e está centralizada.
- [ ] Em 134 × 38 células, o tabuleiro parece quadrado.
- [ ] Em 100 × 32 e 80 × 24, nada se sobrepõe e o texto é legível.
- [ ] X e O são distinguíveis em preto e branco.
- [ ] A linha vencedora é visível sem cobrir as marcas.
- [ ] O foco de menu é inequívoco.
- [ ] Textos em pt-BR e inglês cabem sem truncar.
- [ ] Tier A, B e C são visualmente coerentes.
- [ ] A pausa não esconde completamente o tabuleiro.
- [ ] O resultado não bloqueia a visão da linha vencedora.
- [ ] Animações não piscam e podem ser desligadas.
- [ ] O contraste atende aos mínimos.
- [ ] Nenhum emoji aparece.
- [ ] O tema parece madeira/papel, não neon nem fliperama.
