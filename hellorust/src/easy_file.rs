use std::io;
use std::io::Read;
use std::fs::File;

pub struct EasyFile {}

impl EasyFile {
    pub fn new() -> EasyFile {
        EasyFile{}
    }

    // pub fn read_file(self, file_path: String) -> Result<String,io::Error> {
    //     let mut f = match File::open(file_path) {
    //         Err(e) => return Err(e),
    //         Ok(f) => f,
    //     };
    //
    //     let mut data = String::new();
    //     match f.read_to_string(&mut data) {
    //         Err(e) => Err(e),
    //         Ok(_) => Ok(data),
    //     }
    // }

    // 简化版本
    // pub fn read_file(self, file_path: String) -> Result<String,io::Error> {
    //     let mut f = File::open(file_path)?; // 如果出现问题就 返回
    //
    //     let mut data = String::new();
    //     f.read_to_string(&mut data)?;
    //     Ok(data)
    // }

    // pub fn read_file(self, file_path: String) -> Result<String,io::Error> {
    //     let mut f = match File::open(file_path) {
    //         Err(e) => return Err(e),
    //         Ok(f) => f,
    //     };
    //
    //     let mut data = String::new();
    //     match f.read_to_string(&mut data) {
    //         Err(e) => Err(e),
    //         Ok(_) => Ok(data),
    //     }
    // }

    // 简化版本2
    pub fn read_file(self, file_path: String) -> Result<String,io::Error> {
        let mut result = String::new();
        File::open(file_path)?.read_to_string(&mut result)?;
        Ok(result)
    }
}

pub mod a {

}