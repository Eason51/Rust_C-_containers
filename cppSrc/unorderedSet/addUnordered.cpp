#include <unordered_set>
#include <vector>

int main(){
    
    std::vector<int> v = {4269, 4712, 6183, 2764, 1747, 1095, 3740, 3758, 6612, 1256, 373, 346, 411, 765};
    std::vector<int> v2 = {9520, 3605, 4915, 8217, 8018, 7952, 7392, 8185, 9883, 8271, 7851, 6111, 5300};
    std::vector<int> v3 = {6481, 3751, 782, 5187, 1592, 6660, 2065, 1297, 8252, 5048, 2785, 7438, 7963};
    std::vector<int> v4 = {2867, 9986, 7578, 2376, 8589, 2015, 4210, 9230, 2402, 5373, 6643, 1775, 9755};
    std::vector<int> v5 = {3391, 3, 7912, 223, 349, 340, 7179, 4058, 9918, 7750, 5964, 2087, 511, 478};

    for(int counter = 0; counter != 100000; ++counter)
    {
        std::unordered_set<int> set;
        for(int i = 0; i != v.size(); ++i)
        {
            set.insert(v[i]);
        }
        for(int i = 0; i != v2.size(); ++i)
        {
            set.insert(v2[i]);
        }
        for(int i = 0; i != v3.size(); ++i)
        {
            set.insert(v3[i]);
        }
        for(int i = 0; i != v4.size(); ++i)
        {
            set.insert(v4[i]);
        }
        for(int i = 0; i != v5.size(); ++i)
        {
            set.insert(v5[i]);
        }
    }
}