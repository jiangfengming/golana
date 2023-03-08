use crate::idl;
use anyhow::{anyhow, Context, Result};
use borsh::ser::BorshSerialize;
use golana;
use goscript_engine as gos;
use std::io::Write;
use std::path::Path;

pub fn build(out_name: &str, out_dir: &Path) -> Result<()> {
    let reader =
        gos::run_fs::FsReader::new(Some(Path::new("./")), Some(Path::new("../../lib/")), None);
    let engine = gos::Engine::new();
    let (bc, _) = engine
        .compile(false, false, &reader, Path::new("./main.go"))
        .map_err(|el| {
            el.sort();
            anyhow!(el.to_string())
        })
        .context("compile error")?;

    let tx_meta =
        golana::check(&bc).map_err(|e| anyhow::Error::new(e).context("type check error"))?;

    // Generate idl
    let idl = idl::get_idl(&tx_meta, &bc.objects.metas)?;
    let idl_str = serde_json::to_string_pretty(&idl)
        .map_err(|e| anyhow::Error::new(e).context("serialize idl error"))?;

    let buf = bc.try_to_vec().unwrap();

    std::fs::create_dir_all(out_dir)?;
    write_file(
        &format!("{}_idl.json", out_name),
        out_dir,
        idl_str.as_bytes(),
    )
    .map_err(|e| anyhow::Error::new(e).context("write idl error"))?;
    write_file(&format!("{}.gosb", out_name), out_dir, &buf)
        .map_err(|e| anyhow::Error::new(e).context("write gosb error"))
}

fn write_file(out_name: &str, out_dir: &Path, data: &[u8]) -> std::io::Result<()> {
    let mut full_name = out_dir.to_owned();
    full_name.push(out_name);

    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(full_name)?;
    f.write_all(data)?;
    f.flush()
}
