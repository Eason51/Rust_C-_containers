use std::collections::BTreeSet;


fn main(){
    

    for _ in 0..10
    {
        let mut set: BTreeSet<i32> = BTreeSet::new();
        for i in 0..1000000
        {
            set.insert(i);
        }
    }
}