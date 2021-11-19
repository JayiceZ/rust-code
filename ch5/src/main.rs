struct Student {
    name: String,
    age:u8,
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
}

fn build_student(name:String)->Student{
    Student{
        name,
        age:21
    }
}