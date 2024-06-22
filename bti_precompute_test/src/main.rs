use aho_corasick::AhoCorasick;
use serde_json::{self, Value};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use zune_inflate::DeflateDecoder;

async fn get_folders() -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let entries = fs::read_dir(".")?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .map(|entry| entry.path())
        .collect();
    Ok(entries)
}

async fn get_parts(year: &PathBuf) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let entries = fs::read_dir(year)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .map(|entry| entry.path())
        .collect();
    Ok(entries)
}

async fn get_places_in_part(
    _year: &PathBuf,
    part: &PathBuf,
) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let entries = fs::read_dir(part)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect();
    Ok(entries)
}

async fn read_place(place_path: &PathBuf) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut file = File::open(place_path).await?;
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).await?;
    Ok(buffer)
}

fn search_aho_corasick(
    file_buffer: &Vec<u8>,
    map: &mut Vec<HashMap<String, Value>>,
    place_path: &PathBuf,
) {
    let mut decoder = DeflateDecoder::new(file_buffer);
    let data = decoder.decode_zlib().unwrap();

    let string = String::from_utf8_lossy(&data);
    let true_string = string.to_string();
    let string_len = string.len();
    let bytes = string.as_bytes();

    let mut output: HashSet<String> = HashSet::new();
    let patterns = &["<string", "<ProtectedString"];
    let ac = AhoCorasick::builder()
        .ascii_case_insensitive(false)
        .build(patterns)
        .unwrap();

    for mat in ac.find_iter(&true_string) {
        let mut idx = mat.end() + 1;
        while idx < string_len && bytes[idx] != b'>' {
            idx += 1;
        }

        idx += 1;
        let start = idx;
        while idx < string_len && bytes[idx] != b'<' {
            idx += 1;
        }

        if start != idx {
            output.insert(true_string[start..idx].to_string());
        }
    }

    let strings_array: Vec<Value> = output.into_iter().map(Value::String).collect(); // slow?

    let mut new_thing = HashMap::new();
    new_thing.insert(
        // ugly, but it works
        "place_name".to_string(),
        Value::String(
            place_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        ),
    );
    new_thing.insert("strings".to_string(), Value::Array(strings_array));

    map.push(new_thing);
}

async fn process_place(
    _year: &PathBuf,
    _part: &PathBuf,
    file: &PathBuf,
    map: &mut Vec<HashMap<String, Value>>,
) -> Result<(), Box<dyn Error>> {
    let place_path = file;
    let file_buffer = read_place(&place_path).await?;
    if !file_buffer.is_empty() {
        if file_buffer[0] == 0x78 && file_buffer[1] == 0x9C {
            search_aho_corasick(&file_buffer, map, &place_path);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut map_array: Vec<HashMap<String, Value>> = Vec::new();
    let time = std::time::Instant::now();
    let folders = get_folders().await?;
    if folders.is_empty() {
        println!("No folders found.");
        return Ok(());
    }

    let current_folder = &folders[0]; // 0 for 2006, 1 for 2007, etc
    println!("Processing folder: {:?}", current_folder);
    let parts = get_parts(current_folder).await?;
    for part in parts {
        println!("Processing part: {:?}", part);
        let files = get_places_in_part(current_folder, &part).await?;
        for file in files {
            process_place(current_folder, &part, &file, &mut map_array).await?;
        }
        println!("Finished processing part: {:?}", part);
    }
    let file = fs::File::create(
        "output-".to_owned() + current_folder.file_name().unwrap().to_str().unwrap() + ".json",
    )?;
    let writer = std::io::BufWriter::new(file);
    serde_json::to_writer(writer, &map_array)?;

    println!("Total time: {:?}", time.elapsed());
    Ok(())
}
