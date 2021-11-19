#[derive(Debug)]
struct Student {
    name: String,
    age:u8,
}

impl Student {
    fn check_myself(&self){
        println!("{:?}",self);
    }
    fn new(name:String)->Student{
        Student{
            name,
            age:21
        }
    }
}

fn main() {
    let student=Student{
        name:String::from("Jayice"),
        age:21,
    };
    let student2=build_student(String::from("zjw"));

    let student3=Student{
        name:student.name.clone(),
        age:21,
    };

    println!("{}",&student.name);
    println!("{}",&student2.name);

    println!("{:?}",student);

    check_student(&student2);


    let new_student=Student::new(String::from("new born jayice"));
    new_student.check_myself();
}

fn build_student(name:String)->Student{
    Student{
        name,
        age:21
    }
}

fn check_student(stu:&Student){
    println!("{:?}",stu);
}