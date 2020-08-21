/*
 * Structs -- used to create custom data types
*/

// Traditional Struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// Tuple Struct
struct Color(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Construct Person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: std::string::String::from(last),
        }
    }

    // Get Full Name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set Last Name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string()
    }

    // Set Name to Tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    // let mut c = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0,
    // };

    // c.blue = 150;

    // println!("Color: {} {} {}", c.red, c.green, c.blue);
    // Tuple Struct
    let mut c = Color(255, 0, 0);
    c.2 = 150;
    println!("Color: {} {} {}", c.0, c.1, c.2);

    // Complex Traditional Struct
    let mut p = Person::new("Evan", "Hutch");
    println!("Person: {}", p.full_name());
    p.set_last_name("Hutchinson");
    println!("Person: {}", p.full_name());
    println!("Person: {:#?}", p.to_tuple());
}
