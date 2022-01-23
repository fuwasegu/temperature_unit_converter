use std::io;
use std::process;

fn main() {
    println!("input converte mode.");
    println!("1: F -> C ");
    println!("2: C -> F");

    // ほんとは 1 か 2 っていう型をつくりたい
    let mut mode = String::new();
    io::stdin().read_line(&mut mode)
        .expect("Faild to read line");

    let mode: i8 = match mode.trim().parse() {
        Result::Ok(num) => num,
        Result::Err(_) => {
            println!("Plese type number, 1 or 2");
            return;
        }
    };

    println!("input temperature.");
    let mut temperature = String::new();
    io::stdin().read_line(&mut temperature)
        .expect("Faild to read line");

    let temperature: f32 = match temperature.trim().parse() {
        Result::Ok(num) => num,
        Result::Err(_) => {
            println!("Plese type temperature as u32.");
            return;
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
