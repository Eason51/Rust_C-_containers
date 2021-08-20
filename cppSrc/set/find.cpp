#include <set>
#include <vector>
#include <numeric>

int main(){

    for(int counter = 0; counter != 10; ++counter)
    {
        std::vector<int> v(1000000);
        std::iota(std::begin(v), std::end(v), 0);

        std::set<int> set(v.begin(), v.end());
        for(int i = 0; i != v.size(); ++i)
        {
            set.find(i);
        }
    }
}