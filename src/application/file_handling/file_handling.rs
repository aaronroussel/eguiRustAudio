use std::path::PathBuf;
use std::fs;
use audiotags::Tag;
#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct music_file {
    pub name: String,
    pub file_path: PathBuf,
    pub title: String,
}

pub fn get_library() -> Vec<music_file> {
    let mut music_files: Vec<music_file> = Vec::new();
    
    let paths = fs::read_dir("C:\\Users\\aaron\\Music\\songs").unwrap(); // THIS FOLDER MUST ONLY CONTAIN MUSIC FILES OR SHIT WILL BREAK!!!
    
    for p in paths {
        let dir_entry = p.unwrap();
        let mut tag = Tag::new().read_from_path(dir_entry.path().clone()).unwrap();
        
        
        let song_title = tag.title().map(|s| s.to_string()).unwrap_or_default(); // Thank you ChatGPT
        
        let music = music_file {
            name: dir_entry.path().file_name().unwrap().to_string_lossy().to_string(),
            file_path: dir_entry.path().clone(),
            title: song_title
        };
        
        music_files.push(music);
    }
    
    for m in &music_files {
        println!("File Name: {},\n -- File Path: {}\n -- Title: {}", m.name, m.file_path.display(), m.title)
    }
    
    music_files
}
