use std::io;

pub fn printer_vector(content: Vec<String>) {
    println!("{:?}", content);
}

pub fn printer_string(content: String) {
    println!("{}", content);
}

pub fn printer_float(content: f32) {
    println!("{}", content);
}

pub fn input_to_vector() -> Vec<String> {
    
    println!("\nenter command: ");
    let input_command = input_to_string();
    let v: Vec<String> = input_command.split_whitespace().map(|str| str.to_string()).collect();

    return v;
}

pub fn input_to_string() -> String {

    let mut input_command = String::new();
    io::stdin()
        .read_line(&mut input_command)
        .expect("ERROR READING COMMAND TYPE (yal help) FOR MORE INFORMATION");

    return input_command.to_string();
}
