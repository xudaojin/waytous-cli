use crate::waytous::Waytous;
extern crate colored;
mod waytous;
mod subcommand_define;
mod cmd;
mod common;

fn main() {
    let app = Waytous::new();
    app.run();
}
