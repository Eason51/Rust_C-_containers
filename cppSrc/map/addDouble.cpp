#include <map>

int main(){

    for(int counter = 0; counter != 10; ++counter)
    {

        std::map<float, int> map;
        for(int i = 0; i != 1000000; ++i)
        {
            double num = static_cast<double>(i);
            map.insert({num, 0});
        }
    }
}