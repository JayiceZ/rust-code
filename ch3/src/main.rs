use std::io;
fn main() {
    let x:(i32,u32,u8)=(12,321,21);
    let y=x.0;
    let z=x.1;
    let k=x.2;


    let arr=[1,2,3,4];
    let arr1=[2;6];
    let arr2:[i32;2]=[1,2];

    println!("{}",arr[0]);

    arr();


    let a= if index<2{
        3
    }else{
        6
    };
    println!("{}",a);
}

fn arr(){
    let arr = [1, 2, 3, 4];


    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("fail");
    let index: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("error");
            return;
        }
    };

    println!("{}",arr[index]);
}


//func

/**
123321
*/
fn func(i:i32,j:i32)->i32{
    i+j
}