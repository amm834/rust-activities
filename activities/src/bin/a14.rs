// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    name: String,
    age: i32,
    favorite_colors: Vec<String>,
}

impl Person {
    fn new(name: String, age: i32, favorite_color: Vec<String>) -> Self {
        Self {
            name,
            age,
            favorite_colors: favorite_color,
        }
    }

    fn display_name(&self) {
        println!("Name: {}", self.name);
    }

    fn display_age(&self) {
        println!("Age: {}", self.age);
    }

    fn display_colors(&self) {
        for color in &self.favorite_colors {
            print!("{} ", color);
        }
    }
}


fn main() {
    let people = vec![
        Person::new("Alice".to_string(), 10, vec![String::from("red"), String::from("blue")]),
        Person::new("Bob".to_string(), 11, vec![String::from("green"), String::from("yellow")]),
        Person::new("Charlie".to_string(), 9, vec![String::from("purple"), String::from("orange")]),
        Person::new("Dave".to_string(), 12, vec![String::from("black"), String::from("white")]),
    ];

    for person in people {
        if person.age <= 10 {
            person.display_name();
            person.display_age();
            person.display_colors();
            println!("\n-----------------");
        }
    }
}
