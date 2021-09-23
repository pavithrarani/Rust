use std::fs::File;
use std::io::ErrorKind;
use std::io;
fn main()
{
    let f  = File::open("hello.txt");

    /*let f = match f{
        Ok(file) => file,
        Err(error) => panic!("problem opening file {:?}", error),
        };*/

    let f  = match f{
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file {:?}",e),
        },
        other_error =>{ 
            panic!("Problem opening the file {:?}",other_error)
        }
        },
    };
   //let f1 = File::open("hello1.txt").unwrap(); 
   let f2 = File::open("helloo1.txt").expect("failed to open");



}
