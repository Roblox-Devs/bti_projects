/*
use serde_json::Value;
use std::path::Path;
use tokio::fs;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};

const CHUNK_SIZE: usize = 1024 * 1024; // 1 MB chunks

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    // Get the file name and maximum size from user input
    println!("Welcome to the JSON Splitter");
    println!("First, enter the name of the file you want to split");
    let mut file_name = String::new();
    let stdin = tokio::io::stdin();
    let mut stdin_reader = BufReader::new(stdin);
    stdin_reader.read_line(&mut file_name).await?;
    let file_name = file_name.trim();

    println!("Enter maximum file size (MB):");
    let mut mb_per_file = String::new();
    stdin_reader.read_line(&mut mb_per_file).await?;
    let mb_per_file: u64 = mb_per_file.trim().parse().expect("Invalid number");

    // Calculate the maximum file size in bytes
    let max_file_size = mb_per_file * 1_000_000;

    // Open the file
    let mut file = fs::File::open(file_name).await?;
    let metadata = file.metadata().await?;
    let file_size = metadata.len();

    // Check if the file needs to be split
    if file_size < max_file_size {
        println!("File smaller than split size, exiting");
        return Ok(());
    }

    // Initialize variables for splitting the file
    let mut part_number = 1;
    let mut current_size = 0;
    let mut buffer = String::new();
    let mut json_buffer = Vec::new();

    let mut chunk = vec![0; CHUNK_SIZE];

    loop {
        let n = file.read(&mut chunk).await?;
        if n == 0 {
            break;
        }
        buffer.push_str(&String::from_utf8_lossy(&chunk[..n]));

        while let Some((json_object, remaining)) = extract_json_object(&buffer) {
            let json_size = json_object.len() as u64;
            json_buffer.push(serde_json::from_str(&json_object).expect("Invalid JSON"));
            println!(
                "Part {} ... {} bytes",
                part_number,
                json_size + current_size
            );
            if current_size + json_size >= max_file_size {
                write_to_file(file_name, &json_buffer, part_number).await?;
                part_number += 1;
                json_buffer.clear();
                current_size = 0;
            }

            current_size += json_size;
            buffer = remaining.to_string();
        }
    }

    if !json_buffer.is_empty() {
        write_to_file(file_name, &json_buffer, part_number).await?;
    }

    println!("Success! Script Completed");
    Ok(())
}

// Function to extract a JSON object from the buffer
fn extract_json_object(buffer: &str) -> Option<(String, &str)> {
    let mut in_string = false;
    let mut escape = false;
    let mut depth = 0;

    let mut start_index = None;
    let mut is_place_name = false;

    for (i, c) in buffer.char_indices() {
        match c {
            '"' if !escape => in_string = !in_string,
            '\\' if in_string => escape = !escape,
            '{' if !in_string => {
                depth += 1;
                if depth == 1 {
                    start_index = Some(i);
                    is_place_name = true; // Assuming "place_name" is always first
                }
            }
            '}' if !in_string => {
                depth -= 1;
                if depth == 0 {
                    return Some((
                        buffer[start_index.unwrap()..=i].to_string(),
                        &buffer[i + 1..],
                    ));
                }
            }
            ',' if !in_string && depth == 1 => {
                // Check if "strings" key is encountered after "place_name"
                if is_place_name {
                    is_place_name = false; // Reset for next object
                    let potential_key = &buffer[start_index.unwrap()..i];
                    if potential_key.trim() == r#"`"place_name"`"# {
                        start_index = Some(i + 1); // Start of "strings" value
                    }
                }
            }
            _ => escape = false,
        }
    }
    None
}

// Helper function to write the buffer to a new file
async fn write_to_file(
    base_name: &str,
    buffer: &[Value],
    part_number: u32,
) -> tokio::io::Result<()> {
    let output_file_name = format!(
        "{}_{}.json",
        Path::new(base_name).file_stem().unwrap().to_str().unwrap(),
        part_number
    );
    let file = fs::File::create(output_file_name).await?;
    let mut writer = BufWriter::new(file);
    let json_data = serde_json::to_string(buffer)?;
    writer.write_all(json_data.as_bytes()).await?;
    writer.flush().await?;
    println!("Part {} ... completed", part_number);
    Ok(())
}

*/
use serde_json::Value;
use std::path::Path;
use tokio::fs;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};

const CHUNK_SIZE: usize = 1024 * 1024; 
#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    println!("Welcome to the JSON Splitter");
    println!("First, enter the name of the file you want to split");
    let mut file_name = String::new();
    let stdin = tokio::io::stdin();
    let mut stdin_reader = BufReader::new(stdin);
    stdin_reader.read_line(&mut file_name).await?;
    let file_name = file_name.trim();

    println!("Enter maximum file size (MB):");
    let mut mb_per_file = String::new();
    stdin_reader.read_line(&mut mb_per_file).await?;
    let mb_per_file: u64 = mb_per_file.trim().parse().expect("Invalid number");

    let max_file_size = mb_per_file * 1_000_000;

    let mut file = fs::File::open(file_name).await?;
    let metadata = file.metadata().await?;
    let file_size = metadata.len();

    if file_size < max_file_size {
        println!("File smaller than split size, exiting");
        return Ok(());
    }

    let mut part_number = 1;
    let mut current_size = 0;
    let mut buffer = String::new();
    let mut json_buffer = Vec::new();

    let mut chunk = vec![0; CHUNK_SIZE];

    loop {
        let n = file.read(&mut chunk).await?;
        if n == 0 {
            break;
        }
        buffer.push_str(&String::from_utf8_lossy(&chunk[..n]));

        while let Some((json_object, remaining)) = extract_json_object(&buffer) {
            let json_size = json_object.len() as u64;
            json_buffer.push(serde_json::from_str(&json_object).expect("Invalid JSON"));
            println!(
                "Part {} ... {} bytes",
                part_number,
                json_size + current_size
            );
            if current_size + json_size >= max_file_size {
                write_to_file(file_name, &json_buffer, part_number).await?;
                part_number += 1;
                json_buffer.clear();
                current_size = 0;
            }

            current_size += json_size;
            buffer = remaining.to_string();
        }
    }

    if !json_buffer.is_empty() {
        write_to_file(file_name, &json_buffer, part_number).await?;
    }

    println!("Success! Script Completed");
    Ok(())
}

fn extract_json_object(buffer: &str) -> Option<(String, &str)> {
    let mut in_string = false;
    let mut escape = false;
    let mut depth = 0;

    let mut start_index = None;

    for (i, c) in buffer.char_indices() {
        match c {
            '"' if !escape => in_string = !in_string,
            '\\' if in_string => escape = !escape,
            '{' if !in_string => {
                depth += 1;
                if depth == 1 {
                    start_index = Some(i);
                }
            }
            '}' if !in_string => {
                depth -= 1;
                if depth == 0 {
                    return Some((
                        buffer[start_index.unwrap()..=i].to_string(),
                        &buffer[i + 1..],
                    ));
                }
            }
            _ => escape = false,
        }
    }
    None
}

async fn write_to_file(
    base_name: &str,
    buffer: &[Value],
    part_number: u32,
) -> tokio::io::Result<()> {
    let output_file_name = format!(
        "{}_{}.json",
        Path::new(base_name).file_stem().unwrap().to_str().unwrap(),
        part_number
    );
    let file = fs::File::create(output_file_name).await?;
    let mut writer = BufWriter::new(file);
    let json_data = serde_json::to_string(buffer)?;
    writer.write_all(json_data.as_bytes()).await?;
    writer.flush().await?;
    println!("Part {} ... completed", part_number);
    Ok(())
}
