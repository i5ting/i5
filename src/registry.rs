use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::path::Path;

struct InternalFile {
    path: String,
    pub data: String,
    pub actual_size: usize,
}

pub struct FileSystem {
    files: HashMap<String, InternalFile>,
}

// 将写入数据先写入到内存中
pub fn write(&mut self, path: &str, data: &str) {
    self.files.insert(
        calculate_hash(&path.to_string().as_bytes()),
        InternalFile {
            path: path.to_string(),
            data: data.to_string(),
            actual_size: data.len(),
        },
    );
}

// 将文件系统数据转存到文件
pub fn save<T: AsRef<Path>>(&self, path: T) {
    let mut f = fs::File::create(&path).unwrap();
    for (_, file) in &self.files {
        f.write(&file.data.as_bytes()).unwrap();
    }
}
