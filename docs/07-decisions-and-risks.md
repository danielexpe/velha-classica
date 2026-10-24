# 07 — Decisões, Premissas e Riscos

## 1. Formato

Este documento registra decisões de arquitetura e produto no estilo ADR
(Architecture Decision Record). Cada decisão tem:

- **ID** e título;
- **status** (`aceita`, `proposta`, `adiada`, `substituída`);
- **contexto**;
- **decisão**;
- **consequências**.

Questões abertas e riscos ficam no final.

## 2. Decisões

### D-001 — Linux local é a plataforma oficial

- **Status:** aceita.
- **Contexto:** o jogo nasce para Zorin OS e Ubuntu Server, ambos locais.
- **Decisão:** suportar Linux x86_64 local. SSH, Windows e macOS ficam fora do
  escopo oficial.
- **Consequências:** podemos usar APIs Linux (framebuffer, ioctls, ALSA) e assumir
  um TTY local. A TUI ainda funciona em SSH como melhor esforço, mas sem garantia
  de imagens/áudio.

### D-002 — Rust stable + ratatui + crossterm

- **Status:** aceita.
- **Contexto:** é preciso uma TUI moderna, testável e com bom suporte a cores e
  eventos.
- **Decisão:** usar Rust stable, edição 2021, com `ratatui` e `crossterm`.
- **Consequências:** boa ergonomia e ecossistema; a lógica fica separada para não
  depender da TUI. Versões fixadas no `Cargo.lock`.

### D-003 — Dois backends de vídeo

- **Status:** aceita.
- **Contexto:** o desktop usa terminal; o servidor não tem X/Wayland e precisa de
  PNG real no console local.
- **Decisão:** compartilhar núcleo e cena do tabuleiro entre um backend TUI
  (`ratatui`) e um backend framebuffer (`/dev/fb0`). Desktop na v1.0; framebuffer
  na v1.1.
- **Consequências:** mais trabalho de renderização, porém atende aos dois cenários.
  O framebuffer não bloqueia a entrega do desktop.

### D-004 — Framebuffer baseline em fbdev

- **Status:** aceita.
- **Contexto:** DRM/KMS é mais moderno, mas mais complexo; muitos sistemas ainda
  expõem `/dev/fb0`.
- **Decisão:** v1.1 suporta fbdev (`/dev/fb0`) com 32/24/16 bpp. DRM/KMS fica no
  backlog para uma v1.2, caso necessário.
- **Consequências:** implementação menor e testável; sistemas sem `/dev/fb0`
  recebem erro claro. O usuário deve verificar o hardware antes da v1.1.

### D-005 — Imagens via SVG → PNG com fallback em três tiers

- **Status:** aceita.
- **Contexto:** PNGs reais são desejados, mas nem todo terminal suporta protocolo
  de imagem; o servidor usa framebuffer.
- **Decisão:** arte original em SVG, rasterizada para PNG e embutida. No terminal:
  Tier A (Kitty/Sixel/iTerm2), Tier B (half-blocks truecolor), Tier C (legado 16
  cores). No framebuffer: composição completa em PNG/vetor.
- **Consequências:** experiência consistente e degradação honesta. O pipeline de
  assets e o LOD precisam ser mantidos.

### D-006 — Tema clássico madeira e papel

- **Status:** aceita.
- **Contexto:** o usuário escolheu uma estética clássica, não neon ou CRT.
- **Decisão:** paleta de madeira escura, papel creme, tinta azul-petróleo/vinho e
  detalhes em latão; tipografia serifada para display e sans para UI.
- **Consequências:** texturas e fontes são parte do escopo; contraste e
  legibilidade têm prioridade sobre ornamento.

### D-007 — Modos 1P/2P e IA em três níveis

- **Status:** aceita.
- **Contexto:** o jogo deve funcionar sozinho e com duas pessoas.
- **Decisão:** 1 jogador vs IA (Fácil, Médio, Impossível) e 2 jogadores local.
- **Consequências:** a IA precisa de testes rigorosos; o modo 2P não depende de
  IA nem de áudio.

### D-008 — Humano começa no 1P; 2P alterna iniciante em revanches

