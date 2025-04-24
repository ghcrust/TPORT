use std::env;
use count::count_lines_in_path;
use anyhow::{Result, bail};

fn main() -> Result<()> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        bail!("Usage: count <files>...");
    }
    for path in args {
        let count = count_lines_in_path(&path)?;
        println!("{path}: {count} lines");
    }
    Ok(())
}
