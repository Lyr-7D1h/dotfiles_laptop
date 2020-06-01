use std::fs;
use std::path;
use std::io;

pub struct Importer {
    destpath: path::PathBuf,
    srcpath: path::PathBuf
}

impl Importer {
    pub fn new(destpath: &str, srcpath: &str) -> Result<Importer, String> {
        let importer = Importer {
            destpath: path::PathBuf::from(destpath),
            srcpath: path::PathBuf::from(srcpath)
        };

        importer.validate()?;

        Ok(importer)
    }

    fn validate(&self) -> Result<(), &str> {
        if ! &self.destpath.is_dir() {
            return Err("Destination path in not a directory")
        }
        if ! &self.srcpath.is_dir() {
            return Err("Source path in not a directory")
        }

        Ok(())
    }

    pub fn backup(&self) -> io::Result<()> {
        let mut backup_path = self.destpath.with_file_name("config-backup");

        let mut file_extension = 0;
        loop {
            let error = fs::create_dir(&backup_path).err();

            match error {
                None => {
                    break; 
                },
                Some(err) => {
                    if err.kind() == io::ErrorKind::AlreadyExists {
                        let file_name ;
                        if file_extension > 0 {
                            file_name = format!("config-backup{}", file_extension);
                        } else {
                            file_name = String::from("config-backup");
                        }

                        backup_path.set_file_name(file_name);
                        file_extension += 1;
                    } else {
                        println!("{}", err);
                        return Err(
                            io::Error::new(err.kind(), "could not create backup directory")
                        );
                    }
                }
            }
        }

        println!("Copying from {} to {}", &self.destpath.display(), backup_path.display());

        return copy(&self.destpath, &backup_path);

        // Ok(())
    }

    pub fn link(&self) -> std::io::Result<()> {
        // Hard link
        Ok(())
    }
}

///
/// Move all files from one directory to another
/// Wraps recursive function copy_current_dir
///
fn copy(src_dir: &path::PathBuf, dest_dir: &path::PathBuf) -> io::Result<()> {
    let orig_dir = src_dir.clone();

    return copy_current_dir(&orig_dir, src_dir, dest_dir);
}

fn copy_current_dir(orig_dir: &path::PathBuf, cur_dir: &path::PathBuf, dest_dir: &path::PathBuf) -> io::Result<()> {

    for entry in cur_dir.read_dir()? {
        if let Ok(entry) = entry {
            println!("Found: {:?}", entry);

            // Resolve all symbolic links and get the full absolute path
            let found_path = entry.path().canonicalize().unwrap();

            // New absolute path from config directory
            let new_found_path = dest_dir.join(
                found_path.strip_prefix(&orig_dir).unwrap()
            );

            if found_path.is_file() {
                println!("New file path: from {:?} to {:?}",
                    found_path,
                    new_found_path
                );

                match fs::copy(entry.path(), new_found_path) {
                    Ok(_) => (),
                    Err(err) => {
                        return Err(
                            io::Error::new(
                                err.kind(),
                                "could not copy file to backup location"
                            )
                        )
                    }
                }
            } else {
                // if dir do recurse
                fs::create_dir(new_found_path)?;
                return copy_current_dir(orig_dir, &found_path, &dest_dir)
            }
        }
    };

    Ok(())
}