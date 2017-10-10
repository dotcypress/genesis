#[macro_use]
extern crate log;
extern crate clap;
extern crate env_logger;

use clap::App;

fn main() {
    env_logger::init();
    debug!("starting up");
    let matches = App::new("genesis")
        .version(env!("CARGO_PKG_VERSION"))
        .get_matches();
}
