use std::io;

use crate::util::printer_float;

pub fn calculator_imp() {
    println!("WELCOME TO SIMPLE CALCULATOR APP");

    loop {
        println!("\nWHICH OPERATION DO YOU WANT?");
        println!("1 <- sum\n2 <- subtract\n3 <- multiply\n4 <- divide\n99 <- for exit");

        let user_input: String = get_operation();

        if user_input.eq("1") {
            sum_operations(get_number());
        } else if user_input.eq("2") {
            sub_operations(get_number());
        } else if user_input.eq("3") {
            mul_operations(get_number());
        } else if user_input.eq("4") {
            div_operations(get_number());
        } else if user_input.eq("99") {
            println!("THANKS FOR USING ME");
            std::process::exit(0);
        } else {
            println!("INCORRECT OPTION, TRY AGAIN");
        }
    }
}

fn get_operation() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    return input.trim().to_string();
}

fn get_number() -> (f32, f32) {
    let mut input: String = String::new();

    println!("FIRST NUMBER");
    io::stdin()
        .read_line(&mut input)
        .expect("UNABLE TO READ USER NUMBER");

    input = input.trim().to_string();

    let first_value_f: f32 = input.parse::<f32>().unwrap();
    input = String::new();

    println!("SECOND NUMBER");
    io::stdin()
        .read_line(&mut input)
        .expect("UNABLE TO READ USER NUMBER");

    input = input.trim().to_string();
    let second_value: f32 = input.parse::<f32>().unwrap();
    return (first_value_f, second_value);
}



fn sum_operations(inputs: (f32, f32)) {
    printer_float(inputs.0 + inputs.1);
}

fn sub_operations(inputs: (f32, f32)) {
    printer_float(inputs.0 - inputs.1);
}

fn mul_operations(inputs: (f32, f32)) {
    printer_float(inputs.0 * inputs.1);
}

fn div_operations(inputs: (f32, f32)) {
    printer_float(inputs.0 / inputs.1);
}
