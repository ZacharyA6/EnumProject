/******************************************************************************

Rust_1_EnumBasics

This program demonstrates basic enum declaration, assigning enum values to variables, 
and printing them. It also tests invalid enum behavior by assigning or casting an integer 
outside the defined range to show whether Rust catches the issue at compile time, runtime, or not at all.

Rust prevents arbitrary integer-to-enum casts in safe code at compile time. 
When a manual conversion is implemented, invalid values are typically caught at runtime 
as an error result rather than silently accepted.

*******************************************************************************/

use std::convert::TryFrom;

// A simple fieldless enum with explicit integer values (discriminants)
#[derive(Debug, Clone, Copy)]
enum TrafficLight {
    Red = 0,
    Yellow = 1,
    Green = 2,
}

// Implement safe conversion from i32 -> TrafficLight
impl TryFrom<i32> for TrafficLight {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TrafficLight::Red),
            1 => Ok(TrafficLight::Yellow),
            2 => Ok(TrafficLight::Green),
            _ => Err(format!("{} is not a valid TrafficLight value", value)),
        }
    }
}

fn main() {
    println!("Program 1: Enumeration Basics in Rust\n");

    // Basic enum declaration, assignment, and printing
    let current_light = TrafficLight::Green;
    println!("Current light: {:?}", current_light);

    // Enum -> integer conversion
    let numeric_value = current_light as i32;
    println!("TrafficLight::Green as integer = {}", numeric_value);

    // Valid integer -> enum conversion (safe)
    let valid_number = 1;
    match TrafficLight::try_from(valid_number) {
        Ok(light) => println!("Converted {} into enum value: {:?}", valid_number, light),
        Err(error_message) => println!("Error converting {}: {}", valid_number, error_message),
    }

    // Invalid integer -> enum conversion (safe, beyond normal usage)
    let invalid_number = 99;
    match TrafficLight::try_from(invalid_number) {
        Ok(light) => println!("Converted {} into enum value: {:?}", invalid_number, light),
        Err(error_message) => println!("Error converting {}: {}", invalid_number, error_message),
    }

    println!("\nConclusion:");
    println!("- Rust allows enum -> integer conversion with 'as' for fieldless enums.");
    println!("- Rust does NOT allow arbitrary integer -> enum conversion in safe code.");
    println!("- We used TryFrom<i32> to handle invalid values safely.");
    println!("- Invalid enum values are caught by our safe conversion logic at runtime as an error result.");
}