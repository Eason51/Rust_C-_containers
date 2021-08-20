use std::collections::HashMap;
use std::iter::FromIterator;

struct Pair{
    key: i32,
    value: i32
}

impl Pair{
    fn toTuple(&self) -> (i32, i32){
        (self.key, self.value)
    }
}

fn main(){

    for _ in 0..10
    {
        let vec: Vec<(i32, i32)> = (0..1000000).map(
            |x| Pair{
                key: x,
                value: 0i32
            }.toTuple()
        ).collect();

        let map: HashMap<i32, i32> = HashMap::from_iter(vec.iter().cloned());
        for i in 0..1000000
        {
            map.get(&i);
        }
    }
}