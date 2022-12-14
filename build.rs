//! This script auto-generates a man page from the clap configuration.
//! It creates the `cizrna.1` file in the current directory, which is
//! ignored by git.
//!
//! The code comes from the sample at <https://rust-cli.github.io/book/in-depth/docs.html>.

use clap::CommandFactory;

// We're reusing the module just for the Cli struct. Ignore the rest of the code
// and don't report it as "never used" in this build script.
#[allow(dead_code)]
#[path = "src/cli.rs"]
mod cli;
use cli::Cli;

fn main() -> std::io::Result<()> {
    let cmd: clap::Command = Cli::command();

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write("cizrna.1", buffer)?;

    Ok(())
}
