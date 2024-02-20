use std::{env, fs};
use std::path::Path;

fn main()
{
    let args: Vec<String> = env::args().collect();
    let path : String;
    if args.len() < 2
    {

        path = "".to_string();//??
    }
    else {
        path = args[1].clone();
    }
    let it = match fs::read_dir(Path::new(&path)){
        Ok(directories) => directories,
        Err(_) => panic!("Path doesn't exist")
    };
    for i in it
    {
        let dir_entry = match i{
            Ok(dir) => dir,
            Err(_) => panic!("Directory name error")
        };
        println!("{}", match dir_entry.file_name().into_string(){
            Ok(string) => string,
            Err(_) => panic!("Directory name couldn't be converted into string")
        });
    }
}

