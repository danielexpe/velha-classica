# Velha Clássica

Jogo da velha para terminal Linux com visual de mesa de madeira e papel.

> **Versão inicial (Fase 1):** ainda **não é possível jogar**. Esta versão abre
> um placeholder da interface para validar o terminal, o layout e o núcleo de
> regras. A partida jogável chega na Fase 2.

## O que já funciona

- Placeholder da TUI nos layouts Wide, Compact e Minimal.
- Interface em pt-BR e inglês, alternada com `F2`.
- Núcleo de tabuleiro, regras, placar e estado de partida implementado e testado.
- CLI com ajuda, versão, idioma e escolha de backend.
- Restauração do terminal ao sair, inclusive com `Esc`, `Q` e `Ctrl+C`.

## Requisitos

- Linux com um terminal local (Zorin OS, Ubuntu e similares).
- Terminal de no mínimo **80 × 24** caracteres.
- Terminal com cores é recomendado; há fallback para 256 e 16 cores.
- Para compilar: **Rust 1.88 ou superior** (edição 2021).

## Instalar o Rust

Se ainda não tiver Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"
```

## Como rodar

Dentro da pasta do projeto:

```bash
# Desenvolvimento
cargo run

# Versão otimizada
cargo run --release

# Começar em inglês
cargo run -- --lang en

# Forçar o backend de terminal
cargo run -- --backend tui
```

Outras opções:

```bash
cargo run -- --help
cargo run -- --version
```

> `--backend fb` ainda retorna código de saída 3 de propósito: o backend de
> framebuffer só será implementado na v1.1.

Para desativar cores:

```bash
NO_COLOR=1 cargo run
```

## Controles desta versão

| Tecla | Ação |
| --- | --- |
| `F2` | Alterna pt-BR/inglês |
| `M` | Alterna o indicador de som (o áudio ainda não existe) |
| `Q`, `Esc`, `Ctrl+C` | Sair e restaurar o terminal |

## Como testar

Rodar a suíte automatizada:

```bash
cargo test
```

Resultado esperado nesta versão: **67 testes passando**.

Rodar os portões de qualidade:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo build --release
cargo build --release --no-default-features
```

Teste manual rápido:

1. Execute `cargo run` em um terminal local.
2. Confirme que o placeholder aparece e que o layout se adapta ao redimensionar.
3. Pressione `F2` e verifique os textos em inglês.
4. Pressione `Q` ou `Ctrl+C`.
5. Confirme que o cursor reapareceu, que a tela normal voltou e que não restou
   nenhum conteúdo do jogo no terminal.

## Problemas comuns

**“stdin and stdout must be a local terminal”**
Execute o jogo em um terminal real, não via pipe, redirecionamento ou CI.

**“Terminal muito pequeno. Mínimo: 80×24.”**
Aumente a janela do terminal até pelo menos 80 colunas por 24 linhas.

**“the framebuffer backend is not available in this version”**
Esperado ao usar `--backend fb`. Use `--backend tui` ou deixe o padrão `auto`.

**As cores parecem estranhas**
O jogo detecta truecolor, 256 e 16 cores automaticamente. Para jogar sem cor,
use `NO_COLOR=1 cargo run`.

## Limitações desta versão

- Sem partida, menu, dificuldade, IA, placar em tela ou revanche.
- Sem imagens, animações ou áudio.
- Sem backend de framebuffer.
- Sem persistência de idioma ou configurações.
- A TUI é apenas um esqueleto visual; o próximo passo é a Fase 2.

## Documentação do projeto

As especificações completas estão em [`docs/`](docs/):

- [Game design](docs/01-game-design.md)
- [Especificação técnica](docs/02-technical-spec.md)
- [Design visual](docs/03-visual-design.md)
- [Testes](docs/05-test-plan.md)
- [Roadmap](docs/06-roadmap.md)
- [Catálogo i18n](docs/08-i18n-catalog.md)

## Licença

Licença de código proposta: MIT. A licença final será confirmada antes da v1.0.
