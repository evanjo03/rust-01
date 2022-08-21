use std::io::stdin;

#[derive(Debug)]
enum ConversionType {
    Celcius,
    Fahrenheit,
}

fn print_opening_text() {
    println!("Welcome to the rust temperature converter!");
    println!("Enter 'C' to convert a temperature from Celcius to Fahrenheit, and enter and F to convert from Fahrenheit to Celcius.")
}

fn read_conversion_type() -> ConversionType {
    let res: ConversionType = loop {  
        print_opening_text();
        let mut conversion_type = String::new();
        stdin().read_line(&mut conversion_type).unwrap();
        let conversion_type: char = conversion_type.trim().parse().unwrap();
        let result: Result<ConversionType, &str> = match conversion_type {
            'C' => Ok(ConversionType::Celcius),
            'F' => Ok(ConversionType::Fahrenheit),
            _ => Err("Invalid conversion type")
        };
        if result.is_ok() {
            break result.unwrap()
        }

        println!("Invalid input provided!");
    };

    res
}

fn read_number() -> i32 {
    println!("Enter the number you wish to convert.");
    let mut number_input = String::new();
    stdin().read_line(&mut number_input).unwrap();
    let number_input: i32 = number_input.trim().parse().unwrap();
    number_input
}

fn convert_celsius_to_fahrenheit(number: i32) -> (f64, char) {
    let val: f64 = number.into();
    ((val * 1.8 + 32.0), 'F')
}

fn convert_fahrenheit_to_celsius(number: i32) -> (f64, char) {
    let val: f64 = number.into();
    ((val - 32.0) * 0.555555, 'C')
}

fn main() {
    let conversion_type = read_conversion_type();
    let number_to_convert: i32 = read_number();

    let (res, unit) = match conversion_type {
        ConversionType::Celcius => convert_celsius_to_fahrenheit(number_to_convert),
        ConversionType::Fahrenheit => convert_fahrenheit_to_celsius(number_to_convert)
    };

    println!("The converted value is {}{}", res, unit);
}