use crate::command_structure;

#[derive(Clone)]
pub struct Config {
    pub command: String,
    pub querry: Vec<String>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args[0].is_empty()
            || !args[0].eq(&command_structure::InicialComands::_Yal.to_string())
        {
            return Err(&"COMMAND NOT FOUND TYPE ( yal help ) FOR COMMAND LIST OR INFORMATIONS");
        }

        let command = args[0].clone();
        let mut querry: Vec<String> = Vec::new();

        for n in 1..args.len() {
            querry.push(args[n].clone());
        }

        return Ok(Config { command, querry });
    }
}