use dialoguer::{theme::ColorfulTheme, Select, Input};

fn main() {
    clearscreen::clear().unwrap();

    let temp: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Input a temperature to convert:")
        .interact_text()
        .unwrap();

    clearscreen::clear().unwrap();

    let temp: f32 = temp.trim().parse().expect("A number is required");

    let temp_types = &["Fahrenheit", "Celsius"];

    let temp_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Convert temperature {}° to...", temp))
        .default(0)
        .items(&temp_types[..])
        .interact()
        .unwrap();

    clearscreen::clear().unwrap();

    match temp_types[temp_type] {
        "Fahrenheit" => println!("Temperature is {:.1}°F", celsius_to_fahrenheit(temp)),
        "Celsius" => println!("Temperature is {:.1}°C", fahrenheit_to_celsius(temp)),
        _ => println!("Temperature type not recognised"),
    }
}

fn celsius_to_fahrenheit(temp: f32) -> f32 {
    (temp * 1.8) + 32.0
}

fn fahrenheit_to_celsius(temp: f32) -> f32 {
    ((temp - 32.0) * 5.0) / 9.0
}
