struct Count{
    count:i32,
}

impl Count {
    fn new()->Count{
        Count{
            count:0,
        }
    }
}

impl Iterator for Count {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count+=1;
        if self.count<=6 {
            Some(self.count)
        }else {
            None
        }
    }
}

fn main(){
    let func=|num:i32|{
        println!("{}",num);
    };
    func(12);
    func(123);

    let s=String::from("str");
    let func2=move |str:&str|{
        println!("{}",str);
        println!("{}",s);
    };

    func2("str");
    //println!("{}",s);



    let vec=vec![1,2,3];
    let mut ite=vec.iter();
    assert_eq!(ite.next(),Some(&1));
    assert_eq!(ite.next(),Some(&2));
    assert_eq!(ite.next(),Some(&3));

    let v1=vec![1,2,3];
    let v2:Vec<_>=v1.iter().map(|x|x+1).collect();
    assert_eq!(v2, vec![2, 3, 4]);



    let mut counter=Count::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), Some(6));
    assert_eq!(counter.next(), None);
}