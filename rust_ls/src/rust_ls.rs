use std::{env, fs};
use std::fs::{DirEntry};
use std::path::Path;
use std::vec::Vec;

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s|!s.starts_with("."))
        .unwrap_or(false)
}

fn main()
{
    let args: Vec<String> = env::args().collect();
    let path : String;
    if args.len() < 2
    {

        path = ".".to_string();//??
    }
    else {
        path = args[1].clone();
    }
    let it = match fs::read_dir(Path::new(&path)){
        Ok(directories) => directories,
        Err(_) => panic!("Path doesn't exist")
    };
    // WalkDir::new(Path::new(&path))
    //     .into_iter()
    //     .filter_entry(|e| !is_not_hidden(e))
    //     .filter_map(|v| v.ok())
    //     .for_each(|x| println!("{:?}", x.file_name().to_os_string()));
    let mut vec_of_dir: Vec<DirEntry>=Vec::new();
    for i in it {
        let dir_entry=match i  {
            Ok(dir)=>dir,
            Err(_)=>panic!("Error in file")
        };
        if is_not_hidden(&dir_entry)
        {
            vec_of_dir.push(dir_entry);
        }
    }
    for dir in vec_of_dir{
        println!("{}", match dir.file_name().into_string(){
                     Ok(string) => string,
                     Err(_) => panic!("Directory name couldn't be converted into string")
                 })
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

