# 04 — Assets e Licenças

## 1. Princípios

- Todos os assets de arte são **originais** do projeto, criados como SVG e
  rasterizados para PNG.
- O build e a execução não dependem de internet.
- O binário é autossuficiente: assets e fontes são embutidos com `include_bytes!`.
- Nenhum asset de terceiros é usado em arte, exceto as fontes OFL.
- A geração é determinística e verificável por checksum.
- SVG é a fonte da verdade; PNG é artefato de build versionado.
- Nada de texto essencial dentro de PNG; a UI é traduzida em runtime.
- Selos de resultado podem ser PNGs localizados, mas o status textual sempre
  repete a informação.

## 2. Estrutura

```text
assets/
├── src/                         # SVGs originais
│   ├── wood_bg.svg
│   ├── paper_board.svg
│   ├── paper_panel.svg
│   ├── mark_x.svg
│   ├── mark_o.svg
│   ├── mark_x_ghost.svg
│   ├── mark_o_ghost.svg
│   ├── cursor_brackets.svg
│   ├── divider_brass.svg
│   ├── ornament_corner.svg
│   ├── vignette.svg
│   ├── grain_overlay.svg
│   ├── stamp_win_pt.svg
│   ├── stamp_win_en.svg
│   ├── stamp_draw_pt.svg
│   └── stamp_draw_en.svg
├── generated/                   # PNGs rasterizados e otimizados
│   ├── wood_bg_1920x1080.png
│   ├── paper_board_720.png
│   ├── paper_panel_512x640.png
│   ├── mark_x_512.png
│   ├── mark_o_512.png
│   ├── mark_x_ghost_512.png
│   ├── mark_o_ghost_512.png
│   ├── cursor_brackets_320.png
│   ├── divider_brass_640x32.png
│   ├── ornament_corner_128.png
│   ├── vignette_1920x1080.png
│   ├── grain_overlay_1024.png
│   ├── stamp_win_pt_640x200.png
│   ├── stamp_win_en_640x200.png
│   ├── stamp_draw_pt_640x200.png
│   └── stamp_draw_en_640x200.png
├── fonts/
│   ├── EBGaramond-Regular.ttf
│   ├── EBGaramond-SemiBold.ttf
│   ├── Inter-Regular.ttf
│   ├── Inter-SemiBold.ttf
│   ├── OFL-EBGaramond.txt
│   └── OFL-Inter.txt
└── manifest.sha256
```

## 3. Nomenclatura

- IDs em `snake_case`.
- Dimensões no nome quando forem fixas: `mark_x_512.png`.
- Variações de estado com sufixo: `_ghost`, `_active`, `_hover`.
- Idiomas com sufixo `_pt` e `_en`.
- Nada de espaços, acentos ou maiúsculas em nomes de arquivo.
- SVG e PNG compartilham o mesmo nome-base.

## 4. Manifesto de assets

| ID | Origem SVG | Saída PNG | Dimensão | Alpha | Uso |
| --- | --- | --- | --- | --- | --- |
| `wood_bg` | `wood_bg.svg` | `wood_bg_1920x1080.png` | 1920 × 1080 | Não | Fundo do framebuffer |
| `paper_board` | `paper_board.svg` | `paper_board_720.png` | 720 × 720 | Sim | Papel do tabuleiro |
| `paper_panel` | `paper_panel.svg` | `paper_panel_512x640.png` | 512 × 640 | Sim | Painel lateral no framebuffer |
| `mark_x` | `mark_x.svg` | `mark_x_512.png` | 512 × 512 | Sim | Marca X |
| `mark_o` | `mark_o.svg` | `mark_o_512.png` | 512 × 512 | Sim | Marca O |
| `mark_x_ghost` | `mark_x_ghost.svg` | `mark_x_ghost_512.png` | 512 × 512 | Sim | Pré-visualização X |
| `mark_o_ghost` | `mark_o_ghost.svg` | `mark_o_ghost_512.png` | 512 × 512 | Sim | Pré-visualização O |
| `cursor_brackets` | `cursor_brackets.svg` | `cursor_brackets_320.png` | 320 × 320 | Sim | Foco do tabuleiro |
| `divider_brass` | `divider_brass.svg` | `divider_brass_640x32.png` | 640 × 32 | Sim | Divisor de latão |
| `ornament_corner` | `ornament_corner.svg` | `ornament_corner_128.png` | 128 × 128 | Sim | Canto de papel |
| `vignette` | `vignette.svg` | `vignette_1920x1080.png` | 1920 × 1080 | Sim | Vinheta de fundo |
| `grain_overlay` | `grain_overlay.svg` | `grain_overlay_1024.png` | 1024 × 1024 | Sim | Grão sobreposto |
| `stamp_win_pt` | `stamp_win_pt.svg` | `stamp_win_pt_640x200.png` | 640 × 200 | Sim | Selo "VITÓRIA" |
| `stamp_win_en` | `stamp_win_en.svg` | `stamp_win_en_640x200.png` | 640 × 200 | Sim | Selo "WIN" |
| `stamp_draw_pt` | `stamp_draw_pt.svg` | `stamp_draw_pt_640x200.png` | 640 × 200 | Sim | Selo "EMPATE" |
| `stamp_draw_en` | `stamp_draw_en.svg` | `stamp_draw_en_640x200.png` | 640 × 200 | Sim | Selo "DRAW" |

