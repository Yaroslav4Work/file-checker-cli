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

pub struct TextCounters {
    lines_count: usize,
    words_count: usize,
    chars_count: usize,
}

impl TextCounters {
    pub fn lines_count(&self) -> usize {
        self.lines_count
    }

    pub fn words_count(&self) -> usize {
        self.words_count
    }

    pub fn chars_count(&self) -> usize {
        self.chars_count
    }
}

// Вынес в отдельный метод для возможности переиспользования
pub fn read_file_content(file_path: &str) -> Result<String, String> {
    let mut file_contents = String::new();

    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => return Err(UNABLE_TO_OPEN.to_string()),
    };

    match BufReader::new(file).read_to_string(&mut file_contents) {
        Ok(_) => (),
        Err(_) => return Err(UNABLE_TO_READ.to_string()),
    }

    Ok(file_contents)
}

// Вынес в отдельный метод для возможности переиспользования
pub fn calc_text_counters(text: &str) -> Result<TextCounters, String> {
    let lines_count = text.lines().count();
    let words_count = text.split_whitespace().count();
    let chars_count = text.chars().count();

    return Ok(TextCounters {
        lines_count,
        words_count,
        chars_count,
    });
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
    fn read_file_content_can_read() {
        let result = read_file_content("./src/test.txt");
        assert!(result.is_ok());
    }

    #[test]
    fn read_file_content_cant_read() {
        let result = read_file_content("./src/text.tst");
        assert!(result.is_err());
    }

    #[test]
    fn calc_text_counters_test() -> Result<(), String> {
        let file_content = read_file_content("./src/test.txt")?;
        let result = calc_text_counters(&file_content);

        assert!(result.is_ok());

        let text_counters = result?;

        assert_eq!(
            (
                text_counters.lines_count(),
                text_counters.words_count(),
                text_counters.chars_count()
            ),
            (4, 50, 340)
        );

        Ok(())
    }
}
