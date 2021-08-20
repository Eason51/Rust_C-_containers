use std::collections::BTreeSet;
use float_ord::FloatOrd;


fn main(){
    

    for _ in 0..10
    {
        let mut set: BTreeSet<FloatOrd<f64>> = BTreeSet::new();
        for i in 0..1000000
        {
            let orderedFloat: FloatOrd<f64> = FloatOrd(i as f64);
            set.insert(orderedFloat);
        }
    }
}