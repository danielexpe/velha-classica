# Third-party notices

Velha Clássica includes third-party software and fonts. The full license texts
ship with the sources and with the packages listed in `Cargo.lock`.

## Rust crates

| Crate | License |
| --- | --- |
| `clap` | MIT OR Apache-2.0 |
| `crossterm` | MIT |
| `image` | MIT OR Apache-2.0 |
| `libloading` | ISC |
| `ratatui` | MIT |
| `ratatui-image` | MIT |
| `thiserror` | MIT OR Apache-2.0 |
| `tiny-skia` | BSD-3-Clause |

Build-time crates in `xtask`:

| Crate | License |
| --- | --- |
| `oxipng` | MIT |
| `resvg` | Apache-2.0 OR MIT |
| `sha2` | MIT OR Apache-2.0 |
| `tiny-skia` | BSD-3-Clause |

## Fonts

| Font | Copyright | License |
| --- | --- | --- |
| EB Garamond | Georg Duffner, Octavio Pardo, and contributors | SIL Open Font License 1.1 |
| Inter | Rasmus Andersson and contributors | SIL Open Font License 1.1 |

The complete OFL texts are in:

- `assets/fonts/OFL-EBGaramond.txt`
- `assets/fonts/OFL-Inter.txt`

The bundled TTF files are static instances derived from the upstream variable
fonts.

## Artwork

The SVG sources and generated PNGs are original artwork for this project,
licensed under CC BY 4.0 with the credit "Velha Clássica — arte original".
See `LICENSE-ASSETS` for the full license text.
