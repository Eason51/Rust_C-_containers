use std::mem;

fn pushSize<T: Copy>(v: &mut Vec<T>, elem: T, size: i32){

    for _ in 0..size
    {
        v.push(elem);
    }
    println!("length: {}  capacity: {}", v.len(), v.capacity());
}

fn main(){
    let mut intVec: Vec<i32> = Vec::new();
    println!("stack size: {}", mem::size_of_val(&intVec));
    pushSize(&mut intVec, 0i32, 0);
    let mut size: i32 = 1;
    for _ in 0..6
    {
        size *= 10;
        pushSize(&mut intVec, 0i32, size);
    }

    let mut doubleVec: Vec<f64> = Vec::new();
    println!("stack size: {}", mem::size_of_val(&doubleVec));
    pushSize(&mut doubleVec, 0.0f64, 0);
    let mut size: i32 = 1;
    for _ in 0..6
    {
        size *= 10;
        pushSize(&mut doubleVec, 0.0f64, size);
    }


    let mut strVec: Vec<&str> = Vec::new();
    println!("stack size: {}", mem::size_of_val(&strVec));
    println!("length: {}  capacity: {}", strVec.len(), strVec.capacity());
    let mut size: i32 = 1;
    for _ in 0..6
    {
        size *= 10;
        for _j in 0..size
        {
            strVec.push("a");
        }
        println!("length: {}  capacity: {}", strVec.len(), strVec.capacity());
    }
}