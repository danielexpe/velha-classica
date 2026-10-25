use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

use sha2::{Digest, Sha256};
use tiny_skia::Pixmap;

#[derive(Debug, Clone, Copy)]
struct Asset {
    id: &'static str,
    source: &'static str,
    output: &'static str,
    width: u32,
    height: u32,
    opaque: bool,
}

const ASSETS: &[Asset] = &[
    Asset {
        id: "wood_bg",
        source: "wood_bg.svg",
        output: "wood_bg_1920x1080.png",
        width: 1920,
        height: 1080,
        opaque: true,
    },
    Asset {
        id: "paper_board",
        source: "paper_board.svg",
        output: "paper_board_720.png",
        width: 720,
        height: 720,
        opaque: false,
    },
    Asset {
        id: "paper_panel",
        source: "paper_panel.svg",
        output: "paper_panel_512x640.png",
        width: 512,
        height: 640,
        opaque: false,
    },
    Asset {
        id: "mark_x",
        source: "mark_x.svg",
        output: "mark_x_512.png",
        width: 512,
        height: 512,
        opaque: false,
    },
    Asset {
        id: "mark_o",
        source: "mark_o.svg",
        output: "mark_o_512.png",
        width: 512,
        height: 512,
        opaque: false,
    },
    Asset {
        id: "mark_x_ghost",
        source: "mark_x_ghost.svg",
        output: "mark_x_ghost_512.png",
        width: 512,
        height: 512,
        opaque: false,
    },
    Asset {
        id: "mark_o_ghost",
        source: "mark_o_ghost.svg",
        output: "mark_o_ghost_512.png",
        width: 512,
        height: 512,
        opaque: false,
    },
    Asset {
        id: "cursor_brackets",
        source: "cursor_brackets.svg",
        output: "cursor_brackets_320.png",
        width: 320,
        height: 320,
        opaque: false,
    },
    Asset {
        id: "divider_brass",
        source: "divider_brass.svg",
        output: "divider_brass_640x32.png",
        width: 640,
        height: 32,
        opaque: false,
    },
    Asset {
        id: "ornament_corner",
        source: "ornament_corner.svg",
        output: "ornament_corner_128.png",
        width: 128,
        height: 128,
        opaque: false,
    },
    Asset {
        id: "vignette",
        source: "vignette.svg",
        output: "vignette_1920x1080.png",
        width: 1920,
        height: 1080,
        opaque: false,
    },
    Asset {
        id: "grain_overlay",
        source: "grain_overlay.svg",
        output: "grain_overlay_1024.png",
        width: 1024,
        height: 1024,
        opaque: false,
    },
    Asset {
        id: "stamp_win_pt",
        source: "stamp_win_pt.svg",
        output: "stamp_win_pt_640x200.png",
        width: 640,
        height: 200,
        opaque: false,
    },
    Asset {
        id: "stamp_win_en",
        source: "stamp_win_en.svg",
        output: "stamp_win_en_640x200.png",
        width: 640,
        height: 200,
        opaque: false,
    },
    Asset {
        id: "stamp_draw_pt",
        source: "stamp_draw_pt.svg",
        output: "stamp_draw_pt_640x200.png",
        width: 640,
        height: 200,
        opaque: false,
    },
    Asset {
        id: "stamp_draw_en",
        source: "stamp_draw_en.svg",
        output: "stamp_draw_en_640x200.png",
        width: 640,
        height: 200,
        opaque: false,
    },
];

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("xtask: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let check = std::env::args().skip(1).any(|arg| arg == "--check");

    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or("xtask must live inside the project")?
        .to_path_buf();
    let source_dir = root.join("assets").join("src");
    let output_dir = root.join("assets").join("generated");
    let manifest_path = root.join("assets").join("manifest.sha256");

    if !check {
        fs::create_dir_all(&output_dir)?;
    }

    let mut manifest = String::new();
    let mut stale = Vec::new();

    for asset in ASSETS {
        let png = render(&source_dir, asset)?;
        let path = output_dir.join(asset.output);
        let hash = sha256(&png);
        manifest.push_str(&format!("{hash}  generated/{}\n", asset.output));

        if check {
            let current = fs::read(&path).unwrap_or_default();

            if current != png {
                stale.push(asset.output);
            }
        } else {
            fs::write(&path, &png)?;
        }
    }

    if check {
        let current_manifest = fs::read_to_string(&manifest_path).unwrap_or_default();

        if !stale.is_empty() {
            return Err(format!("generated PNGs are stale: {}", stale.join(", ")).into());
        }

        if current_manifest != manifest {
            return Err("assets/manifest.sha256 is stale; run `cargo xtask assets`".into());
        }

        println!("assets are up to date ({} files)", ASSETS.len());
    } else {
        fs::write(&manifest_path, manifest)?;
        println!("generated {} PNGs and assets/manifest.sha256", ASSETS.len());
    }

    Ok(())
}

fn render(source_dir: &Path, asset: &Asset) -> Result<Vec<u8>, Box<dyn Error>> {
    let source_path = source_dir.join(asset.source);
    let data =
        fs::read(&source_path).map_err(|error| format!("{}: {error}", source_path.display()))?;
    let tree = resvg::usvg::Tree::from_data(&data, &resvg::usvg::Options::default())
        .map_err(|error| format!("{}: {error}", source_path.display()))?;

    let mut pixmap = Pixmap::new(asset.width, asset.height)
        .ok_or_else(|| format!("invalid dimensions for {}", asset.id))?;
    let sx = asset.width as f32 / tree.size().width();
    let sy = asset.height as f32 / tree.size().height();
    let transform = tiny_skia::Transform::from_scale(sx, sy);
    resvg::render(&tree, transform, &mut pixmap.as_mut());

    if asset.opaque
        && pixmap
            .data()
            .as_chunks::<4>()
            .0
            .iter()
            .any(|pixel| pixel[3] != 255)
    {
        return Err(format!("{} was expected to be opaque", asset.id).into());
    }

    let encoded = pixmap.encode_png()?;
    let mut options = oxipng::Options::from_preset(3);
    options.strip = oxipng::StripChunks::All;
    options.force = true;
    let optimized = oxipng::optimize_from_memory(&encoded, &options)?;

    Ok(optimized)
}

fn sha256(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}
