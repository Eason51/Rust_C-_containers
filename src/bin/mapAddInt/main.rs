use std::collections::BTreeMap;


fn main(){
    

    for _ in 0..10
    {
        let mut map: BTreeMap<i32, i32> = BTreeMap::new();
        for i in 0..1000000
        {
            map.insert(i, 0i32);
        }
    }
}