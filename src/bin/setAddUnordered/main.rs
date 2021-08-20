use std::collections::BTreeSet;


fn main(){
    
    let v: Vec<i32> = vec![4269, 4712, 6183, 2764, 1747, 1095, 3740, 3758, 6612, 1256, 373, 346, 411, 765];
    let v2: Vec<i32> = vec![9520, 3605, 4915, 8217, 8018, 7952, 7392, 8185, 9883, 8271, 7851, 6111, 5300];
    let v3: Vec<i32> = vec![6481, 3751, 782, 5187, 1592, 6660, 2065, 1297, 8252, 5048, 2785, 7438, 7963];
    let v4: Vec<i32> = vec![2867, 9986, 7578, 2376, 8589, 2015, 4210, 9230, 2402, 5373, 6643, 1775, 9755];
    let v5: Vec<i32> = vec![3391, 3, 7912, 223, 349, 340, 7179, 4058, 9918, 7750, 5964, 2087, 511, 478];

    
    for _ in 0..100000
    {        
        let mut set: BTreeSet<i32> = BTreeSet::new();
        for i in 0..v.len()
        {            
            set.insert(v[i]);
        }
        for i in 0..v2.len()
        {            
            set.insert(v2[i]);
        }
        for i in 0..v3.len()
        {            
            set.insert(v3[i]);
        }
        for i in 0..v4.len()
        {            
            set.insert(v4[i]);
        }
        for i in 0..v5.len()
        {            
            set.insert(v5[i]);
        }
    }
}