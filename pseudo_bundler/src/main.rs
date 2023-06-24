mod utils;
// mod bindings;



use env_logger::Env;
fn main() {

    env_logger::Builder::from_env(Env::default()).init();
    
    // generate bindings
    // utils::generate_bindings().unwrap();


}
