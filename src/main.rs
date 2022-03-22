use dialoguer::{theme::ColorfulTheme, Select, Input, Confirm};

fn main() {
    clearscreen::clear().unwrap();

    loop {
        let temp: f32 = prompt_temp_input();

        clearscreen::clear().unwrap();

        // Prompt select temperature type
        let temp_types = &["Fahrenheit", "Celsius"];
        let temp_type = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Convert temperature {}° to...", temp))
            .default(0)
            .items(&temp_types[..])
            .interact()
            .unwrap();
        prompt_select_type(&temp, &temp_types);

        clearscreen::clear().unwrap();

        // Output conversion
        let result = match temp_types[temp_type] {
            "Fahrenheit" => Temp::F(temp),
            "Celsius" => Temp::C(temp),
            _ => Temp::None(),
        };

        print_temp(&result);

        // Prompt continue or quit
        match Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Convert another?")
            .default(true)
            .wait_for_newline(true)
            .interact_opt()
            .unwrap()
            {
                Some(true) => {
                    clearscreen::clear().unwrap();
                    continue;
                },
                Some(false) => {
                    clearscreen::clear().unwrap();
                    break;
                },
                None => break,
            }
    }
}

enum Temp {
    F(f32),
    C(f32),
    None(),
}

fn prompt_temp_input() -> f32 {
    let temp: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Input a temperature to convert:")
        .interact_text()
        .unwrap();

    temp.trim().parse().expect("A number is required")
}

fn prompt_select_type(temp: &f32, temp_types: &[&str; 2]) {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Convert temperature {}° to...", temp))
        .default(0)
        .items(&temp_types[..])
        .interact()
        .unwrap();
}

fn convert_temp(temp: &Temp) -> f32 {
    match temp {
        &Temp::F(degrees) => (degrees * 1.8) + 32.0,
        &Temp::C(degrees) => ((degrees - 32.0) * 5.0) / 9.0,
        _ => 0.0
    }
}

fn print_temp(temp: &Temp) {
    match temp {
        &Temp::F(degrees) => println!("{:.0}°F = {:.0}°C", degrees, convert_temp(temp)),
        &Temp::C(degrees) => println!("{:.0}°C = {:.0}°F", degrees, convert_temp(temp)),
        _ => println!("Conversion not performed")
    }
}
