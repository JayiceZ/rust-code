use std::cell::{RefCell, Cell, Ref};
use std::collections::HashMap;
use std::borrow::{BorrowMut, Borrow};

fn main(){
    let mut c:Cell<HashMap<i32,i32>>=Cell::new(HashMap::new());
    let a=c.borrow_mut();
    let b=c.borrow_mut();

    let c:RefCell<HashMap<i32,i32>>=RefCell::new(HashMap::new());
    {
        let mut a = c.borrow_mut();
        a.insert(1, 21);
    }

    {
        let a=c.borrow();
        println!("{}",a.len());
    }

    struct Student{
        cell:Cell<HashMap<i32,i32>>,
    }
    let student=Student{cell:Cell::new(HashMap::new())};
    let mut map=HashMap::new();
    map.insert(1,1,);
    student.cell.set(map);

    let mut map=student.cell.take();
    map.insert(2,2);

    let mut map2=student.cell.take();
    map2.insert(3,3);
    println!("{:?}",map);        //{1: 1, 2: 2}
    println!("{:?}",map2);        //{3: 3}




    let a=RefCell::new(HashMap::new());
    a.borrow_mut().insert(1,1);
    a.borrow_mut().insert(2,2,);
    let r:Ref<HashMap<_,_>>=a.borrow();
    println!("{}",*r.get(&1).unwrap());

}