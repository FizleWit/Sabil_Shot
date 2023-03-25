use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn delete_oldest_files(path: &Path) {
    let entries = fs::read_dir(path).unwrap();
    let mut files = vec![];

    for entry in entries {
        let entry = entry.unwrap();
        let file_type = entry.file_type().unwrap();
        if file_type.is_file() {
            files.push((entry.path(), entry.metadata().unwrap().modified().unwrap()));
        }
    }

    if files.len() >= 4 {
        files.sort_by_key(|f| f.1);
        let mut oldest_files = files.drain(0..files.len()-2);
        for oldest_file in oldest_files {
            fs::remove_file(oldest_file.0).unwrap();
        }
    }
}

fn main() {
    let path = Path::new("F:\\School\\Capstone\\FFMPEGCache");
    loop {
        delete_oldest_files(path);
        std::thread::sleep(std::time::Duration::from_secs(3));
    }
}

//cargo run to execute
//in the directory deletes the oldest files leaving the two newest (exempts an incompletly written file *FFMPEG mp4 being written) triggers every 3 seconds
//will only delte if there are 4 or more files in the directory