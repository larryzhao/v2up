use crate::errors::kind::ErrorKind;
use crate::errors::Error;
use crate::workdir::file;
use dirs;
use serde_json::json;
use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::str;

pub struct Dir {
    dir: PathBuf,
}

impl Dir {
    pub fn new(path: String) -> Result<Self, Error> {
        let path_str = path.as_str();
        // in home dir
        if path_str.starts_with("~/") {
            let splits = path_str.split("~/");
            let splits_vec: Vec<&str> = splits.collect();
            if splits_vec.len() <= 1 {
                return Err(Error {
                    kind: ErrorKind::InvalidPath,
                    message: format!("path {} is not a vaild home dir", path_str),
                });
            }

            let home_dir = match dirs::home_dir() {
                Some(dir) => dir,
                None => {
                    return Err(Error {
                        kind: ErrorKind::InvalidPath,
                        message: format!("cannot get homedir path"),
                    });
                }
            };

            let buf = home_dir.join(splits_vec[1]);
            return Ok(Dir { dir: buf });
        }

        let buf = PathBuf::from(path_str);
        let abs_buf = match fs::canonicalize(&buf) {
            Ok(path_buf) => path_buf,
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => {
                    // do not raise err on path not exists
                    return Ok(Dir { dir: buf });
                }
                _ => {
                    return Err(Error {
                        kind: ErrorKind::InvalidPath,
                        message: format!("path invalid"),
                    })
                }
            },
        };

        Ok(Dir { dir: abs_buf })
    }

    pub fn path(self: &Self) -> &str {
        return self.dir.to_str().unwrap();
    }

    pub fn filepath(self: &Self, name: &str) -> String {
        let filepath = self.dir.join(name).to_str().unwrap().to_owned();
        return filepath;
    }

    pub fn exist(self: &Self) -> bool {
        return self.dir.exists();
    }

    pub fn available(self: &Self) -> Result<(), Error> {
        if !self.dir.exists() {
            return Err(Error {
                kind: ErrorKind::WorkdirUninitialized,
                message: format!("work directory {} not exist", self.path()),
            });
        }

        let read_dir_result = self.dir.read_dir();
        if read_dir_result.is_err() || read_dir_result.unwrap().next().is_none() {
            return Err(Error {
                kind: ErrorKind::WorkdirUninitialized,
                message: format!("work directory {} is empty", self.path()),
            });
        }

        // TODO: how to find dir already taken by others?
        // if !path.is_dir() {
        //     return Err(Error {
        //         kind: ErrorKind::DirAlreadyTaken,
        //         message: format!("dir {} already taken", self.dir.as_str()),
        //     });
        // }

        Ok(())
    }

    /// init initialize a dir as v2up workspace
    pub fn init(self: &Self) -> Result<(), Error> {
        match self.available() {
            Ok(_) => {
                // if dir already ok
                return Ok(());
            }
            Err(err) => match err.kind {
                ErrorKind::WorkdirUninitialized => {
                    // create dir first
                    let result = self.create();
                    if result.is_err() {
                        return Err(err);
                    }

                    // create default files
                    let template_generations: [(&str, &serde_json::Value); 3] = [
                        (
                            "settings.yaml",
                            &json!({
                                "v2up_log": self.dir.join("v2up.log").to_str().unwrap()
                            }),
                        ),
                        (
                            "v2ray.json",
                            &json!({
                                "v2ray_access_log": self.dir.join("v2ray.access.log").to_str().unwrap(),
                                "v2ray_error_log": self.dir.join("v2ray.error.log").to_str().unwrap()
                            }),
                        ),
                        ("pac.js", &json!({})),
                    ];

                    for tup in template_generations {
                        let name = tup.0;
                        let data = tup.1;

                        let dest_file = self.dir.join(name);
                        let template_name = format!("{}.handlebars", name);

                        match file::create_file_with_template(
                            dest_file.to_str().unwrap(),
                            template_name.as_str(),
                            data,
                        ) {
                            Err(err) => {
                                // print create file error
                                println!("create file {} with errr {}", name, err)
                            }
                            Ok(_) => (),
                        }
                    }
                    return Ok(());
                }
                _ => {
                    return Err(Error {
                        kind: ErrorKind::ExecuteCommandError,
                        message: format!("init workspace dir err: {}", err),
                    })
                }
            },
        }
    }

    pub fn create(self: &Self) -> Result<(), Error> {
        fs::create_dir_all(self.dir.to_str().unwrap()).unwrap();
        return Ok(());
    }
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.path())
    }
}
