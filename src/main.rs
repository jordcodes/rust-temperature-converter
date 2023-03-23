use std::io;
fn main() {
    let mut inputted_temperature = String::new();
    println!("Enter temperature: ");

    io::stdin()
        .read_line(&mut inputted_temperature)
        .expect("Failed to read line");

    let mut inputted_temperature_unit = String::new();
    println!("Enter temperature unit: ");

    io::stdin()
        .read_line(&mut inputted_temperature_unit)
        .expect("Failed to read line");

    convert_temperature(
        inputted_temperature.trim().parse::<f32>().unwrap(),
        inputted_temperature_unit.trim().parse::<String>().unwrap(),
    );
}

fn convert_to_celsius(temperature: f32) {
    let converted_temperature = (temperature - 32.0) * 5.0 / 9.0;
    println!("{temperature} Fahrenheit in Celsius: {converted_temperature}");
}

fn convert_to_fahrenheit(temperature: f32) {
    let converted_temperature = temperature * 9.0 / 5.0 + 32.0;
    println!("{temperature} Celsius in Fahrenheit: {converted_temperature}");
}

fn convert_temperature(temperature: f32, unit: String) {
    match unit.as_str() {
        "C" => convert_to_fahrenheit(temperature),
        "F" => convert_to_celsius(temperature),
        _ => panic!("Invalid temperature unit"),
    }
}
