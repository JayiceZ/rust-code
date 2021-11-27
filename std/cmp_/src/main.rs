use std::cmp::Ordering;

fn main() {
    let s = String::new();
    let c = s.clone();

    #[derive(Eq)]
    struct Student {
        a: i32,
    }
    let s = Student { a: 1 };
    let c = Student { a: 1 };
    // compile error
    //println!("{}",s==c);

    impl PartialEq for Student {
        fn eq(&self, other: &Self) -> bool {
            self.a == other.a
        }
    }

    impl Ord for Student {
        fn cmp(&self, other: &Self) -> Ordering {
            self.a.cmp(&other.a)
        }
    }

    impl PartialOrd for Student {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    //ok
    println!("{}", s == c);

    let a = 1;
    let b = 2;
}
