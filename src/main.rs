extern crate errors;
use errors::Error;


fn error_check(i: i32) -> Result<String,Error> {
    if i > 100_000 {
        Ok(String::from("Oh sh1t, here we go again..."))
    }else {
       //Err(Error::Message("error_check(): Error::Message(String) called...".to_string()))
        Err(Error::Custom(101,"Couldn't import data from database".to_string()))
    }
}

fn main() {
    let f = error_check(1000000).unwrap();
    println!("{f}");
}
