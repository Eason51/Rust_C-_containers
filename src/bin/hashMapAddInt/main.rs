use std::collections::HashMap;

fn main(){

    for _ in 0..10
    {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for i in 0..1000000
        {
            map.insert(i, 0i32);
        }
    }

}