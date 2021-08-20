// // use cpu_time::ProcessTime;

// fn main(){

//     let mut vec: Vec<i32> = Vec::new();

//     // let start = ProcessTime::now();

//     for i in 0..100000000
//     {
//         vec.push(i);
//     }

//     // let runningTime = start.elapsed();
//     // println!("{:?}", runningTime);
// }


use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Create a path to the desired file
    let path = Path::new("prideAndPrejudice.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => ()
    }

    let v: Vec<String> = s.split_whitespace().map(String::from).collect();
    for i in 0..200
    {
        println!("{}", v[i]);
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}
