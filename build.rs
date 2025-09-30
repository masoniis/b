use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    // Conditions for when to rerun the build step
    println!("cargo:rerun-if-changed=assets/textures");

    // Sets output to the rust outdir in target
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("texture_ids.rs");
    let mut f = BufWriter::new(File::create(&dest_path).unwrap());

    // INFO: -----------------------------------------
    //         Generating the "TextureId" Enum
    // -----------------------------------------------

    writeln!(
        &mut f,
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize)]"
    )
    .unwrap();
    writeln!(&mut f, "#[serde(rename_all = \"snake_case\")]").unwrap();
    writeln!(&mut f, "pub enum TextureId {{").unwrap();

    writeln!(&mut f, "  Missing,").unwrap();

    // This will store tuples of (PascalCaseVariant, snake_case_filename)
    let mut texture_info = Vec::new();

    // Generates the rest of the enum variants by reading from the textures folder
    for entry in glob::glob("assets/textures/*.png").expect("Failed to read glob pattern") {
        if let Ok(path) = entry {
            let name = path.file_stem().unwrap().to_str().unwrap().to_string();

            // 2. Skip any file named `_missing.png` to avoid duplicates.
            if name == "_missing" {
                continue;
            }

            let enum_variant = to_pascal_case(&name);

            writeln!(&mut f, "  {},", &enum_variant).unwrap();
            texture_info.push((enum_variant, name));
        }
    }
    writeln!(&mut f, "}}").unwrap();

    // Generate the `name()` method to get the filename back from the enum
    writeln!(&mut f, "\nimpl TextureId {{").unwrap();
    writeln!(&mut f, "  pub fn name(&self) -> &'static str {{").unwrap();
    writeln!(&mut f, "    match self {{").unwrap();
    writeln!(&mut f, "      Self::Missing => \"_missing\",").unwrap(); // missing has no file name
    for (variant, name) in &texture_info {
        writeln!(&mut f, "      Self::{} => \"{}\",", variant, name).unwrap();
    }
    writeln!(&mut f, "    }}").unwrap();
    writeln!(&mut f, "  }}").unwrap();

    // INFO: -------------------------------------------------------------------
    //         Generate a constant array that includes all enum variants
    // -------------------------------------------------------------------------

    writeln!(&mut f, "  pub const ALL: &'static [TextureId] = &[").unwrap();
    writeln!(&mut f, "    Self::Missing,").unwrap(); // Add Missing variant to the array
    for (variant, _) in &texture_info {
        writeln!(&mut f, "    Self::{},", variant).unwrap();
    }
    writeln!(&mut f, "  ];").unwrap();

    writeln!(&mut f, "}}").unwrap();
}

/// Converts snake_case to PascalCase (unchanged)
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