### 4.1 Observações por asset

- **`wood_bg`**: opaco, sem transparência, com grão horizontal e vinheta
  incorporada. Não deve ter texto.
- **`paper_board`**: fundo de papel com bordas irregulares e sombra suave. A grade
  do jogo é desenhada em runtime para se adaptar ao tamanho.
- **`paper_panel`**: similar ao tabuleiro, mais vertical; usado no framebuffer.
- **Marcas**: PNGs com alpha, sem texto, com margem de segurança para rotação e
  sangramento.
- **Ghost**: mesma arte com alpha reduzido; o runtime pode ajustar a opacidade.
- **Cursor**: quatro cantos de latão; o centro é transparente para mostrar a
  célula.
- **Selos**: contêm texto rasterizado, por isso são localizados e usados apenas
  como reforço visual; não substituem o status textual.
- **Vignette/grain**: overlays leves; nunca devem escurecer o texto além do
  contraste mínimo.

## 5. Especificação dos SVGs

- `viewBox` explícito e coordenadas em pixels.
- Sem elementos `<text>`; textos de selos são convertidos em caminhos.
- Sem imagens raster embutidas, scripts, links externos ou fontes externas.
- Sem filtros que dependam de recursos não suportados pelo `resvg`.
- `feTurbulence` é permitido apenas com semente fixa; se o resultado não for
  determinístico entre versões, substituir por caminhos/ruído gerado.
- Cores sempre pelos tokens da paleta; opacidades explícitas.
- sRGB.
- Nada de metadados de autor, data ou software no PNG final.
- IDs internos únicos e semânticos.
- Arte com margem de segurança; nada importante a menos de 4 px da borda.

### 5.1 Madeira

- Gradiente radial escuro.
- Veios horizontais longos, com 2–4 px de espessura e opacidade baixa.
- Nós de madeira ocasionais, fora das áreas de texto.
- Grão não pode criar padrões repetitivos evidentes.

### 5.2 Papel

- Base creme, fibras em baixa opacidade.
- Manchas suaves, sem aspecto de mapa.
- Borda levemente irregular, sem rasgos dramáticos.
- Sombra difusa para baixo e para a direita.

### 5.3 Tinta

- Contornos irregulares, mas limpos.
- X em azul-petróleo; O em vinho.
- Alpha nas bordas para simular absorção.
- Sem brilho, sem contorno neon.

### 5.4 Latão

- Amarelo fosco, sem reflexo especular forte.
- Divisores com pequenas falhas e pontas arredondadas.
- Foco e linha vencedora podem usar um brilho suave.

## 6. Pipeline de geração

### 6.1 Comando

```bash
cargo xtask assets
cargo xtask assets --check
```

- `assets` lê `assets/src/*.svg`, rasteriza para os tamanhos do manifesto,
  otimiza os PNGs e grava `assets/generated/` + `assets/manifest.sha256`.
- `--check` verifica se os PNGs e checksums correspondem às fontes; falha com
  código diferente de zero se estiverem desatualizados.
- `xtask` é um crate/binário de desenvolvimento; não entra no binário final.
- `resvg` é usado para rasterização; `oxipng` é opcional para otimização.
- Nenhum download é feito no build.

### 6.2 Etapas

1. Validar presença e sintaxe dos SVGs.
2. Rasterizar cada SVG no tamanho de saída, com fundo transparente quando
   indicado.
3. Aplicar otimização sem perda.
4. Remover metadados.
5. Calcular SHA-256 de cada PNG e escrever o manifesto em ordem estável.
6. Comparar com o manifesto anterior em modo `--check`.

### 6.3 Reprodutibilidade

