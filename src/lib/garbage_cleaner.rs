use crate::util;
use std::fs;

#[derive(Clone)]
pub struct GarbageCleaner {}

impl GarbageCleaner {
    fn delete(path: String) -> Result<String, String> {
        match fs::remove_dir_all(path) {
            Ok(_) => return Ok("\nsucess".to_string()),
            Err(err) => return Err(err.to_string() + " ou existem arquivos que estão em uso atualmente"),
        };
    }

    fn create(path: String) -> Result<String, String> {
        match fs::create_dir(path) {
            Ok(_) => return Ok("\nsucess".to_string()),
            Err(err) => return Err(err.to_string() + " ou pasta não foi completamente esvaziada"),
        };
    }

    pub fn run(mut _user: String) -> Result<String, String> {
        _user = _user.trim().to_string();
        let mut status: String = " ".to_string();

        let user_temp_path: String = "C:\\Users\\".to_owned() + &_user + &"\\AppData\\Local\\Temp".to_string();
        let win_temp_path: String = "C:\\Windows\\Temp".to_string();
        let sis_temp_path: String = "C:\\Temp".to_string();

        println!("TEM CERTEZA QUE DESEJA DELETAR OS ARQUIVOS? SIM/NÃO");
        let mut option = util::input_to_string();

        option = option.to_uppercase().trim().to_string();
        println!("{}", option);

        if option.eq(&"SIM".trim().to_string()) {
            match GarbageCleaner::delete(user_temp_path.to_string()) {
                Ok(_) => {
                    status += "\nsucess deleting in the PATH";
                    status += " ";
                    status += &user_temp_path;
                    status += " ";
                }
                Err(err) => {
                    status += " \nfailed tryng to delete in  the PATH";
                    status += " ";
                    status += &user_temp_path;
                    status += " ";
                    status += &err;
                }
            };

            match GarbageCleaner::create(user_temp_path.to_string()) {
                Ok(_) => {
                    status += "\nsucess creating in the PATH";
                    status += " ";
                    status += &user_temp_path;
                    status += " ";
                }
                Err(err) => {
                    status += " \nfailed tryng to create in case of delete everything in the PATH";
                    status += " ";
                    status += &user_temp_path;
                    status += " ";
                    status += &err;
                }
            };

            match GarbageCleaner::delete(win_temp_path.to_string()) {
                Ok(_) => {
                    status += "\nsucess deleting in the PATH";
                    status += " ";
                    status += &win_temp_path;
                    status += " ";
                }
                Err(err) => {
                    status += " \nfailed tryng to delete in case of delete everything in the PATH";
                    status += " ";
                    status += &win_temp_path;
                    status += " ";
                    status += &err;
                }
            };
            match GarbageCleaner::create(win_temp_path.to_string()) {
                Ok(_) => {
                    status += "\nsucess creating in the PATH";
                    status += " ";
                    status += &win_temp_path;
                    status += " ";
                }
                Err(err) => {
                    status += " \nfailed tryng to create in case of delete everything in the PATH\n";
                    status += " ";
                    status += &win_temp_path;
                    status += " ";
                    status += &err;
                }
            };

            match GarbageCleaner::delete(sis_temp_path.to_string()) {
                Ok(_) => {
                    status += "\nsucess deleting in the PATH";
                    status += " ";
                    status += &sis_temp_path;
                    status += " ";
                }
                Err(err) => {
                    status += " \nfailed tryng to delete in case of delete everything in the PATH";
                    status += " ";
                    status += &sis_temp_path;
                    status += " ";
                    status += &err;
                }
            };
            match GarbageCleaner::create(sis_temp_path.to_string()) {
                Ok(_) => {
                    status += "\nsucess creating in the PATH";
                    status += " ";
                    status += &sis_temp_path;
                    status += " ";
                }
                Err(err) => {
                    status += "\nfailed tryng to create a new folder in case of delete everything in the PATH";
                    status += " ";
                    status += &sis_temp_path;
                    status += " ";
                    status += &err;
                }
            };

            if status.contains("failed") {
                return Err(status);
            }

            return Ok("sucess".to_string());
        } else {
            return Err("fail, process aborted".to_string());
        }
    }
}
