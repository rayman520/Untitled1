
use std::process;

pub mod events;
pub mod keyboard;


pub trait CheckResult<T, E>{
    fn check_result(self) -> T;
}

impl<T, E: ::std::fmt::Debug> CheckResult<T, E> for Result<T, E> {
    fn check_result(self) -> T {
        match self{
            Ok(val) => val,
            Err(msg) => {
                println!("Error: {:?}", msg);
                process::exit(1);
            }
        }
    }
}

pub fn exit(err: &str) {
    println!("{}", err);
    ::std::process::exit(1);
}