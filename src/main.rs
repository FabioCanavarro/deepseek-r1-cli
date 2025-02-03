#[macro_use]
extern crate clap;
use clap::App;
fn main() {
    let yml = load_yaml!("cli.yml");
    let m = App::from_yaml(yml).get_matches();
    
    println!("{}",m.value_of("argument1").unwrap());
}


