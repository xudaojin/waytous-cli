extern crate colored;

use crate::waytous::Waytous;


mod waytous;
mod subcommand_define;
mod cmd;
mod common;

fn main() {
    let app = Waytous::new();
    app.run();
}