- **Status:** aceita.
- **Contexto:** manter o modo 1P convidativo e o 2P justo.
- **Decisão:** no 1P, o humano é X e sempre começa. No 2P, X começa a primeira
  rodada; cada revanche alterna quem inicia, mantendo os símbolos.
- **Consequências:** regra simples e previsível; o placar da sessão acumula
  resultados.

### D-009 — Placar apenas em memória

- **Status:** aceita.
- **Contexto:** persistência não foi selecionada no escopo inicial.
- **Decisão:** o placar da sessão existe em memória e é zerado ao voltar ao menu,
  trocar modo/dificuldade ou sair.
- **Consequências:** nenhum arquivo de configuração, migração ou corrupção de
  dados. Persistência fica no backlog.

### D-010 — Somente teclado

- **Status:** aceita.
- **Contexto:** o terminal e o console local têm teclado; mouse não foi escolhido.
- **Decisão:** toda a interface é operável por teclado. Mouse fica no backlog.
- **Consequências:** menos complexidade de input e melhor suporte a framebuffer.
  A navegação precisa ser clara e os atalhos, documentados.

### D-011 — Áudio sintetizado, opcional e silencioso

- **Status:** aceita.
- **Contexto:** o servidor pode não ter dispositivo de áudio; arquivos externos
  aumentariam o pacote.
- **Decisão:** efeitos sintetizados com `rodio`, feature `audio` ligada por padrão;
  falha de dispositivo desabilita o som sem bloquear.
- **Consequências:** sem assets de áudio; build no servidor com
  `--no-default-features`. Sem música de fundo.

### D-012 — Área de jogo de 70% centralizada

- **Status:** aceita.
- **Contexto:** tela de 1920 × 1080; o usuário quer ~70% para o layout.
- **Decisão:** canvas de referência 1344 × 756 (70% de cada dimensão, 16:9),
  centralizado, com fundo de madeira em toda a tela. No terminal, 70% das células
  com clamps 80–140 × 24–45.
- **Consequências:** em 1920 × 1080 com célula 10 × 20, ~134 × 38 células. Telas
  menores usam breakpoints; abaixo de 80 × 24 há aviso.

### D-013 — Assets embutidos no binário

- **Status:** aceita.
- **Contexto:** evitar caminhos quebrados e simplificar a distribuição.
- **Decisão:** `include_bytes!` para PNGs e fontes; nenhum asset lido do disco em
  runtime.
- **Consequências:** binário único e portátil; aumento de tamanho controlado por
  orçamento. Atualizações exigem recompilar.

### D-014 — Fontes OFL embutidas

- **Status:** proposta (confirmar).
- **Contexto:** o framebuffer precisa renderizar texto com qualidade e acentos.
- **Decisão:** usar EB Garamond (display) e Inter (UI), ambas OFL 1.1, embutidas.
  A TUI usa a fonte do terminal.
- **Consequências:** dependência de licença de fonte, não de arte. Alternativa
  seria usar fontes do sistema, com menos consistência.

### D-015 — Licenciamento proposto

- **Status:** proposta (confirmar antes da v1.0).
- **Contexto:** o projeto pode ou não ser distribuído.
- **Decisão inicial:** código MIT; arte original CC BY 4.0; fontes OFL 1.1;
  crates com suas licenças.
- **Consequências:** permite uso e contribuição amplos com atribuição. Se o
  usuário quiser outro modelo, atualizar `LICENSE`, `LICENSE-ASSETS` e créditos.

### D-016 — Sem arquivo de configuração ou persistência

- **Status:** aceita.
- **Contexto:** o escopo inicial não pediu preferências salvas.
- **Decisão:** idioma, som, dificuldade e placar vivem só na sessão. CLI define
  opções iniciais.
- **Consequências:** inicialização previsível; o usuário sempre começa em pt-BR.
  Backlog pode adicionar persistência.

### D-017 — Sem música de fundo

- **Status:** aceita.
- **Contexto:** manter o clima silencioso e reduzir escopo/licenças.
- **Decisão:** apenas efeitos curtos; nenhuma trilha.
- **Consequências:** menos complexidade de áudio e menor risco de fadiga.

### D-018 — Backend auto conservador

