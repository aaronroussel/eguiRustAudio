use audiotags::Tag;
use std::fs;
use std::path::PathBuf;
use std::io;

#[derive(Clone)]
pub struct music_file {
    pub name: String,
    pub file_path: PathBuf,
    pub title: String,
    pub artist: String,
    pub duration: f64,
    pub album: String,
}

pub fn get_library() -> Result<Vec<music_file>, io::Error> {
    let mut music_files: Vec<music_file> = Vec::new();
    

// THIS FOLDER MUST ONLY CONTAIN MUSIC FILES OR SHIT WILL BREAK!!!
    let paths = match fs::read_dir("C:\\Users\\aones\\Music") {
        Ok(paths) => paths,
        Err(err) => {
            print!("Error reading directory: {}", err);
            return Err(err);
        }
    }; 
      

    
    for p in paths {
        let dir_entry = p.unwrap();
        if let Some(extension) = dir_entry.path().extension() {
            //added so it does not crash if there are files other than .mp3 extension
            if extension.to_string_lossy().to_lowercase() != "mp3" {
                continue;
            }
        }
        let tag = Tag::new().read_from_path(dir_entry.path().clone()).unwrap();

        let song_title = tag.title().map(|s| s.to_string()).unwrap_or_default();
        let song_artist = tag
            .artists()
            .map(|artists| artists.join(", "))
            .unwrap_or_default();
        let song_duration = tag.duration().unwrap_or_default();
        let song_album = tag.album_title().map(|s| s.to_string()).unwrap_or_default();

        let music = music_file {
            name: dir_entry
                .path()
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string(),
            file_path: dir_entry.path().clone(),
            title: song_title,
            duration: song_duration,
            artist: song_artist,
            album: song_album,
        };

        music_files.push(music);
    }

    Ok(music_files)
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


#[derive(Clone)]
pub struct MusicCollection {
    pub name: String,
    pub collection: Vec<music_file>,
    pub song_count: i32,
}

impl MusicCollection {
    
}
