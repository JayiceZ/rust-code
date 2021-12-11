struct A {
    i: i32,
}

impl A {
    fn call(&self) {
        println!("{:?}", self.i);
    }
}

fn main() {
    let mut a = Box::new(A { i: 1 });
    // a.call();

    println!("{:p}",a);


    {
        let mut b = &mut a;
        println!("{:p}",b);
        println!("{:p}",*b);
        println!();
        // b.call();

        **b=A{i:6};
        println!("{:p}",b);
        println!("{:p}",*b);
        println!();

        *b = Box::new(A { i: 2 });
        println!("{:p}",b);
        println!("{:p}",*b);
        println!();
        /*b.call();
        a.call();*/

        let mut c = Box::new(A { i: 3 });
        b = &mut c;
        /*b.call();
        a.call();*/
    }
    println!("{:p}",a);
    *a=A{i:4};
    println!("{:p}",a);
    // a.call();
}