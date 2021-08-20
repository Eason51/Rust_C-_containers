use std::mem;

fn pushChar(v: &mut String, elem: char, size: i32){

    for _ in 0..size
    {
        v.push(elem);
    }
    println!("length: {}  capacity: {}", v.len(), v.capacity());
}

fn pushString(v: &mut String, elem: & String, size: i32){
    
    for _ in 0..size
    {
        v.push_str(elem)
    }
    println!("length: {} capacity: {}", v.len(), v.capacity())
}



fn main(){
    let mut string = String::new();
    println!("stack size: {}", mem::size_of_val(&string));
    pushChar(&mut string, 'a', 0);
    let mut size: i32 = 1;
    for _ in 0..6
    {
        size *= 10;
        pushChar(&mut string, 'a', size);
    }

    let mut string2 = String::new();
    println!("stack size: {}", mem::size_of_val(&string2));
    pushString(& mut string2, &String::new(), 0);
    let mut size: i32 = 1;
    for _ in 0..9
    {
        size *= 3;
        let newString: String = "a".repeat(size as usize);
        pushString(& mut string2, &newString, size);
    }
}