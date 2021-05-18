use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::PathBuf;
use std::time::SystemTime;

pub trait FileSystemHandler {
    fn open_stream<'a>(&'a self, name: &str) -> Option<Box<dyn Read + 'a>>;
    fn get_last_modification(&self, name: &str) -> Option<SystemTime>;
}

#[derive(Clone, Debug, Default)]
pub struct MemoryFileSystem {
    files_map: HashMap<String, String>,
}

impl MemoryFileSystem {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_file(&mut self, filename: String, file_content: String) {
        self.files_map.insert(filename, file_content);
    }
}

impl FileSystemHandler for MemoryFileSystem {
    #[allow(clippy::manual_map)]
    fn open_stream<'a>(&'a self, name: &str) -> Option<Box<dyn Read + 'a>> {
        if let Some(body) = self.files_map.get(name) {
            Some(Box::new(BufReader::new(body.as_bytes())))
        } else {
            None
        }
    }
    fn get_last_modification(&self, _name: &str) -> Option<SystemTime> {
        Some(SystemTime::now())
    }
}
#[derive(Clone, Debug)]
pub struct RealFileSystem {
    root_folder: String,
}

impl RealFileSystem {
    pub fn new(root_folder: String) -> Self {
        Self { root_folder }
    }
    pub fn set_root_folder(&mut self, new_root: String) {
        self.root_folder = new_root;
    }
    pub fn get_root_folder(&self) -> &str {
        &self.root_folder
    }
    pub fn get_full_file_path(&self, name: &str) -> PathBuf {
        let mut path = PathBuf::from(&self.root_folder);
        path.push(name);
        path
    }
}
impl FileSystemHandler for RealFileSystem {
    fn open_stream<'a>(&'a self, name: &str) -> Option<Box<dyn Read + 'a>> {
        let path = self.get_full_file_path(name);

        let file_exists = File::open(path);
        if let Ok(file) = file_exists {
            Some(Box::new(BufReader::new(file)))
        } else {
            None
        }
    }
    fn get_last_modification(&self, name: &str) -> Option<SystemTime> {
        let path = self.get_full_file_path(name);
        let file_exists = File::open(path);
        if let Ok(file) = file_exists {
            let metadata = file.metadata().unwrap().modified().unwrap();
            Some(metadata)
        } else {
            None
        }
    }
}
