use indicatif::{ParallelProgressIterator, ProgressStyle};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;
use std::error::Error;
use walkdir::WalkDir;

pub fn walk(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut files = Vec::new();
    for entry in WalkDir::new(path) {
        let entry = entry?;
        if entry.file_type().is_file() {
            files.push(entry.path().to_str().unwrap().to_string());
        }
    }
    Ok(files)
}

pub fn find(files: Vec<String>, pattern: &str) -> Vec<String> {
    let mut matches = Vec::new();
    for file in files {
        if file.contains(pattern) {
            matches.push(file);
        }
    }
    matches
}

pub fn checksum(files: Vec<String>) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    let checksums = std::sync::Mutex::new(HashMap::new());
    let progress_bar = indicatif::ProgressBar::new(files.len() as u64);
    let style = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap();
    progress_bar.set_style(style);
    files
        .par_iter()
        .progress_with(progress_bar)
        .for_each(|file| {
            let checksum = md5::compute(std::fs::read(file).unwrap());
            let checksum = format!("{:x}", checksum);
            let mut checksums = checksums.lock().unwrap();
            checksums
                .entry(checksum)
                .or_insert_with(Vec::new)
                .push(file.to_string());
        });
    Ok(checksums.into_inner().unwrap())
}

pub fn find_duplicates(checksums: HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
    let mut duplicates = Vec::new();
    for (_checksum, files) in checksums {
        if files.len() > 1 {
            duplicates.push(files);
        }
    }
    duplicates
}

pub fn run(path: &str, pattern: &str) -> Result<(), Box<dyn Error>> {
    let files = walk(path)?;
    let files = find(files, pattern);
    println!("Found {} files matching {}", files.len(), pattern);
    let checksums = checksum(files)?;
    let duplicates = find_duplicates(checksums);
    println!("{:?}", duplicates);
    for duplicate in duplicates {
        println!("{}", duplicate.join(", "));
    }
    Ok(())
}
