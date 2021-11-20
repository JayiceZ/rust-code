fn main() {
    let arr: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let mut largest = arr.get(0).unwrap();
    for a in arr.iter() {
        if a > largest {
            largest = a;
        }
    }
    println!("{}", largest);

    println!("{:?}", arr);
    let mut largest = find_max(&arr);
    println!("{}", largest);

    largest = 10;
    println!("{:?}", arr);


    let largest = find_max_trait(&arr);
    println!("{}", largest);

    let r:&str;
    let a="123";
    {
        let b="4567";
        r=get_longest(a,b);
    }
    println!("{}",r);

    let r:&str;
    let a=String::from("123");
    {
        let b=String::from("4567");
        r=get_longest(a.as_str(),b.as_str());
    }
    println!("{}",r);
}

fn get_longest<'a,'b>(a:& 'a str,b:&'b str)->&'a str{
    if a.len()>=b.len() {
        a
    }else {
        a
    }
}

fn get_longest_string<'a>(a:&'a String,b:&'a String)->&'a String{
    if a.len()>=b.len() {
        a
    }else {
        b
    }
}

fn find_max(arr: &[i32]) -> i32 {
    let mut largest = arr.get(0).unwrap();
    for a in arr.iter() {
        if a > largest {
            largest = a;
        }
    }
    *largest
}

fn find_max_trait<T:PartialOrd+Copy>(arr: &[T]) -> T {
    let mut largest = &arr[0];
    for a in arr.iter() {
        if a > largest {
            largest = a;
        }
    }
    *largest
}

pub trait Summary {
    fn summarize(&self) -> String {
        // default imple
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn func1(a: impl Summary) {}

fn func2<T: Summary>(t: T) {}

fn func3<T>(t: T) -> i32 where T: Summary {
    32
}

