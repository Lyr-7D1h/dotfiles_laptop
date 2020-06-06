use std::env;
use std::path;

pub struct ImporterArgs {
  pub srcpath: path::PathBuf,
  pub destpath: path::PathBuf,
  pub backup: bool,
}

impl ImporterArgs {
  fn default() -> ImporterArgs {
    let dot_files_path  = match env::current_dir() {
      Err(_) => panic!("Could not fetch current working directory"),
      Ok(value) => value.parent().unwrap().canonicalize().unwrap()
    };

    let home_path = match env::var("HOME") {
      Err(_) => panic!("Could not find home directory"),
      Ok(value) => value
    };

    let destpath = path::PathBuf::from(home_path);

    ImporterArgs {
      srcpath: dot_files_path,
      destpath: destpath,
      backup: true
    }
  }

  pub fn new() -> Result<ImporterArgs, String> {
    let mut args: Vec<String> = env::args().collect();

    let mut importer_args = ImporterArgs::default();

    args.remove(0);

    for arg in args.iter() {
      match arg.as_str() {
        "-h" | "--help" => return Err(String::from("Help")),
        "-n" | "--no-backup" => importer_args.backup = false,
        "-t" | "--test" => {
          println!("Using test paths..");
          importer_args.srcpath = path::PathBuf::from("test-config/new_config");
          importer_args.destpath = path::PathBuf::from("test-config/config");
        },
        _ => return Err(format!("Invalid option: {}", arg))
      }
    }

    Ok(importer_args)
  }
}