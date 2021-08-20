use std::collections::BTreeMap;
use float_ord::FloatOrd;

fn main(){

    for _ in 0..10
    {
    
        let mut map: BTreeMap<FloatOrd<f64>, i32> = BTreeMap::new();
        for i in 0..1000000
        {
            let orderedFloat: FloatOrd<f64> = FloatOrd(i as f64);
            map.insert(orderedFloat, 0i32);
        }

    }
}