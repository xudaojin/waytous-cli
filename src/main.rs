use crate::cli::Cli;

mod cli;
mod commands;
mod common;
mod subcommand_define;


fn main() {
    let app = Cli::new();
    app.run();

}
