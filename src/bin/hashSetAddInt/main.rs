use std::collections::HashSet;


fn main(){
    

    for _ in 0..10
    {
        let mut set: HashSet<i32> = HashSet::new();
        for i in 0..1000000
        {
            set.insert(i);
        }
    }
}