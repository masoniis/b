use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use wesl::Wesl;

fn main() {
    // INFO: -------------------------------------------
    //         Task 1: Generating TextureId Enum
    // -------------------------------------------------

    println!("cargo:rerun-if-changed=assets/textures"); // run condition

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("texture_ids.rs");
    let mut f = BufWriter::new(File::create(&dest_path).unwrap());

    // Writing the code for the enum into the build file
    writeln!(
        &mut f,
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize)]"
    )
    .unwrap();
    writeln!(&mut f, "#[serde(rename_all = \"snake_case\")]").unwrap();
    writeln!(&mut f, "pub enum TextureId {{").unwrap();
    let mut texture_info = Vec::new();
    for entry in glob::glob("assets/textures/*.png").expect("Failed to read glob pattern") {
        if let Ok(path) = entry {
            let name = path.file_stem().unwrap().to_str().unwrap().to_string();
            if name == "_missing" {
                continue;
            }
            let enum_variant = to_pascal_case(&name);
            writeln!(&mut f, "  {},", &enum_variant).unwrap();
            texture_info.push((enum_variant, name));
        }
    }
    writeln!(&mut f, "  #[serde(other)]").unwrap();
    writeln!(&mut f, "  Missing,").unwrap();
    writeln!(&mut f, "}}").unwrap();

    writeln!(&mut f, "\nimpl TextureId {{").unwrap();
    writeln!(&mut f, "  pub fn name(&self) -> &'static str {{").unwrap();
    writeln!(&mut f, "    match self {{").unwrap();
    writeln!(&mut f, "      Self::Missing => \"_missing\",").unwrap();
    for (variant, name) in &texture_info {
        writeln!(&mut f, "      Self::{} => \"{}\",", variant, name).unwrap();
    }
    writeln!(&mut f, "    }}").unwrap();
    writeln!(&mut f, "  }}").unwrap();
    writeln!(&mut f, "  pub const ALL: &'static [TextureId] = &[").unwrap();
    writeln!(&mut f, "    Self::Missing,").unwrap();
    for (variant, _) in &texture_info {
        writeln!(&mut f, "    Self::{},", variant).unwrap();
    }
    writeln!(&mut f, "  ];").unwrap();
    writeln!(&mut f, "}}").unwrap();

    // INFO: --------------------------------------
    //         Compile WESL shaders to WGSL
    // --------------------------------------------

    println!("cargo:rerun-if-changed=src/shaders"); // run condition

    let compiler = Wesl::new("assets/shaders"); // src dir for shaders

    // NOTE: Opaque shaders
    compiler.build_artifact(
        &"package::opaque::main_vert".parse().unwrap(),
        "opaque_vert",
    );
    compiler.build_artifact(
        &"package::opaque::main_frag".parse().unwrap(),
        "opaque_frag",
    );

    // NOTE: Transparent shaders
    compiler.build_artifact(
        &"package::transparent::main_vert".parse().unwrap(),
        "transparent_vert",
    );
    compiler.build_artifact(
        &"package::transparent::main_frag".parse().unwrap(),
        "transparent_frag",
    );

    // NOTE: Wireframe shaders
    compiler.build_artifact(
        &"package::wireframe::main_vert".parse().unwrap(),
        "wireframe_vert",
    );
    compiler.build_artifact(
        &"package::wireframe::main_frag".parse().unwrap(),
        "wireframe_frag",
    );

    // NOTE: UI shaders
    compiler.build_artifact(&"package::ui::main_vert".parse().unwrap(), "ui_vert");
    compiler.build_artifact(&"package::ui::main_frag".parse().unwrap(), "ui_frag");
}

/// Converts snake_case to PascalCase
fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect()
}
