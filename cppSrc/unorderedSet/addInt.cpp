#include <unordered_set>

int main(){

    for(int counter = 0; counter != 10; ++counter)
    {
        std::unordered_set<int> set;
        for(int i = 0; i != 1000000; ++i)
        {
            set.insert(i);
        }
    }
}