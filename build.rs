use clap::{Command, CommandFactory};
use clap_complete::Shell;
use std::fs::File;
use std::path::Path;

include!("src/cli.rs");
const APP_NAME: &str = "unicorn"; // application name.
fn generate(s: Shell, app: &mut Command, outdir: &Path, file: &str) {
    let destfile = outdir.join(file);
    println!("dest: {}", destfile.display());
    std::fs::create_dir_all(destfile.parent().unwrap()).unwrap();
    let mut dest = File::create(destfile).unwrap();
    clap_complete::generate(s, app, APP_NAME, &mut dest);
}

fn main() {
    let mut app = CliOpts::command();
    app.set_bin_name(APP_NAME);

    let outdir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("target/completions/");
    generate(Shell::Bash, &mut app, &outdir, "bash/unicorn");
    generate(Shell::Elvish, &mut app, &outdir, "elvish/unicorn");
    generate(Shell::Fish, &mut app, &outdir, "fish/unicorn");
    generate(Shell::PowerShell, &mut app, &outdir, "powershell/unicorn");
    generate(Shell::Zsh, &mut app, &outdir, "zsh/_unicorn")
}
