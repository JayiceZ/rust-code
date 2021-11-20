use std::collections::HashMap;

fn main(){
    let mut vec:Vec<i32>=Vec::new();
    vec.push(123);

    let v2=vec![1,2,3];

    let third:&i32=v2.get(2).unwrap();

    println!("{}",third);





    let s1=String::from("hello ");
    let s2=String::from("world");
    //let s3=s1+s2;
    let s3=s1+&s2;  //s1 lost its ownership,but s2 didn't


    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];

    let mut iter = a1.iter().zip(a2.iter());

    assert_eq!(iter.next(), Some((&1, &4)));
    assert_eq!(iter.next(), Some((&2, &5)));
    assert_eq!(iter.next(), Some((&3, &6)));
    assert_eq!(iter.next(), None);

    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut map:HashMap<_,_>=teams.iter().zip(initial_scores.iter()).collect();
    let value=map.get(&String::from("Blue")).unwrap();
    println!("{}",value)

}

fn func() {
    use std::collections::HashMap;

    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
}
