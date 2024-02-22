use std::{env, fs};
use std::fs::{DirEntry, read_dir};
use std::path::Path;
use std::vec::Vec;
use std::env::current_dir;
use std::ptr::read;
fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s|!s.starts_with("."))
        .unwrap_or(false)
}


fn ls_r(path: String, tabs_nr: u8)
{
    let it = match fs::read_dir(Path::new(&path)){
        Ok(directories) => directories,
        Err(_) => panic!("Path doesn't exist")
    };
    let tabs = "\t".repeat(tabs_nr as usize);
    println!("{}--Names--",tabs);
    for i in it
    {
        let dir_entry = match i{
            Ok(dir) => dir,
            Err(_) => panic!("Directory name error")
        };
        let name :String  = match dir_entry.file_name().into_string() {
            Ok(string) => string,
            Err(_) => panic!("Directory name couldn't be converted into string")
        };
        if name.contains(".")
        {
            println!("{}{}", tabs, name)
        }
        else
        {
            println!("\n{}Printing {} directory:", tabs, name);
            let new_path: String = format!("{}\\{}", path, name);
            ls_r(new_path, tabs_nr + 1);
        }
    }

    // for i in it
    // {
    //     let dir_entry = match i{
    //         Ok(dir) => dir,
    //         Err(_) => panic!("Directory name error")
    //     };
    //     println!("{}", match dir_entry.file_name().into_string(){
    //         Ok(string) => string,
    //         Err(_) => panic!("Directory name couldn't be converted into string")
    //     })
    // }
}

fn main()
{
    let args: Vec<String> = env::args().collect();
    let (mut switch_l, mut switch_recursive, mut switch_a, mut switch_h, mut switch_sort_n, mut switch_sort_s, mut switch_sort_t, mut switch_sort_v, mut switch_sort_x, mut switch_sort_reverse):
        (bool, bool, bool, bool, bool, bool, bool, bool, bool, bool);
    let mut paths: Vec<String> = Vec::new();

    for arg in args {
        if arg == "-a" || arg == "--all" {
            switch_a = true;
        } else if arg == "-l" {
            switch_l = true;
        } else if arg == "-R" || arg == "--recursive" {
            switch_recursive = true;
        } else if arg == "-h" || arg == "--human-readable" {
            switch_h = true;
        } else if arg == "-U" || arg == "--sort=none" {
            switch_sort_n = true;
        } else if arg == "-S" || arg == "--sort=size" {
            switch_sort_s = true;
        } else if arg == "-t" || arg == "--sort=time" {
            switch_sort_t = true;
        } else if arg == "-v" || arg == "--sort=version" {
            switch_sort_v = true;
        } else if arg == "-X" || arg == "--sort=extension" {
            switch_sort_x = true;
        } else if arg == "-r" || arg == "--reverse" {
            switch_sort_reverse = true;
        } else {
            paths.push(arg);
        }
    }
    if paths.len() == 0 {
        paths.push(current_dir().unwrap().display().to_string())
    }
    for path in paths { // for all paths given as arguments to ls
        println!("{}aaaaa", path);
        let mut read_dir = match fs::read_dir(Path::new(&path)) {
            Ok(read_dir) => read_dir,
            Err(_) => panic!("Path doesn't exist")
        };
        //     for i in read_dir
        //     {
        //         let dir_entry = match i{
        //             Ok(dir) => dir,
        //             Err(_) => panic!("Directory name error")
        //         };
        //         println!("{}", match dir_entry.file_name().into_string(){
        //             Ok(string) => string,
        //             Err(_) => panic!("Directory name couldn't be converted into string")
        //         });
        //     }
        let mut vec_of_dir: Vec<DirEntry> = Vec::new();
        for i in read_dir {
            let dir_entry = match i {
                Ok(dir) => dir,
                Err(_) => panic!("Error in file")
            };
            if is_not_hidden(&dir_entry)
            {
                vec_of_dir.push(dir_entry);
            }
        }
        for dir in vec_of_dir {
            println!("{}", match dir.file_name().into_string() {
                Ok(string) => string,
                Err(_) => panic!("Directory name couldn't be converted into string")
            })
        };
    }
        //ls_r(path, 0); tak wywolac ale usunac wszystko co jest na dole
}

