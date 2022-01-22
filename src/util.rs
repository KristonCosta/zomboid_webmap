use std::path::PathBuf;

pub struct Directory {
    buf: PathBuf,
}

impl Directory {
    pub fn new(path: &str) -> Self {
        let buf = PathBuf::from(&path);
        if !buf.is_dir() {
            panic!("Invalid directory path: {}", path);
        }
        Self { buf }
    }

    pub fn join(&self, dest: &PathBuf) -> PathBuf {
        self.buf.join(dest)
    }
}
