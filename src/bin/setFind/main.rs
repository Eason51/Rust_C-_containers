use std::collections::BTreeSet;

use std::iter::FromIterator;




fn main(){
    

    for _ in 0..10
    {
        let vec: Vec<i32> = (0..1000000).collect();
        let set: BTreeSet<i32> = BTreeSet::from_iter(vec.iter().cloned());
        for i in 0..1000000
        {
            set.get(&i);
        }
    }
}