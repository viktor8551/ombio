use std::{
    fs::File,
    io,
    path::{Component, Path, PathBuf},
};

use crate::Error;

/// Read-only access to files beneath an asset directory.
pub struct Filesystem {
    directory: PathBuf,
}

impl Filesystem {
    /// Resolves the directory against the working directory without opening it.
    pub fn new(directory: impl AsRef<Path>) -> Result<Self, Error> {
        let directory = directory.as_ref();
        let directory = std::path::absolute(directory)
            .map_err(|error| Error::AssetDirectory(directory.to_path_buf(), error))?;
        Ok(Self { directory })
    }

    /// Opens an asset-relative path for reading. Filesystem links are followed.
    pub fn open(&self, path: impl AsRef<Path>) -> Result<File, Error> {
        let path = path.as_ref();
        let mut relative = PathBuf::new();
        for component in path.components() {
            match component {
                Component::Normal(name) if !name.as_encoded_bytes().contains(&b':') => {
                    relative.push(name)
                }
                Component::CurDir => {}
                Component::ParentDir if relative.pop() => {}
                _ => {
                    return Err(Error::AssetOpen(
                        self.directory.join(path),
                        io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "asset paths must be relative and cannot traverse above the asset directory",
                        ),
                    ));
                }
            }
        }
        let path = self.directory.join(relative);
        File::open(&path).map_err(|error| Error::AssetOpen(path, error))
    }
}
