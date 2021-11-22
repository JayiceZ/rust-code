#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_fails() {
        assert_eq!(1 + 1, 3);
    }

    #[test]
    #[should_panic]
    fn should_panic() {
        println!("print sth");
        panic!("panic!!!!!");
    }
    #[test]
    #[ignore]
    fn should_ignore() {
    }

}

pub fn add_two() -> i32 {
    4
}
