#include <fstream>
#include <vector>
#include <string>
#include <iostream>
#include <algorithm>
#include <unordered_map>

#include "boost/timer/timer.hpp"
using boost::timer::cpu_timer;
using boost::timer::cpu_times;
using boost::timer::nanosecond_type;

int main(){
    

    std::ifstream ifs("int.txt");
    std::vector<int> v;
    std::string str;
    while(ifs >> str)
    {
        str.erase(std::remove_if(str.begin(), str.end(),
            [](char c){return !isalnum(c);}
        ), str.end());
        v.push_back(stoi(str));
    }

    ifs.close();
    cpu_timer timer;

    std::unordered_map<int, int> map;
    for(auto it = v.begin(); it != v.end(); ++it)
    {
        auto mapIt = map.find(*it);
        if(mapIt == map.end())
        {
            map.insert({*it, 1});
        }
        else
        {
            mapIt->second += 1;
        }
    }

    cpu_times const runningTime(timer.elapsed());
    nanosecond_type const total(runningTime.system + runningTime.user);
    std::cout << total << std::endl;

}