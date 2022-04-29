use crate::{
    lib::calculator::calculator_imp, lib::command_structure, lib::file_explorer,
    lib::garbage_cleaner::GarbageCleaner, lib::util,
};

pub fn read_actions(args: Vec<String>, is_debug: bool) {
    if args.contains(&command_structure::FileSistemCommands::_Path.to_string()) {
        let mut text: String = String::new();
        let mut args_option: String = String::new();

        if is_debug {
            if args.len() >= 6 {
                args_option = args[5].clone();
            } else {
                println!("INCORRECT PATH LOCATION OR NOT FOUND, TRY IT : yal -debugMode -loop -read -path <enter_path_here>\n\n");
            }
        } else {
            if args.len() >= 3 {
                args_option = args[3].clone();
            } else {
                println!("INCORRECT PATH LOCATION OR NOT FOUND, TRY IT : yal -read -path <enter_path_here>\n\n");
            }
        }

        match  file_explorer::read_text(args_option) {
            Ok(t) => text = t,
            Err(err) => println!("INCORRECT PATH LOCATION OR NOT FOUND, TRY IT : yal -read -path <enter_path_here>\n\n{err}"),
        }

        if args.contains(&command_structure::FileSistemCommands::_Read.to_string()) {
            util::printer_string(text.clone());
        }

        if args.contains(&command_structure::FileSistemCommands::_Search.to_string()) {
            let mut querry: Vec<String> = args.clone();

            if is_debug {
                querry.drain(0..4);
            } else {
                querry.drain(0..3);
            }

            match file_explorer::search_in_text(&text, &querry) {
                Ok(s) => {
                    let mut vec_result: Vec<String> = Vec::new();
                    for str in s {
                        vec_result.push(str.to_string());
                    }

                    util::printer_vector(vec_result);
                }
                Err(err) => println!("{err}"),
            }
        }
    } else {
        println!("INCORRECT, TRY: yal -read -path <filepath> <search?>");
    }
}

pub fn calculator_actions() {
    calculator_imp();
}

pub fn garbage_cleaner(args: Vec<String>) {
    if args.len() == 3 {

        let mut user = String::new();
        for i in 0..args.len() {
            if args[i].eq(&command_structure::GarbageCleanerCommands::_Garbage.to_string()) {
                user = args[i+1].clone();
            }
        }
        match GarbageCleaner::run(user) {
            Ok(ok) => println!("{}", ok),
            Err(err) => println!("{}", err),
        }
    } else {
        println!("INCORRECT, TRY: yal -garbage <username>");
    }
}

pub fn yal_help() {
    println!("\n\nYAL BY RODRIGO ZANCHETTA");
    yal_version();
    print!("\nCOMMANDS: \n");
    println!("yal -read -path <enter_file_path_here> -dntsearch");
    println!("yal -read -path <enter_file_path_here> -search <enter_key_words_here>");
    println!("yal -dntread -path <enter_file_path_here> -search <enter_key_words_here>");
    println!("yal -math");
    println!("yal -garbage <windows_username>");
}

pub fn yal_version() {
    println!("yal version 1.1.0");
}