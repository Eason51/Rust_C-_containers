use std::collections::HashSet;
use std::iter::FromIterator;




fn main(){
    

    for _ in 0..10
    {
        let vec: Vec<i32> = (0..1000000).collect();
        let mut set: HashSet<i32> = HashSet::from_iter(vec.iter().cloned());
        for i in 0..1000000
        {
            set.remove(&i);
        }
    }
}