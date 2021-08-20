#include <unordered_set>

int main(){
    for(int counter = 0; counter != 10; ++counter)
    {
        std::unordered_set<double> set;
        for(int i = 0; i != 1000000; ++i)
        {
            double num = static_cast<double>(i);
            set.insert(num);
        }
    }
}