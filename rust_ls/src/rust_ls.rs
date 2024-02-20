use std::{env, fs};
use std::path::Path;

fn main()
{
    let args: Vec<String> = env::args().collect();
    let (mut switch_l, mut switch_r, mut switch_a, mut switch_h, mut switch_sort_n, mut switch_sort_s, mut switch_sort_t, mut switch_sort_v, mut switch_sort_x) : (bool, bool, bool, bool, bool, bool, bool, bool, bool);
    let path : String;

    for arg in args{
        if arg == "-a" || arg == "--all"{
            switch_a = true;
        }
        else if arg == "-l"{
            switch_l = true;
        }
        else if arg == "-r" || arg == "--reverse"{
            switch_r = true;
        }
        else if arg == "-a" || arg == "--all"{
            switch_h = true;
        }
        else if arg == "-U" || arg == "--sort=none"{
            switch_sort_n = true;
        }
        else if arg == "-S" || arg == "--sort=size"{
            switch_sort_s = true;
        }
        else if arg == "-t" || arg == "--sort=time"{
            switch_sort_t = true;
        }
        else if arg == "-v" || arg == "--sort=version"{
            switch_sort_v = true;
        }
        else if arg == "-X" || arg == "--sort=extension"{
            switch_sort_x = true;
        }
    }


    if args.len() < 2
    {

        path = ".".to_string();
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

