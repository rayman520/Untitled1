
pub mod events;

pub fn exit(err: &String) {
    println!("{}", err);
    ::std::process::exit(1)
}