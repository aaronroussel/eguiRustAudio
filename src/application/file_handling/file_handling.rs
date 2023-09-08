use std::path::PathBuf;
use std::fs;
use audiotags::Tag;

#[derive(Clone)]
pub struct music_file {
    pub name: String,
    pub file_path: PathBuf,
    pub title: String,
    pub artist: String,
    pub duration: f64,
    pub album: String,
}

pub fn get_library() -> Vec<music_file> {
    let mut music_files: Vec<music_file> = Vec::new();
    
    // let paths = fs::read_dir("C:\\Users\\aaron\\Music\\songs").unwrap(); // THIS FOLDER MUST ONLY CONTAIN MUSIC FILES OR SHIT WILL BREAK!!!
    let paths = fs::read_dir("/Users/aaronroussel/Music/Music/Media.localized/Music/Future/Unknown Album").unwrap();
    
    for p in paths {
        let dir_entry = p.unwrap();
        let tag = Tag::new().read_from_path(dir_entry.path().clone()).unwrap();
        
        
        let song_title = tag.title().map(|s| s.to_string()).unwrap_or_default();
        let song_artist = tag.artists().map(|artists| artists.join(", ")).unwrap_or_default();
        let song_duration = tag.duration().unwrap_or_default(); 
        let song_album = tag.album_title().map(|s| s.to_string()).unwrap_or_default();
        
        let music = music_file {
            name: dir_entry.path().file_name().unwrap().to_string_lossy().to_string(),
            file_path: dir_entry.path().clone(),
            title: song_title,
            duration: song_duration,
            artist: song_artist,
            album: song_album,
        };
        
        music_files.push(music);
    }
    
    music_files
}

pub fn get_from_path(path_string: &str) -> Vec<music_file> {
    
    let mut music_files: Vec<music_file> = Vec::new();
    
    let paths = fs::read_dir(path_string).unwrap();
    
    for p in paths {
        let dir_entry = p.unwrap();
        let tag = Tag::new().read_from_path(dir_entry.path().clone()).unwrap();
        
        
        let song_title = tag.title().map(|s| s.to_string()).unwrap_or_default();
        let song_artist = tag.artists().map(|artists| artists.join(", ")).unwrap_or_default();
        let song_duration = tag.duration().unwrap_or_default(); 
        let song_album = tag.album_title().map(|s| s.to_string()).unwrap_or_default();
        
        let music = music_file {
            name: dir_entry.path().file_name().unwrap().to_string_lossy().to_string(),
            file_path: dir_entry.path().clone(),
            title: song_title,
            duration: song_duration,
            artist: song_artist,
            album: song_album,
        };
        
        music_files.push(music);
    }
    
    
    music_files
}
