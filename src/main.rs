use crate::cli::Cli;

mod cli;
mod subcommand_define;
mod commands;
mod common;

fn main() {
    let app = Cli::new();
    app.run();
}
