use std::collections::{VecDeque, BTreeMap, BinaryHeap};
use std::option::Option::Some;

fn main(){
    // vec();
    // vec_deque();
    // btree_map();
    binary_heap();
}

fn vec(){
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], 1);

    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.len(), 1);

    vec[0] = 7;
    assert_eq!(vec[0], 7);

    vec.extend([1, 2, 3].iter().copied());

    for x in &vec {
        println!("{}", x);
    }
    assert_eq!(vec, [7, 1, 2, 3]);


    let mut c=Vec::with_capacity(5);
    c.resize(7,1);
    println!("{:?}",&c);

    let slice:&mut [i32]=& mut c[0..2];
    println!("{:?}",&slice);
    slice[0]=100;
    println!("{:?}",&c);

    //
    // let v=vec![1,2,3];
    // //manage mem by myself
    // let p=mem::ManuallyDrop::new(v);
    //


    let mut v=vec![1,2,3];
    let p=v.as_mut_ptr();

    unsafe {
        for i in 0..v.len() {
            *p.add(i) = i+100 as usize;
        }
    }
    println!("{:?}",v);

    let mut vec = vec![1, 2, 2, 3, 2];

    vec.dedup();

    assert_eq!(vec, [1, 2, 3, 2]);
}

fn vec_deque(){
    let mut vq:VecDeque<i32>=VecDeque::from(vec![1,2,3] );
    vq.push_front(0);
    vq.push_back(4);
    println!("{:?}",&vq);
    vq.pop_back();
    println!("{:?}",&vq);
    vq.pop_front();
    println!("{:?}",&vq);
}

fn btree_map(){
    let mut movie_reviews = BTreeMap::new();

    movie_reviews.insert("Office Space",       "Deals with real issues in the workplace.");
    movie_reviews.insert("Pulp Fiction",       "Masterpiece.");
    movie_reviews.insert("The Godfather",      "Very enjoyable.");
    movie_reviews.insert("The Blues Brothers", "Eye lyked it a lot.");

    if !movie_reviews.contains_key("Les Misérables") {
        println!("We've got {} reviews, but Les Misérables ain't one.",
                 movie_reviews.len());
    }

    movie_reviews.remove("The Blues Brothers");

    let to_find = ["Up!", "Office Space"];
    for movie in &to_find {
        match movie_reviews.get(movie) {
            Some(review) => println!("{}: {}", movie, review),
            None => println!("{} is unreviewed.", movie)
        }
    }

    println!("Movie review: {}", movie_reviews["Office Space"]);


    println!();
    for (movie, review) in &movie_reviews {
        println!("{}: \"{}\"", movie, review);
    }
}

fn binary_heap(){
    let mut bh=BinaryHeap::new();
    bh.push(1);
    bh.push(5);
    bh.push(2);

    println!("{}",bh.peek().unwrap());

    while let Some(val)=bh.pop(){
        println!("{}",val);
    }
}
