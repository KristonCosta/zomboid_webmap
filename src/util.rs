use std::path::PathBuf;

pub struct Directory {
    buf: PathBuf,
}

impl Directory {
    pub fn new(path: String) -> Self {
        let buf = PathBuf::from(&path);
        if !buf.is_dir() {
            panic!("Invalid directory path: {}", path);
        }
        Self { buf }
    }

    pub fn path_to(&self, dest: &str) -> PathBuf {
        self.buf.join(dest)
    }
}
