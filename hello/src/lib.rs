/* A library for Marco Polo*/

// Build a Marco Polo function that returns a string Polo if you parse in Marco or a string Marco if you parse in Polo otherwise it returns NOT Marco Polo.

pub fn marco_polo(input: &str) -> String {
    if input == "Marco" {
        "Polo".to_string()
    } else if input == "Polo" {
        "Marco".to_string()
    } else {
        "NOT Marco Polo".to_string()
    }
}
