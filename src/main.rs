#[macro_use]
extern crate clap;
use clap::App;
fn main() {
    let yml = load_yaml!("cli.yml");
    let m = App::from_yaml(yml).get_matches();
    
    match m.subcommand(){
        ("run",Some(sub_m)) => println!("Run"),
        (&_,_) => println!("invalid command")
    }
}


