use file_checker_cli::*;

fn main() -> Result<(), String> {
    let file_path = input_file_path()?;

    is_file_exists(&file_path)?;

    let file_counters = calc_file_counters(&file_path)?;

    println!("{}", file_counters);

    return Ok(());
}
