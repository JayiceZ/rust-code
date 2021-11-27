trait Animal{
    fn hello(&self);

}
struct A{}
struct B{}

impl Animal for A{
    fn hello(&self) {
        println!("A")
    }
}

impl Animal for B {
    fn hello(&self) {
        println!("B");
    }
}



//func 和func2有相同的效果
fn func(a:impl Animal){

}
fn func2<T:Animal>(a:T){

}
fn func3(a: Box<dyn Animal>){
}

//类似于cpp，也有virtual_ptr
fn new1(i:i32)-> Box<dyn Animal> {
    if i<10 {
        Box::new(A{})
    }else {
        Box::new(B{})
    }
}

fn main(){
    func(A{});
    func(B{});
    func2(A{});
    func2(B{});
}