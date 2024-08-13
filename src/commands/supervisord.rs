use crate::subcommand_define;
use crate::{common::common, print_debug_msg, print_tips_msg, print_info_msg};
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

use crate::subcommand_define::SupervisordCmds;

pub fn supervisord(sub_cmd: &subcommand_define::SupervisordCmds) {
    match sub_cmd {
        SupervisordCmds::Start { all, name } => {
            start(all, name).expect("TODO: panic message");
        }
        SupervisordCmds::Restart { all, name } => {}
        SupervisordCmds::Stop { all, name } => {}
        SupervisordCmds::List {  } => {}
    }
}

fn start(all: &bool, name: &str) -> PyResult<()> {
    println!("ddd");
    Python::with_gil(|py| {
        let sys = py.import_bound("sys")?;
        let version: String = sys.getattr("version")?.extract()?;

        let locals = [("os", py.import_bound("os")?)].into_py_dict_bound(py);
        let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
        let user: String = py.eval_bound(code, None, Some(&locals))?.extract()?;

        println!("Hello {}, I'm Python {}", user, version);
        Ok(())
    })
}

fn restart(all: &bool, name: &str) {

}

fn stop(all: &bool, name: &str) {

}

fn list() {

}