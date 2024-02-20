use std::{env, fs};
use std::path::Path;

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
            let new_patch: String = format!("{}\\{}", path, name);
            ls_r(new_patch, tabs_nr + 1);
        }
    }
}

fn main()
{
    let args: Vec<String> = env::args().collect();
    let (switch_l, switch_r, switch_a, switch_h) : (bool, bool, bool, bool);
    let path : String;
    if args.len() < 2
    {

        path = ".".to_string();
    }
    else
    {
        path = args[1].clone();
    }
    //ls_r(path, 0); tak wywolac ale usunac wszystko co jest na dole
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

