use std::rc::Rc;
use std::cell::RefCell;


struct Cons{
    num:i32,
    next:Box<Option<Cons>>,
}

struct Student<'a>{
    name:&'a str,
    age:RefCell<i32>,
}

struct Class{
    age:i32,
}

impl Clone for Class{
    fn clone(&self) -> Self {
        Class{
            age:self.age,
        }
    }
}

fn main(){
    //let x=Cons{ num: 0, next: Some(Cons{ num: 0, next: None }) };
    let x=Cons{ num: 0, next: Box::new(Some(Cons{ num: 0, next: Box::new(None) })) };

    let x=Box::new(5);
    assert_eq!(5,*x);

    println!("{}",x);
    drop(x);
    //println!("{}",x);


    let p1=Rc::new(5);
    let p2=Rc::clone(&p1);
    println!("{}",Rc::strong_count(&p1));   //2

    //let p2=1;     //直接这样不能释放
    //println!("{}",Rc::strong_count(&p1));     //2

    drop(p2);
    println!("{}",Rc::strong_count(&p1));    //1

    let stu=Student{ name: "jayice", age: RefCell::new(1) };
    *stu.age.borrow_mut()=15;
    //*stu.age=12;
    //stu.name="!23";
    println!("{:?}",stu.age);

    let c=Box::new(Class{ age: 12 });
    let mut c1=Box::clone(&c);
    c1.age=30;
    println!("{}",c1.age);
    println!("{}",c.age);
}