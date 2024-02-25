extern crate chrono;

use std::{env, fs};
use std::collections::HashSet;
use std::fs::{DirEntry};
use std::os::windows::fs::MetadataExt;
use std::path::Path;
use std::vec::Vec;
use chrono::{DateTime, Utc};


fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s|!s.starts_with("."))
        .unwrap_or(false)
}

fn sort(vec_of_entries: &mut Vec<DirEntry>, flag: char)
{
    let mut i: usize = 1;
    if flag == 't'
    {
        while i < vec_of_entries.len()
        {
            let mut j = i;
            let prev = fs::metadata(vec_of_entries[j-1].file_name().into_string().unwrap()).unwrap().created().unwrap();
            let next = fs::metadata(vec_of_entries[j].file_name().into_string().unwrap()).unwrap().created().unwrap();
            while j > 0 &&  prev > next {
                vec_of_entries.swap(j, j-1);
                j -= 1;
            }
            i += 1;
        }
    }
    else if flag == 's'
    {
        while i < vec_of_entries.len()
        {
            let mut j = i;
            let prev = fs::metadata(vec_of_entries[j-1].file_name().into_string().unwrap()).unwrap().file_size();
            let next = fs::metadata(vec_of_entries[j].file_name().into_string().unwrap()).unwrap().file_size();
            while j > 0 &&  prev > next {
                vec_of_entries.swap(j, j-1);
                j -= 1;
            }
            i += 1;
        }
    }
    else if flag == 'x'
    {
        while i < vec_of_entries.len()
        {
            let mut j = i;
            let mut c1: &str = "";
            let mut c2: &str = "";
            let prev = vec_of_entries[j-1].file_name().into_string().unwrap();
            if prev.contains(".")
            {
                let n = prev.find(".").unwrap();
                c1 = &prev[n..];
            }
            let next = vec_of_entries[j].file_name().into_string().unwrap();
            if next.contains(".")
            {
                let n = next.find(".").unwrap();
                c2 =  &next[n..];
            }
            while j > 0 &&  c1 > c2 {
                vec_of_entries.swap(j, j-1);
                j -= 1;
            }
            i += 1;
        }
    }
    else
    {
        while i < vec_of_entries.len() - 1
        {
            let mut j = i;
            let prev: String = vec_of_entries[j-1].file_name().into_string().unwrap();
            let next: String = vec_of_entries[j].file_name().into_string().unwrap();
            while j > 0 && prev > next {
                vec_of_entries.swap(j, j-1);
                j -= 1;
            }
            i += 1;
        }
    }
}


fn print_vec_args(vec_of_dir: &mut Vec<DirEntry>, set_of_switches:HashSet<&str> ) ->() {
    if set_of_switches.contains("switch_sort_n")
    {
        sort(vec_of_dir, 'n');
    } else if set_of_switches.contains("switch_sort_s") {
        sort(vec_of_dir, 's');
    }
    else if set_of_switches.contains("switch_sort_t") {
        sort(vec_of_dir, 't');
    }
    else if set_of_switches.contains("switch_sort_v") {
        sort(vec_of_dir, 'x');
    }
    else if set_of_switches.contains("switch_sort_x") {
        sort(vec_of_dir, 'x');
    }
    else if set_of_switches.contains("switch_sort_reverse") {
        println!("nie dziala jeszcze");
    }
    for dir_entry in vec_of_dir{
        //switch l w forze
        //recursive a mainie
        //human read w forze
        if !set_of_switches.contains("switch_l") {
            print!("{} ", dir_entry.file_name().into_string().unwrap());
        }
        else{
            let metadata = dir_entry.metadata().unwrap();
            let file_size = metadata.len();
            let last_modified_sys_time = metadata.modified().unwrap();
            let last_modified : DateTime<Utc> = last_modified_sys_time.into();
            let file_name = dir_entry.file_name().into_string().unwrap();

            if env::consts::OS == "windows"{
                let permissions = metadata.permissions().readonly();
                if set_of_switches.contains("switch_h") {
                    print!("{}  {}  {}  {}",
                           permissions, file_size, last_modified, file_name
                    );
                } else {
                    print!("{}  {}  {}  {} ",
                           permissions, file_size, last_modified, file_name
                    );
                }
            }
            else{
                /*let permissions = metadata.permissions().mode();
                if set_of_switches.contains("switch_h") {
                    print!("{}  {}  {}  {}",
                           permissions, file_size, last_modified, file_name
                    );
                } else {
                    print!("{}  {}  {}  {} ",
                           permissions, file_size, last_modified, file_name
                    );
                }*/
            }
        }
    }
    println!();
}

fn ls_r(path: &String)
{
    let it = match fs::read_dir(Path::new(&path)) {
        Ok(directories) => directories,
        Err(_) => panic!("Path doesn't exist")
    };
    let mut paths: Vec<String> = Vec::new();
    println!("{}:\n", path);
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
            println!("{}", name)
        }
        else
        {
            let new_path: String = format!("{}\\{}", path, name);
            paths.push(new_path);
        }

    }
    for i in &paths
    {
        ls_r(i);
    }
}

fn main()
{
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
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
    if switches.contains("switch_recursive"){
        for path in paths { // for all paths given as arguments to ls
            ls_r(&path);
        }
    }
    else {
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

            for dir in &vec_of_dir {
                println!("{}", match dir.file_name().into_string() {
                    Ok(string) => string,
                    Err(_) => panic!("Directory name couldn't be converted into string")
                })
            };
        }
    }


}

//TODO
// 0.zmienic na wypisywanie w funkcji
// 1.zmienić ls_r na dodawanie do wektora
// 2.dodac a
// 3.dodac h
// 4.dodac l
// 5.dodac sort
//