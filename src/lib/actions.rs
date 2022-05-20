use crate::{
    lib::calculator::calculator_imp, lib::command_structure,
    lib::garbage_cleaner::GarbageCleaner, lib::util, lib::no_sql_db
};


pub fn read_actions(args: Vec<String>, is_debug: bool) {
    let mut text: String = String::new();
    let mut args_option: String = String::new();

    if is_debug {
        const BASE_STRUCTURE_MIN_LENGHT: usize = 5;

        if args.len() >= BASE_STRUCTURE_MIN_LENGHT {
            args_option = args[4].clone();
        } else {
            println!("INCORRECT PATH LOCATION OR NOT FOUND, TRY IT : yal -debugMode -read <enter_path_here>\n\n");
        }
    } else {
        const BASE_STRUCTURE_MIN_LENGHT: usize = 3;

        if args.len() >= BASE_STRUCTURE_MIN_LENGHT {
            args_option = args[2].clone();
        } else {
            println!(
                "INCORRECT PATH LOCATION OR NOT FOUND, TRY IT : yal -read <enter_path_here>\n\n"
            );
        }
    }

    match no_sql_db::read_text(args_option) {
        Ok(t) => text = t,
        Err(err) => println!(
            "INCORRECT PATH LOCATION OR NOT FOUND, TRY IT : yal -read <enter_path_here>\n\n{err}"
        ),
    }

    if args.contains(&command_structure::NoSqlDb::_Read.to_string()) {
        util::printer_string(text.clone());
    }

    if args.contains(&command_structure::NoSqlDb::_Search.to_string()) {
        let mut querry: Vec<String> = args.clone();

        if is_debug {
            querry.drain(0..5);
        } else {
            querry.drain(0..4);
        }

        match no_sql_db::search_in_text(&text, &querry) {
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
}

pub fn calculator_actions() {
    calculator_imp();
}

pub fn garbage_cleaner(args: Vec<String>) {
    if args.len() == 3 {
        let mut user = String::new();
        for i in 0..args.len() {
            if args[i].eq(&command_structure::GarbageCleanerCommands::_Garbage.to_string()) {
                user = args[i + 1].clone();
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
    println!("yal -read {{enter_file_path_here}}    <- USE FOR READ JSON AND TXT FILES");
    println!("yal -read <enter_file_path_here> -search <enter_key_words_here>   <- FOR READ THE FILE THEN SEARCH KEYWORDS IN FILES");
    println!("yal -dntread <enter_file_path_here> -search <enter_key_words_here>    <- FOR ONLY SEARCH THE KEYWORDS IN FILES");
    println!("yal -math     <- BASIC MATH CALCULATOR");
    println!("yal -garbage <windows_username>   <- CLEAR WINDOWS TEMPORARY FILES");
    println!("yal -db -create <enter_collection_path_here>  <- [BETA] COLLECTION CREATION IMPLEMENTATION FOR FUTURE NOSQL DATABASE");
    println!("yal -db -delete <enter_collection_path_here>  <- [BETA] COLLECTION DELETION IMPLEMENTATION FOR FUTURE NOSQL DATABASE");
}

pub fn yal_version() {
    println!("yal version 1.2.2");
}

pub fn create_db(args: Vec<String>) {
    
    const BASE_STRUCTURE_MIN_LENGHT: usize = 4;

    if args.len() >= BASE_STRUCTURE_MIN_LENGHT {
        
        let path = args[3].to_owned();

        match no_sql_db::create_db(&path) {
            Ok(msg) => println!("{}", msg),
            Err(err) => println!("ERROR CREATING COLLECTION: {}   TRY YAL -HELP FOR MORE INFORMATION", err),
        }

    } else {
        println!(
            "ERROR CREATING COLLECTION TRY YAL -HELP FOR MORE INFORMATION"
        );
    }
}

pub fn delete_db(args: Vec<String>) {

    const BASE_STRUCTURE_MIN_LENGHT: usize = 4;

    if args.len() >= BASE_STRUCTURE_MIN_LENGHT {
        let path = args[3].to_owned();

        match no_sql_db::delete_db(&path) {
            Ok(msg) => println!("{}", msg),
            Err(err) => println!("ERROR DELETING COLLECTION: {}   TRY YAL -HELP FOR MORE INFORMATION", err),
        }

    } else {
        println!(
            "ERROR DELETING COLLECTION TRY YAL -HELP FOR MORE INFORMATION"
        );
    }
}