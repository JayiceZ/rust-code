
fn main() {
    let mut s=String::new();
    s.push_str("1");
    //func1(s);
    //println!("{}",s);  error

    func2(&s);

    func3(&mut s);
    println!("{}",s);


    let index=func4(&s);
    println!("{}",index);

    func5(&s[0..2]);
    func5(&s);
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

fn func4(a:&String)->usize{
    let bytes=a.as_bytes();
    for  (i,&item)in bytes.iter().enumerate() {
        if item== b' '{
            return i;
        }
    }
    a.len()
}

fn func5(a:&str)->usize{
    let bytes=a.as_bytes();
    for  (i,&item)in bytes.iter().enumerate() {
        if item== b' '{
            return i;
        }
    }
    a.len()
}
