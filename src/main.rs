mod command_structure;
mod config;
mod file_explorer;
mod util;
mod actions;
mod calculator;

use config::Config;
use std::process;

use crate::util::input_to_vector;

fn main() {
    let args: Vec<String> = util::input_to_vector();

    if args.contains(&command_structure::DebugCommannds::_DebugMode.to_string()) {
        if args.contains(&command_structure::DebugCommannds::_Loop.to_string()) {
            dev_mode(args.clone());
        } else {
            println!("for debug please use -debugMode and -loop");
        }
    }
}

fn dev_mode(mut args: Vec<String>) {
    const IS_DEBUG: bool = true;

    loop {
        if args.contains(&command_structure::DebugCommannds::_Exit.to_string()) {
            break;
        }

        let _configuration: Config = Config::new(&args.clone()).unwrap_or_else(|err| {
            println!("ERROR : {err}");
            process::exit(1);
        });

        action(args.clone(), IS_DEBUG);
        args = input_to_vector();
    }
}

fn action(args: Vec<String>, is_debug: bool) {
    if args.contains(&command_structure::FileSistemCommands::_Read.to_string())
        || args.contains(&command_structure::FileSistemCommands::_Dntread.to_string())
    {
        actions::read_actions(args.clone(), is_debug.clone());
    } else if args.contains(&command_structure::CalculatorCommands::_Math.to_string()) {
        actions::calculator_actions();
    } else {
        println!("SELECT A VALID OPTION, TRY yal -help FOR ALL COMANDS");
    }
}