- Versões de `resvg`/`oxipng` fixadas no `Cargo.lock`.
- Sem timestamps nos PNGs.
- Sementes fixas para qualquer ruído.
- Ordem de arquivos determinística.
- O commit inclui SVG, PNG e manifesto.

## 7. Embutição no binário

- `src/assets.rs` usa `include_bytes!` para cada PNG e fonte.
- A decodificação é feita no início do app e cacheada.
- Falha de decodificação é erro fatal claro (não deve acontecer em release).
- O binário não lê `assets/` em runtime; mover o executável não quebra o jogo.
- Em desenvolvimento, `cargo xtask assets --check` garante que os embutidos estão
  atualizados.

## 8. Fontes

### 8.1 Escolhidas

| Fonte | Uso | Licença |
| --- | --- | --- |
| EB Garamond | Título, números, selos | SIL Open Font License 1.1 |
| Inter | Interface, status, atalhos | SIL Open Font License 1.1 |

- Arquivos `.ttf` versionados em `assets/fonts/`.
- Licenças completas copiadas para `assets/fonts/OFL-*.txt`.
- Créditos no README e em `THIRD_PARTY_NOTICES.md` na implementação.
- A TUI usa a fonte do terminal; as fontes embutidas são para o framebuffer.
- Subset para latim + acentos portugueses é desejável, mas opcional; se usado,
  validar todos os textos do catálogo i18n.
- Não usar fontes sem licença explícita.

### 8.2 Fallback

- Se uma fonte falhar ao carregar, o framebuffer deve abortar com mensagem clara,
  pois não há fallback confiável sem dependências do sistema.
- A TUI não é afetada.

## 9. Áudio

- Nenhum arquivo de áudio é distribuído.
- Todos os sons são sintetizados em runtime (senoides, ruído e envelopes).
- Isso reduz o binário, evita licenças de áudio e permite desligar a feature.
- Não há música.

## 10. Orçamentos

| Categoria | Orçamento |
| --- | --- |
| PNGs | < 4 MB |
| Fontes | < 2,5 MB |
| Binário release | < 15 MB |
| SVG fontes | < 500 KB |
| Total do repositório de assets | < 8 MB |

- Se um asset estourar o orçamento, reduzir dimensão, simplificar textura ou
  aumentar a compressão sem perda visível.
- Medir o binário com `cargo build --release` e `ls -lh`.

## 11. Licenciamento

### 11.1 Proposta

- **Código:** MIT.
- **Arte original:** CC BY 4.0, com crédito "Velha Clássica — arte original".
- **Fontes:** OFL 1.1, com avisos preservados.
- **Crates:** licenças originais; listar em `THIRD_PARTY_NOTICES.md`.
- **Sons:** gerados pelo código; seguem a licença do código.

### 11.2 Regras

- Não usar arte de terceiros sem licença compatível e crédito.
- Não remover avisos de licença das fontes.
- Selos localizados são arte original.
- Se o projeto for distribuído, incluir `LICENSE`, `LICENSE-ASSETS` (ou equivalente)
  e `THIRD_PARTY_NOTICES.md`.
- A licença final deve ser confirmada antes da v1.0; ver questões abertas em
  [07 — Decisões e Riscos](07-decisions-and-risks.md).

## 12. QA de assets

- [ ] Todos os SVGs rasterizam sem erro.
- [ ] Todos os PNGs têm dimensões e alpha conforme o manifesto.
- [ ] Nenhum PNG contém metadados de data/software.
- [ ] Checksums batem com `cargo xtask assets --check`.
- [ ] Não há texto essencial dentro de PNG.
- [ ] Selos pt/en existem e estão corretos.
- [ ] Marcas X/O são distinguíveis em escala pequena.
- [ ] Texturas não reduzem contraste.
- [ ] Assets não têm bordas cortadas.
- [ ] Fontes e licenças estão presentes.
- [ ] Orçamentos de tamanho foram respeitados.
- [ ] O binário funciona sem a pasta `assets/`.

## 13. Adicionar um novo asset

1. Criar o SVG em `assets/src/`.
2. Adicionar a entrada no manifesto do `xtask` (ID, saída, dimensão, alpha).
3. Gerar com `cargo xtask assets`.
4. Inspecionar o PNG em 1× e em escala pequena.
5. Adicionar o asset ao `src/assets.rs` e ao renderer.
6. Atualizar `manifest.sha256`.
7. Rodar `cargo test`, `cargo xtask assets --check` e a matriz visual.
8. Atualizar esta documentação.
