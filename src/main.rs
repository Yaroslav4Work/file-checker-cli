use file_checker_cli::*;

fn main() -> Result<(), String> {
    let file_path = input_file_path()?;

    is_file_exists(&file_path)?;

    let file_content = read_file_content(&file_path)?;

    let file_counters = calc_text_counters(&file_content)?;

    println!(
        "Words: {} Lines: {} Characters: {}",
        file_counters.lines_count(),
        file_counters.words_count(),
        file_counters.chars_count()
    );

    return Ok(());
}
