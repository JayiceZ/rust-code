use std::net::IpAddr;
#[derive(Debug)]
enum IPAddr1 {
    V4,
    V6,
}

fn main() {
    let ip4 = IPAddr1::V4;
    match ip4 {
        IPAddr1::V4 => println!("4"),
        IPAddr1::V6 => println!("6"),
    };
    match ip4 {
        IPAddr1::V4 => println!("4"),
        _ => println!("6"),
    };

    let x=Some(5);
    let y:Option<u32>=None;
    match x {
        Some(n)=>println!("{}",n),
        None=>println!("null"),
    }
    match y {
        None=>println!("null"),
        Some(n)=>println!("{}",n),
    }
    let ip6 = IPAddr1::V6;
    println!("{:?}",ip6);
    println!("{:?}",ip6);
    func(&ip6);
    println!("{:?}",ip6);


    let x=Some(3);
    if let Some (3)=x{
        println!("success");
    }
    println!("{:?}",x);
}

fn func(e:&IPAddr1){

}