use std::{env, fs};
use std::collections::HashSet;
use std::fs::{DirEntry};
use std::path::Path;
use std::vec::Vec;
use std::collections::HashSet;

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s|!s.starts_with("."))
        .unwrap_or(false)
}
fn print_vec_args(vec_of_dir: Vec<DirEntry>, set_of_switches:HashSet<&str> ) ->() {
    if set_of_switches.contains("switch_sort_n")
    {

    } else if set_of_switches.contains("switch_sort_s") {

    }
    else if set_of_switches.contains("switch_sort_t") {

    }
    else if set_of_switches.contains("switch_sort_v") {

    }
    else if set_of_switches.contains("switch_sort_x") {

    }
    else if set_of_switches.contains("switch_sort_reverse") {

    }
    for dir in vec_of_dir{
        //switch l w forze
        //recursive a mianie
        //human read w forze
        if set_of_switches.contains("switch_l"){

        }
        else if set_of_switches.contains("switch_h"){

        }
    }
}

fn ls_r(path: String, tabs_nr: u8)
{
    let it = match fs::read_dir(Path::new(&path)) {
        Ok(directories) => directories,
        Err(_) => panic!("Path doesn't exist")
    };
    let tabs = "\t".repeat(tabs_nr as usize);
    println!("{}--Names--", tabs);
    for i in it
    {
        let dir_entry = match i {
            Ok(dir) => dir,
            Err(_) => panic!("Directory name error")
        };
        let name: String = match dir_entry.file_name().into_string() {
            Ok(string) => string,
            Err(_) => panic!("Directory name couldn't be converted into string")
        };
        if name.contains(".")
        {
            println!("{}{}", tabs, name)
        } else {
            println!("\n{}Printing {} directory:", tabs, name);
            let new_path: String = format!("{}\\{}", path, name);
            ls_r(new_path, tabs_nr + 1);
        }
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

fn main()
{
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let mut switches : HashSet<&str> = HashSet::new();
    let mut paths: Vec<String> = Vec::new();
    let mut switches: HashSet<&str>=HashSet::new();

    for arg in args {
        if arg == "-a" || arg == "--all" {
            switches.insert("switch_a");
        } else if arg == "-l" || arg == "--long"{
            switches.insert("switch_l");
        } else if arg == "-R" || arg == "--recursive" {
            switches.insert("switch_recursive");
        } else if arg == "-h" || arg == "--human-readable" {
            switches.insert("switch_h");
        } else if arg == "-U" || arg == "--sort=none" {
            switches.insert("switch_sort_n");
        } else if arg == "-S" || arg == "--sort=size" {
            switches.insert("switch_sort_s");
        } else if arg == "-t" || arg == "--sort=time" {
            switches.insert("switch_sort_t");
        } else if arg == "-v" || arg == "--sort=version" {
            switches.insert("switch_sort_v");
        } else if arg == "-X" || arg == "--sort=extension" {
            switches.insert("switch_sort_x");
        } else if arg == "-r" || arg == "--reverse" {
            switches.insert("switch_sort_reverse");
        } else {
            paths.push(arg.clone());
        }
    }

    if paths.len() == 0 {
        paths.push(".".to_string())
    }

    let mut vec_of_dir: Vec<DirEntry> = Vec::new();

    for path in paths { // for all paths given as arguments to ls
        let mut read_dir = match fs::read_dir(Path::new(&fs::canonicalize(path).unwrap().display().to_string())) {
            Ok(read_dir) => read_dir,
            Err(_) => panic!("Path doesn't exist")
        };


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

//TODO
// 0.zmienic na wypisywanie w funkcji
// 1.zmienić ls_r na dodawanie do wektora
// 2.dodac a
// 3.dodac h
// 4.dodac l
// 5.dodac sort
//