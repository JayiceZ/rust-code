trait Animal{


    fn shout(&self);

    fn moving(&self){
        println!("all animal will move");
    }


}

struct People{}


trait Item{
    type Class;
    fn return_value()->Box<Self::Class>;
}

impl Animal for People{
    fn shout(&self) {
        println!("human shouting");
    }

    fn moving(&self) {
        println!("running");
    }
}

impl  Item for People {
    type Class = i32;

    fn return_value() -> Box<Self::Class> {
        Box::new(5)
    }
}


fn func<'a>(a:&'a dyn Animal){
    a.moving();
}

fn main(){

    let p=People{};
    p.moving();

    func(&p);

    People::return_value();
}