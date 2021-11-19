
fn main() {
    let mut s=String::new();
    s.push_str("1");
    //func1(s);
    //println!("{}",s);  error

    func2(&s);

    func3(&mut s);
    println!("{}",s);

}

#[warn(dead_code)]
fn func1(a:String){
    println!("{}",a);
}

fn func2(a:&String){
    println!("{}",a);
    //a.push_str("123");   immutable
}

fn func3(a:&mut String){
    println!("{}",a);
    a.push_str("123");
    println!("{}",a);
}
