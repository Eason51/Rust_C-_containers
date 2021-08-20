use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use cpu_time::ProcessTime;

fn main() {

    let path = Path::new("int.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => ()
    }

    let mut v: Vec<String> = s.split_whitespace().map(String::from).collect();

    let mut it = v.iter_mut();
    for val in it
    {
        *val = val.chars().filter(|c| c.is_alphanumeric()).collect();
    }

    let mut map: HashMap<String, i32> = HashMap::new();

    let start = ProcessTime::now();

    for i in 0..v.len()
    {
        if let Some(val) = map.get_mut(&v[i])
        {
            *val += 1;
        }
        else
        {
            map.insert(v[i].clone(), 1i32);
        }
    }

    let runningTime = start.elapsed();
    println!("{:?}", runningTime);
}
