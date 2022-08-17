use std::env;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    #[serde(rename = "xc3")]
    game: GameInfo,
}

#[derive(Deserialize)]
struct GameInfo {
    version: String,
}

fn main() {
    println!("cargo:rerun-if-changed=build.toml");
    println!("cargo:rerun-if-changed=offsets/");
    println!("cargo:rerun-if-env-changed=XC3_VER");

    let config: Config =
        toml::from_str(&fs::read_to_string("build.toml").expect("couldn't read build config"))
            .expect("invalid TOML in build config");

    let offsets_file = format!("offsets/{}.toml", &config.game.version);
    let offsets_file = Path::new(&offsets_file);
    if !offsets_file.is_file() {
        panic!("Offsets file {:?} is not accessible", offsets_file);
    }

    let value: toml::Value =
        toml::from_str(&fs::read_to_string(offsets_file).expect("couldn't read file"))
            .expect("invalid TOML data");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    fs::create_dir_all(&out_dir).unwrap();

    let offsets_out_path = out_dir.join("offsets.bin");
    let writer = BufWriter::new(File::create(offsets_out_path).unwrap());
    ciborium::ser::into_writer(&value, writer).unwrap();
}
