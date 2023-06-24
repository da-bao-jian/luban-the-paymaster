mod bindings;
mod utils;
mod bundler;

use env_logger::Env;
fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();

    // generate bindings
    // utils::generate_bindings().unwrap();
}
