pub mod blockchain;
pub mod dir_iterator;
pub mod file;
pub mod filesystem;
pub mod internet;

use crate::dir_iterator::DirIterator;
use crate::file::FileWriter;
use crate::filesystem::*;
use std::io::{BufWriter, Write};

#[no_mangle]
pub unsafe extern "C" fn run() -> u32 {
    create_dir("testFolder").unwrap();
    {
        let test_case = "Hi wasmer.\nGood to see you.\n".as_bytes();
        let file = FileWriter::new("testFolder/test.txt").unwrap();
        let mut writer = BufWriter::new(file);
        writer.write(test_case).unwrap();
        writer.flush().unwrap();
    }

    {
        let test_case = "cmake_minimum_required(VERSION 3.9)\n
                        \n
                        add_subdirectory(utils)\n
                        add_subdirectory(crypto)\n
                        add_subdirectory(logging)\n
                        add_subdirectory(storage)\n
                        add_subdirectory(virtualMachine)\n
                        add_subdirectory(internet)\n
                        add_subdirectory(messenger)"
            .as_bytes();
        let file = FileWriter::new("testFolder/CMakeLists.txt").unwrap();
        let mut writer = BufWriter::new(file);
        writer.write(test_case).unwrap();
        writer.flush().unwrap();
    }

    create_dir("testFolder/crypto").unwrap();
    create_dir("testFolder/logging").unwrap();
    create_dir("testFolder/utils").unwrap();
    create_dir("testFolder/virtualMachine").unwrap();
    create_dir("testFolder/messenger").unwrap();
    create_dir("testFolder/internet").unwrap();

    let mut res = true;

    {
        let mut iterator = DirIterator::new("testFolder", false);

        let expected = [
            "CMakeLists.txt",
            "crypto",
            "internet",
            "logging",
            "messenger",
            "test.txt",
            "utils",
            "virtualMachine"
        ];

        while let Some(entry) = iterator.next() {
            let path = &entry.name;
            if !expected.contains(&path.as_str()) {
                res = false;
            }
        }
    }

    remove_filesystem_entry("testFolder").unwrap();

    if res {
        return 1;
    }
    return 0;
}
