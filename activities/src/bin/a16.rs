// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let student = Student {
        name: "Alice".to_string(),
        locker: None,
    };

    match student.locker {
        Some(locker) => println!("Student {} has locker {}", student.name, locker),
        None => println!("Student {} has no locker", student.name),
    }
}

