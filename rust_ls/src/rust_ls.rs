use std::fs;

fn main() {
    let it = fs::read_dir(".").unwrap();
    for i in it
    {
        println!("{}", i.unwrap().file_name().into_string().unwrap());
    }
}

