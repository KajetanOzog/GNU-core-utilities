use std::{env, fs};
use std::path::Path;

fn main()
{
    let args: Vec<String> = env::args().collect();
    if args.len() < 2
    {
        println!("Chryste Panie blad");
        std::process::exit(1);
    }
    let path = &args[1];
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

