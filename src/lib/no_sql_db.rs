pub fn read_text(path: String) -> Result<String, Box<dyn std::error::Error>> {
    let contents: String = std::fs::read_to_string(path)?;
    return Ok(contents);
}

pub fn search_in_text(content: &str, querry: &Vec<String>) -> Result<Vec<String>, String> {
    let mut result: Vec<String> = Vec::new();

    for line in content.lines() {
        for keyword in querry {
            if line.contains(keyword) {
                if !result.contains(&&line.to_string()) {
                    result.push(line.to_string());
                }
            }
        }
    }

    if result.len() < 1 {
        return Err("did not found anything or does not have a querry keyword"
            .trim()
            .to_string());
    }

    return Ok(result);
}

pub fn create_db(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    std::fs::File::create(path)?;

    Ok("Collection created successfully".into())
}

pub fn delete_db(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    std::fs::remove_file(path)?;

    Ok("Collection deleted successfilly".into())
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn read_valid_text() {
        let filepath: String = "poem.pdf"
            .trim()
            .to_string();

        match read_text(filepath) {
            Ok(_) => println!(""),
            Err(_) => panic!(""),
        }
    }

    #[test]
    #[should_panic]
    fn read_invalid_text() {
        let filepath: String = "poem.pdfs"
            .trim()
            .to_string();

        match read_text(filepath) {
            Ok(_) => println!(""),
            Err(_) => panic!(""),
        }
    }

    #[test]
    fn search_valid_in_text() {
        let filepath: String = "poem.pdf"
            .trim()
            .to_string();

        let querry: Vec<String> = ["uol".trim().to_string()].to_vec();
        let text: String;

        match read_text(filepath) {
            Ok(s) => text = s,
            Err(_) => panic!(""),
        }

        match search_in_text(&text, &querry) {
            Ok(_) => println!(""),
            Err(_) => panic!(""),
        }
    }

    #[test]
    #[should_panic]
    fn search_invalid_in_text() {
        let filepath: String = "poem.pdfs"
            .trim()
            .to_string();
        let querry: Vec<String> = ["uol".trim().to_string()].to_vec();
        let text: String;

        match read_text(filepath) {
            Ok(s) => text = s,
            Err(_) => panic!(""),
        }

        match search_in_text(&text, &querry) {
            Ok(_) => println!(""),
            Err(_) => panic!(""),
        }
    }

    #[test]
    fn create_valid_collection() {
        let path = "foods.json".trim();

        match create_db(path) {
            Ok(_) => println!(""),
            Err(err) => panic!("{}", err.to_string()),
        }
    }

    #[test]
    #[should_panic]
    fn create_invalid_collection() {
        let path = "cli_scalable_project_missing_folder/foodsss.json".trim();

        match create_db(path) {
            Ok(_) => println!(""),
            Err(err) => panic!("{}", err.to_string()),
        }
    }

    #[test]
    fn delete_valid_collection() {

        let path = "foods.json".trim();

        match create_db(path) {
            Ok(_) => println!(""),
            Err(err) => panic!("{}", err.to_string()),
        }

        match delete_db(path) {
            Ok(_) => println!(""),
            Err(err) => panic!("{}", err.to_string()),
        }
    }

    #[test]
    #[should_panic]
    fn delete_invalid_collection() {

        let mut path = "foods.json".trim();

        match create_db(path) {
            Ok(_) => println!(""),
            Err(err) => panic!("{}", err.to_string()),
        }

        path = "cli_scalable_project/foods.json_ERROR".trim();

        match delete_db(path) {
            Ok(_) => println!(""),
            Err(err) => panic!("{}", err.to_string()),
        }
    }

}