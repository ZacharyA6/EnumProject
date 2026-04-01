/******************************************************************************

Rust_3_ExtendedEnumBehavior

This program demonstrates that Rust enums can store different data inside each variant, 
such as a shape carrying a radius or an error carrying a message. It also shows how that 
data is accessed safely with pattern matching and what happens when an enum value is absent using Option.

*******************************************************************************/
#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Error(String),
}

impl Shape {
    fn describe(&self) {
        match self {
            Shape::Circle { radius } => {
                println!("Shape is a Circle with radius = {}", radius);
            }
            Shape::Rectangle { width, height } => {
                println!("Shape is a Rectangle with width = {} and height = {}", width, height);
            }
            Shape::Error(message) => {
                println!("Shape is an Error with message = \"{}\"", message);
            }
        }
    }

    fn area(&self) -> Option<f64> {
        match self {
            Shape::Circle { radius } => Some(std::f64::consts::PI * radius * radius),
            Shape::Rectangle { width, height } => Some(width * height),
            Shape::Error(_) => None,
        }
    }
}

fn main() {
    println!("Program 3: Extended Enum Behavior in Rust\n");

    // Each enum variant carries its own associated data
    let circle = Shape::Circle { radius: 3.0 };
    let rectangle = Shape::Rectangle {
        width: 4.0,
        height: 5.0,
    };
    let error = Shape::Error(String::from("Invalid dimensions"));

    // Retrieve and print associated data using methods + pattern matching
    circle.describe();
    rectangle.describe();
    error.describe();

    println!();

    // Use associated data in behavior (area calculation)
    match circle.area() {
        Some(value) => println!("Circle area = {:.2}", value),
        None => println!("Circle has no area"),
    }

    match rectangle.area() {
        Some(value) => println!("Rectangle area = {:.2}", value),
        None => println!("Rectangle has no area"),
    }

    match error.area() {
        Some(value) => println!("Error area = {:.2}", value),
        None => println!("Error variant does not have a valid area"),
    }
    

    println!("\nConclusion:");
    println!("- Rust enums natively support associated data for each variant.");
    println!("- Data is attached directly when constructing the variant.");
    println!("- Data is retrieved safely using pattern matching.");
    println!("- Rust does not use null for plain enums; absence is represented explicitly with Option<T>.");
    
    // accessing data on a null or unitialized enum variable (will invoke a panick)
    let maybe_shape: Option<Shape> = None;

    // Forcefully unwrap a missing value (Rust's "null-like" case)
    let shape = maybe_shape.unwrap();

    // This line will never be reached, No value exists for our example thus .unwrap() FAILS
    shape.describe();
}