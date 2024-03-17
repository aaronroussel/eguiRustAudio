use audiotags::Tag;
use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeMap, SerializeSeq, SerializeStruct, Serializer};
use std::fmt;
use std::fs;
use std::io;
use std::path::PathBuf;

//---------------------------------------------------------------------------------------------------
// MusicFile struct
// This struct is used to store the metadata of a music file, as well as the location of the file.
// --------------------------------------------------------------------------------------------------
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct MusicFile {
    pub name: String,
    pub file_path: PathBuf,
    pub title: String,
    pub artist: String,
    pub duration: f64,
    pub album: String,
}

pub fn get_from_path(path_string: &str) -> Vec<MusicFile> {
    // -----------------------------------------------------------------------------------------------
    // ** might need to change the way this works. currently, trying to add more music to the library
    // after this function has already been called will overwrite the existing music lbrary. this needs
    // to be changed so a Vec<MusicFile> is created outside this function and is passed in by reference
    // and then we push the new music files to the existing Vec<MusicFile> **
    // -----------------------------------------------------------------------------------------------
    let mut music_files: Vec<MusicFile> = Vec::new();

    let paths = fs::read_dir(path_string).unwrap();

    for p in paths {
        let dir_entry = p.unwrap();
        let tag = Tag::new().read_from_path(dir_entry.path().clone()).unwrap();

        let song_title = tag.title().map(|s| s.to_string()).unwrap_or_default();
        let song_artist = tag
            .artists()
            .map(|artists| artists.join(", "))
            .unwrap_or_default();
        let song_duration = tag.duration().unwrap_or_default();
        let song_album = tag.album_title().map(|s| s.to_string()).unwrap_or_default();

        let music = MusicFile {
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

    music_files
}

pub fn new_library() -> Vec<MusicFile> {
    let library: Vec<MusicFile> = Vec::new();
    library
}

//---------------------------------------------------------------------------------------------------
// MusicCollection struct
// This struct is used to store a collection [aka playlists] of music files, as well as the name of the collection
// and the number of songs in the collection.
// --------------------------------------------------------------------------------------------------
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct MusicCollection {
    pub name: String,
    pub collection: Vec<MusicFile>,
    pub song_count: i32,
    pub index: i32,
}

impl MusicCollection {
    pub fn new(s: String, i: i32) -> MusicCollection {
        let collection = MusicCollection {
            name: s,
            collection: Vec::new(),
            song_count: 0,
            index: i,
        };
        collection
    }

    pub fn add_song(&mut self, music_file: MusicFile) {
        self.collection.push(music_file);
        self.song_count += 1;
    }
}
