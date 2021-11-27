use std::convert::TryFrom;

fn main(){
    let a=A{i:1};
    func(A{i:1});
    let num=a.as_ref();
    println!("{}",num);


    let a=A::from(2);
    let num:i32=a.into();
}

fn func(a:impl AsRef<i32>)-> i32{
    *a.as_ref()
}

#[derive(Debug)]
struct A{
    i:i32,
}

impl AsRef<i32> for A {
    fn as_ref(&self) -> &i32 {
        &10
    }
}

impl From<i32> for A {
    fn from(a: i32) -> Self {
        A{i:a}
    }
}

impl Into<i32> for A{
    fn into(self) -> i32 {
        self.i
    }
}


//和From<i32>不能同时实现。

impl TryFrom<i32> for A {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value<10 {
            Ok(A { i: value })
        }else {
            Err("123123")
        }
    }
}