use std::io;
use std::process;

fn main() {
    println!("input converte mode.");
    println!("1: F -> C ");
    println!("2: C -> F");

    let mode: i8 = match get_mode_from_read_line() {
        Result::Ok(num) => num,
        Result::Err(message) => {
            println!("{}", message);
            process::exit(0);
        }
    };

    println!("input temperature.");

    let temperature: f32 = match get_temperature_from_read_line() {
        Result::Ok(num) => num,
        Result::Err(message) => {
            println!("{}", message);
            process::exit(0);
        }
    };

    let converted: f32 = if mode == 1 {
        fahrenheit_to_celsius(temperature)
    } else if mode == 2 {
        celsius_to_fahrenheit(temperature)
    } else {
        println!("Undefined mode. Exit.");
        process::exit(0);
    };

    println!("Converted. {} -> {}", temperature, converted);
}

/**
 * 華氏温度 -> セ氏温度
 */
fn fahrenheit_to_celsius(temperature: f32) -> f32 {
    (temperature - 32 as f32) / 1.8
}

/**
 * セ氏温度 -> 華氏温度
 */
fn celsius_to_fahrenheit(temperature: f32) -> f32 {
    temperature * 1.8 + 32 as f32
}

fn get_mode_from_read_line() -> Result<i8, &'static str> {
    let mut mode = String::new();
    io::stdin().read_line(&mut mode)
        .expect("Faild to read line");

    match mode.trim().parse::<i8>() {
        Result::Ok(num) => Result::Ok(num),
        Result::Err(_) => Result::Err("Plese type number")
    }
}

fn get_temperature_from_read_line() -> Result<f32, &'static str> {
    let mut temperature = String::new();
    io::stdin().read_line(&mut temperature)
        .expect("Faild to read line");

    match temperature.trim().parse::<f32>() {
        Result::Ok(num) => Result::Ok(num),
        Result::Err(_) => Result::Err("Plese type temperature as u32.")
    }
}
