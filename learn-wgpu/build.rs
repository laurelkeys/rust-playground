use anyhow::*;
use fs_extra::{copy_items, dir::CopyOptions};

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=assets/*");

    copy_items(
        /* from */ &["assets/"],
        /* to */ std::env::var("OUT_DIR")?,
        &CopyOptions { overwrite: true, ..CopyOptions::default() },
    )?;

    Ok(())
}
