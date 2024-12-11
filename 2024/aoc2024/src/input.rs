use anyhow::Context;
use std::{env, fs, path::PathBuf};

pub fn get_input(day: i32) -> String {
    try_get_input(day).unwrap()
}

pub fn try_get_input(day: i32) -> anyhow::Result<String> {
    let exe_path = env::current_exe()?;
    let exe_dir = exe_path.parent().unwrap();

    let cached_input = exe_dir.with_file_name(format!("input-{day}.txt"));
    if cached_input.exists() {
        return fs::read_to_string(cached_input).context("Failed to read input for today");
    }

    println!("Fetching input for day {day}");

    let home = env::var("HOME")?;
    let access_cookie = fs::read_to_string(PathBuf::from(home).join(".aoc-cookie"))
        .context("Could not read aoc cookie")?;

    let client = reqwest::blocking::Client::new();
    let input = client
        .request(
            reqwest::Method::GET,
            format!("https://adventofcode.com/2024/day/{day}/input"),
        )
        .header("Cookie", format!("session={}", access_cookie.trim()))
        .send()?
        .text()?;

    let input = input.trim().to_string();

    fs::write(&cached_input, &input)?;

    Ok(input)
}
