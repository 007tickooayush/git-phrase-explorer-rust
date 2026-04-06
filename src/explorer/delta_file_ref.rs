use git2::{DiffFile, Repository};


pub struct DeltaFileRef<'diff> {
    path: &'diff std::path::Path,
    size: usize
}

impl<'diff> DeltaFileRef<'diff> {
    /// That helper returns None when:
    /// 
    /// - the diff file does not exist,
    /// - the path is missing,
    /// - or repo.find_blob(oid) fails.
    /// 
    pub fn new(repo: &Repository, file: DiffFile<'diff>) -> Option<Self> {
        if !file.exists() {
            return None;
        }

        let path = file.path()?;

        let oid = file.id();
        let Ok(blob) = repo.find_blob(oid) else {
            return None;
        };

        Some(Self {
            path,
            size: blob.size()
        })
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn get_path(&self) -> &std::path::Path {
        self.path
    }
}