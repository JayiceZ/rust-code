use std::any::Any;
use std::fmt::Debug;

fn func<T:Any+Debug>(a:T){
    let a=&a as &dyn Any;
    match a.downcast_ref::<String>(){
        Some(s)=>{
            println!("{}",s);
        }
        None=>{
            println!("not string");
        }
    }

    match a.downcast_ref::<i32>(){
        Some(v)=>{
            println!("{}",v);
        }
        None=>{
            println!("not i32");
        }
    }
}


fn main(){
    func("abcsaca".to_string());
    func(123 as i32);
}