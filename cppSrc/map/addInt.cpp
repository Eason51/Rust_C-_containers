#include <map>

int main(){

    for(int counter = 0; counter != 10; ++counter)
    {
        std::map<int, int> map;
        for(int i = 0; i != 1000000; ++i)
        {
            map.insert({i, 0});
        }
    }
}