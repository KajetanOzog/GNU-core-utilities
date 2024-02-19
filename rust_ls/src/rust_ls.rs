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
    let it = fs::read_dir(Path::new(&path)).unwrap();
    for i in it
    {
        println!("{}", i.unwrap().file_name().into_string().unwrap());
    }
}