- **Status:** aceita.
- **Contexto:** um desktop pode ter `/dev/fb0`, mas não devemos tomar o console
  gráfico por acidente.
- **Decisão:** `--backend auto` prefere TUI; só escolhe framebuffer com sinais
  claros de console local (TERM=linux, sem DISPLAY/WAYLAND, TTY local). No
  servidor, recomendar `--backend fb`.
- **Consequências:** evita surpresas no desktop; exige uma flag explícita no
  servidor.

### D-019 — Fallback de imagem obrigatório

- **Status:** aceita.
- **Contexto:** protocolos de imagem variam e podem falhar.
- **Decisão:** detectar protocolo; permitir override; cair para half-blocks e
  legado. O jogo nunca depende de PNG para funcionar.
- **Consequências:** mais caminhos de renderização para testar, mas alta
  compatibilidade.

### D-020 — Desempate aleatório na IA Impossível

- **Status:** aceita.
- **Contexto:** minimax tem várias jogadas ótimas; escolher sempre a primeira
  torna o jogo repetitivo.
- **Decisão:** escolher aleatoriamente entre jogadas de pontuação ótima, mantendo
  a garantia de nunca perder.
- **Consequências:** aberturas variadas; testes devem cobrir várias sementes.

### D-021 — Sem tutorial dedicado na v1.0

- **Status:** aceita.
- **Contexto:** o usuário selecionou apenas animações/feedback e som entre os
  extras; o jogo é simples.
- **Decisão:** controles ficam no rodapé e na pausa. Não há tela de tutorial.
- **Consequências:** menos escopo; a primeira tela precisa ser autoexplicativa.

### D-022 — Captura de frame para QA

- **Status:** aceita.
- **Contexto:** testar framebuffer sem hardware é difícil.
- **Decisão:** `--capture-frame <png>` renderiza um frame determinístico do canvas
  e encerra, sem abrir dispositivo interativo.
- **Consequências:** viabiliza snapshots e revisão visual; não substitui o teste
  no hardware real.

## 3. Premissas

1. O Zorin OS tem terminal local com suporte a truecolor; o usuário pode instalar
   Kitty ou WezTerm para o Tier A.
2. A tela principal é 1920 × 1080; o layout deve funcionar em 1366 × 768 e 80 × 24.
3. O Ubuntu Server permite console local com monitor/teclado e `/dev/fb0`; o
   usuário pode ajustar permissões/grupo `video`.
4. Não há requisito de rede em runtime.
5. O jogo é para uso local; SSH é melhor esforço.
6. O usuário aprova o tema, as fontes e a licença propostos ou solicita mudanças
   antes da Fase 1.
7. O binário pode ser distribuído como executável único com assets embutidos.
8. O público aceita jogar com teclado e sem mouse.
9. Não há requisito de acessibilidade além dos itens documentados (contraste,
   formas, redução de movimento, `NO_COLOR`).
10. A versão de Rust usada no servidor pode ser instalada/atualizada para atender
    à MSRV.

## 4. Questões abertas

| ID | Questão | Impacto | Prazo |
| --- | --- | --- | --- |
| Q-001 | Confirmar licença do código e da arte (MIT + CC BY 4.0?) | Legal/distribuição | Antes da v1.0 |
| Q-002 | Aprovar EB Garamond + Inter ou indicar outras fontes | Visual/licenças | Antes da Fase 4 |
| Q-003 | Confirmar hardware do Ubuntu Server: `/dev/fb0`, resolução, bpp | v1.1 | Antes da Fase 6 |
| Q-004 | Confirmar que o servidor pode usar grupo `video`/console local | v1.1 | Antes da Fase 6 |
| Q-005 | Preferência de volume padrão do áudio | UX | Fase 4 |
| Q-006 | Manter a marca "Velha Clássica" também na UI em inglês? | i18n | Fase 4 |
| Q-007 | Deseja persistência futura de placar/idioma? | Backlog | Pós-v1.1 |
| Q-008 | O modo Impossível deve ter atraso de "pensamento" ou ser instantâneo? | UX | Fase 3 |
| Q-009 | Limite mínimo de terminal: 80 × 24 é aceitável? | Layout | Fase 2 |
| Q-010 | Capturas de frame devem entrar no repositório ou ser artefato de release? | QA/repo | Fase 4 |

