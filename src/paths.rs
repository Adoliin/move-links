use path_clean::{self, PathClean};
use std::path::{Path, PathBuf};
use std::{env, fs, io};

#[derive(PartialEq, Debug)]
pub struct Paths {
    pub links_dir: String,
    pub src_path: String,
    pub dest_path: String,
}

impl Paths {
    pub fn new(links_dir_str: &str, src_path: &str, dest_path: &str) -> Result<Paths, String> {
        let pb_links_dir = PathBuf::from(links_dir_str);
        let pb_src_path = PathBuf::from(src_path);
        let mut pb_dest_path = PathBuf::from(dest_path);

        let pb_links_dir = match fs::canonicalize(&pb_links_dir) {
            Ok(x) => x,
            Err(_) => return Err(format!("links_dir: {:#?} is invalid", pb_links_dir)),
        };

        let pb_src_path = match fs::canonicalize(&pb_src_path) {
            Ok(x) => x,
            Err(_) => return Err(format!("src_path: {:#?} is invalid", pb_src_path)),
        };

        if pb_dest_path.is_relative() {
            pb_dest_path = absolute_path(&pb_dest_path).unwrap();
        }
        if pb_dest_path.exists() {
            if pb_dest_path.is_dir() {
                pb_dest_path.push(pb_src_path.file_name().unwrap());
            } else {
                return Err(format!(
                    "dest_path: {:#?} is invalid: a file with that name already exists",
                    pb_dest_path
                ));
            }
        } else {
            let mut pb_tmp = pb_dest_path.clone();
            pb_tmp.pop();
            if !pb_tmp.exists() {
                return Err(format!(
                    "dest_path: {:#?} is invalid: path does not exist",
                    pb_dest_path
                ));
            }
        }
        let links_dir = pb_links_dir.into_os_string().into_string().unwrap();
        let src_path  = pb_src_path.into_os_string().into_string().unwrap();
        let dest_path = pb_dest_path.into_os_string().into_string().unwrap();

        Ok(Paths {
            links_dir,
            src_path,
            dest_path,
        })
    }
}

fn absolute_path(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    let path = path.as_ref();
    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()?.join(path)
    }
    .clean();
    Ok(absolute_path)
}
