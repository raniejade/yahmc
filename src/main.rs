extern crate engine;
extern crate simple_logger;

use std::env;

fn main() {
    simple_logger::init().unwrap();
    
    let exe = env::current_exe().unwrap();
    let root = exe.parent().unwrap();
    let runner = engine::Builder::create(root.to_path_buf())
        .build();
    let result = runner.run();
    match result {
        Err(e) => println!("{}", e),
        _ => ()
    }
}
