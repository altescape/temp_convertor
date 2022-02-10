use dialoguer::{theme::ColorfulTheme, Select, Input};

fn main() {
    print!("{esc}c", esc = 27 as char);

    let temp: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Input a temperature to convert:")
        .interact_text()
        .unwrap();

    print!("{esc}c", esc = 27 as char);

    let temp: f32 = temp.trim().parse().expect("A number is required");

    let temp_types = &["Fahrenheit to Celsius", "Celsius to Fahrenheit"];

    let temp_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Convert:")
        .default(0)
        .items(&temp_types[..])
        .interact()
        .unwrap();

    match temp_types[temp_type] {
        "Celsius to Fahrenheit" => println!("Temperature is {:.1}°F", celsius_to_fahrenheit(temp)),
        "Fahrenheit to Celsius" => println!("Temperature is {:.1}°C", fahrenheit_to_celsius(temp)),
        _ => println!("Temperature type not recognised"),
    }
}

fn celsius_to_fahrenheit(temp: f32) -> f32 {
    (temp * 1.8) + 32.0
}

fn fahrenheit_to_celsius(temp: f32) -> f32 {
    ((temp - 32.0) * 5.0) / 9.0
}
