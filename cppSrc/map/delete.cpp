#include <map>
#include <vector>
#include <utility>

int main(){

    for(int counter = 0; counter != 10; ++counter)
    {
        std::vector<std::pair<int, int>> v(1000000);
        for(int i = 0; i != v.size(); ++i)
        {
            v[i].first = i;
        }

        std::map<int, int> map(v.begin(), v.end());
        for(int i = 0; i != v.size(); ++i)
        {
            map.erase(i);
        }
    }
}