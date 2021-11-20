use std::fs::File;
use std::io::ErrorKind;

fn main(){
    //if the file doesn't exist,here will panic
    //let file=File::open("test.txt").unwrap();

    let file=match File::open("test.txt") {
        Ok(T)=>T,
        Err(_)=>{
            match File::create("test.txt") {
                Ok(T)=>T,
                Err(_)=>panic!("error"),
            }
        }
    };

    //or
    let file=File::open("tests.txt").unwrap_or_else(|error|{
        if error.kind()==ErrorKind::NotFound {
            File::create("tests.txt").unwrap_or_else(|err|{
                panic!("bad create,{}",err);
            })
        }else {
            panic!("bad open");
        }
    });
}