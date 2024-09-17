use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

const HAS_NO_PATH: &str = "Has no file path";
const IS_NOT_EXISTS: &str = "File is not exists";
const UNABLE_TO_OPEN: &str = "Unable to open the file";
const UNABLE_TO_READ: &str = "Unable to read the file";

/*
    Вынес в отдельный метод для возможности переиспользования
    или замены другим методом чтения пользовательского ввода
*/
pub fn input_file_path() -> Result<String, String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 || args[1] == "" {
        return Err(HAS_NO_PATH.to_string());
    }

    return Ok(args[1].clone());
}

// Вынес в отдельный метод для возможности переиспользования
pub fn is_file_exists(file_path: &str) -> Result<(), String> {
    if Path::new(file_path).exists() {
        return Ok(());
    }

    return Err(IS_NOT_EXISTS.to_string());
}

// Вынес в отдельный метод для возможности переиспользования
pub fn calc_file_counters(file_path: &str) -> Result<String, String> {
    let mut file_contents = String::new();

    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => return Err(UNABLE_TO_OPEN.to_string()),
    };

    match BufReader::new(file).read_to_string(&mut file_contents) {
        Ok(_) => (),
        Err(_) => return Err(UNABLE_TO_READ.to_string()),
    }

    let lines = file_contents.lines().count();

    file_contents = file_contents.replace("\n", " ");

    let words = file_contents.split(" ").filter(|ch| *ch != "").count();

    let chars = file_contents
        .replace(" ", "")
        .split("")
        .filter(|ch| *ch != "")
        .count();

    return Ok(format!(
        "Words: {}\nLines: {}\nCharacters: {}",
        words, lines, chars
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_is_exists() {
        let result = is_file_exists("./src/test.txt");
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn file_is_not_exists() {
        let result = is_file_exists("./src/text.tst");
        assert_eq!(result, Err(IS_NOT_EXISTS.to_string()));
    }

    #[test]
    fn calc_file_counters_can_read() {
        let result = calc_file_counters("./src/test.txt");
        assert_eq!(
            result,
            Ok("Words: 50\nLines: 4\nCharacters: 287".to_string())
        );
    }

    #[test]
    fn calc_file_counters_cant_read() {
        let result = calc_file_counters("./src/text.tst");
        assert_eq!(result, Err(UNABLE_TO_OPEN.to_string()));
    }
}