Recomendações iniciais:

- Q-001: MIT + CC BY 4.0.
- Q-002: manter EB Garamond + Inter.
- Q-005: volume 0,32.
- Q-006: manter "Velha Clássica" como marca e traduzir o subtítulo.
- Q-008: manter o atraso de 450–800 ms; em testes, zero.
- Q-009: sim, 80 × 24 como mínimo jogável.
- Q-010: manter apenas capturas pequenas de referência; gerar o restante no CI/
  release.

## 5. Registro de riscos

Probabilidade e impacto: baixa/média/alta.

| ID | Risco | Prob. | Impacto | Mitigação | Sinal de alerta |
| --- | --- | --- | --- | --- | --- |
| R-001 | Terminal sem protocolo de imagem | Alta | Médio | Tier B/C obrigatório; testar GNOME Terminal | Board ilegível no fallback |
| R-002 | Sixel lento/artefatado | Média | Médio | FPS adaptativo, LOD baixo, permitir `--no-images` | Input lag ou frames cortados |
| R-003 | `/dev/fb0` ausente no servidor | Média | Alto | Diagnóstico claro; DRM no backlog; validar cedo | Erro na inicialização do backend |
| R-004 | Permissões de framebuffer/VT | Alta | Médio | Instruções de grupo `video`; validar antes | `EACCES` em `/dev/fb0` |
| R-005 | Kernel sobrescreve o framebuffer | Média | Alto | `KD_GRAPHICS`, sem logs na tela, restauração | Texto do console piscando sobre o jogo |
| R-006 | Troca de VT quebra o estado | Média | Alto | Tratar sinais; restaurar e encerrar com mensagem | Jogo congela ao voltar do VT |
| R-007 | Célula de terminal não é 1:2 | Alta | Baixo | Layout compensa proporção; imagem preserva aspecto | Tabuleiro esticado |
| R-008 | Textos em inglês estouram | Média | Médio | Limites no catálogo + testes + quebra de linha | Truncamento ou sobreposição |
| R-009 | Fontes sem acentos ou glifos | Baixa | Alto | Testar catálogo completo; fontes OFL confiáveis | "ã" vira caixa |
| R-010 | Áudio indisponível no servidor | Alta | Baixo | Feature desligável; silêncio gracioso | Erro de ALSA bloqueante |
| R-011 | Escopo visual cresce | Média | Alto | Orçamento de assets; checklist; backlog | Fase 4 sem fim |
| R-012 | Dependências de TUI mudam | Média | Médio | `Cargo.lock`; camada de adaptação; MSRV | Build quebra após update |
| R-013 | Terminal corrompido em panic | Baixa | Alto | Guards RAII, panic hook, testes de restauração | Cursor escondido/raw mode preso |
| R-014 | IA Impossível falha em caso de borda | Baixa | Alto | Teste exaustivo + proptest | Derrota em simulação |
| R-015 | `--backend auto` escolhe framebuffer no desktop | Baixa | Alto | Regra conservadora; padrão TUI; flag explícita | Console gráfico tomado |
| R-016 | Binário grande demais | Média | Baixo | Compressão de assets, subset de fontes, medir | > 15 MB |
| R-017 | Framebuffer com formato exótico | Média | Médio | Suportar 32/24/16; erro claro; DRM no backlog | Cores trocadas ou imagem quebrada |
| R-018 | Perda de foco/entrada no console | Média | Médio | Testar teclado local; documentar limitações | Teclas não chegam |

## 6. Critérios para revisitar decisões

Revisar um ADR quando:

- o hardware do servidor não expuser `/dev/fb0`;
- o usuário pedir persistência, mouse ou temas;
- a performance de Sixel inviabilizar a experiência;
- as fontes escolhidas não cobrirem os textos;
- a licença precisar mudar antes da distribuição;
- o escopo da v1.0 crescer além do documentado.

## 7. Histórico

| Data | Mudança |
| --- | --- |
| 2026-10-24 | Versão inicial do planejamento e das decisões. |
