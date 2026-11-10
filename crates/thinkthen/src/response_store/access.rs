//! Admit the private directory and retain its identity across every store operation.
use super::error;
use crate::{Code, Error};
use std::fs::{self, File, OpenOptions};
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub(super) struct Directory {
    path: PathBuf,
    database: PathBuf,
    held: File,
    database_file: File,
}
impl Directory {
    pub(super) fn open(path: &Path, replay: bool) -> Result<(Self, bool), Error> {
        let fail = |_| error(Code::LocalResponseStore);
        let created = if replay {
            false
        } else {
            let mut builder = fs::DirBuilder::new();
            #[cfg(unix)]
            {
                use std::os::unix::fs::DirBuilderExt;
                builder.mode(0o700);
            }
            match builder.create(path) {
                Ok(()) => true,
                Err(cause) if cause.kind() == std::io::ErrorKind::AlreadyExists => false,
                Err(_) => return Err(error(Code::LocalResponseStore)),
            }
        };
        if !fs::symlink_metadata(path).map_err(fail)?.is_dir() {
            return Err(error(Code::LocalResponseStore));
        }
        let held = crate::access::directory(path).map_err(fail)?;
        if !held.metadata().map_err(fail)?.is_dir() {
            return Err(error(Code::LocalResponseStore));
        }
        if created {
            crate::access::AccessList::private(&held, path).map_err(fail)?;
        }
        crate::access::validate_private(&held).map_err(fail)?;
        let database = path.join("responses.sqlite3");
        if let Ok(metadata) = fs::symlink_metadata(&database)
            && !metadata.is_file()
        {
            return Err(error(Code::LocalResponseStore));
        }
        let mut options = OpenOptions::new();
        options.read(true);
        let mut new = false;
        let database_file = if replay {
            options.open(&database).map_err(fail)?
        } else {
            options.write(true);
            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                options.mode(0o600);
            }
            match options.create_new(true).open(&database) {
                Ok(file) => {
                    new = true;
                    crate::access::AccessList::private(&file, &database).map_err(fail)?;
                    file
                }
                Err(cause) if cause.kind() == std::io::ErrorKind::AlreadyExists => {
                    options.create_new(false).open(&database).map_err(fail)?
                }
                Err(_) => return Err(error(Code::LocalResponseStore)),
            }
        };
        if replay {
            let mut header = [0; 20];
            let count = (&database_file).take(20).read(&mut header).map_err(fail)?;
            if count == 20 && (header.get(18) == Some(&2) || header.get(19) == Some(&2)) {
                return Err(error(Code::LocalStoreFormat));
            }
        }
        let directory = Self {
            path: path.to_path_buf(),
            database,
            held,
            database_file,
        };
        directory.check()?;
        Ok((directory, new))
    }
    pub(super) fn database(&self) -> &Path {
        &self.database
    }
    pub(super) fn check(&self) -> Result<(), Error> {
        let fail = |_| error(Code::LocalResponseStore);
        for path in [&self.path, &self.database] {
            if fs::symlink_metadata(path)
                .map_err(fail)?
                .file_type()
                .is_symlink()
            {
                return Err(error(Code::LocalResponseStore));
            }
        }
        for suffix in ["-wal", "-shm"] {
            if self
                .path
                .join(format!("responses.sqlite3{suffix}"))
                .try_exists()
                .map_err(fail)?
            {
                return Err(error(Code::LocalStoreFormat));
            }
        }
        let directory = crate::access::directory(&self.path).map_err(fail)?;
        let database = File::open(&self.database).map_err(fail)?;
        if !crate::access::AccessList::same_file(&self.held, &directory).map_err(fail)?
            || !crate::access::AccessList::same_file(&self.database_file, &database)
                .map_err(fail)?
        {
            return Err(error(Code::LocalResponseStore));
        }
        crate::access::validate_private(&directory).map_err(fail)?;
        crate::access::validate_private(&database).map_err(fail)?;
        if !database.metadata().map_err(fail)?.is_file() {
            return Err(error(Code::LocalResponseStore));
        }
        Ok(())
    }
}
