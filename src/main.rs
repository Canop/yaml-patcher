mod args;
mod patch;

use {
    anyhow::*,
    crate::patch::*,
    yaml_rust::*,
    std::{
        fs,
        path::Path,
    },
};

fn parse(path: &Path) -> Result<Vec<Yaml>> {
    let s = fs::read_to_string(path)?;
    Ok(
        YamlLoader::load_from_str(&s)?
    )
}

fn main() -> Result<()> {
    let args: args::Args = argh::from_env();
    if args.version {
        println!("SafeCloset {}", env!("CARGO_PKG_VERSION"),);
        return Ok(());
    }
    let mut base = parse(&args.base)?;
    if base.len() > 1 {
        bail!("unexpected length of base");
    }
    let mut base = base.drain(..).next().unwrap();
    let patch = parse(&args.patch)?;
    apply_patch(&mut base, patch)?;
    let mut out = String::new();
    let mut writer = YamlEmitter::new(&mut out);
    writer.dump(&base)?;
    println!("{}", out);
    Ok(())
}
