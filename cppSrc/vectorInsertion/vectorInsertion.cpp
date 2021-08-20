#include <iostream>
#include <vector>

template <typename T>
void pushSize(std::vector<T> &v, T elem, int size){
    for(int i = 0; i != size; ++i)
        v.push_back(elem);
    std::cout << "length: " << v.size() << "  capacity: " << v.capacity() << std::endl;
}

int main(){
    std::vector<int> intVec = std::vector<int>();
    std::cout << "stack size: " << sizeof(intVec) << std::endl;    
    pushSize<int>(intVec, 0, 0);
    int size = 1;
    for(int i = 0; i != 6; ++i)
    {
        size *= 10;
        pushSize<int>(intVec, 0, size);
    }

    std::vector<double> doubleVec = std::vector<double>();
    std::cout << "stack size: " << sizeof(doubleVec) << std::endl;    
    pushSize<double>(doubleVec, 0, 0);
    size = 1;
    for(int i = 0; i != 6; ++i)
    {
        size *= 10;
        pushSize<double>(doubleVec, 0.0, size);
    }

    std::vector<const char*> strVec = std::vector<const char*>();
    std::cout << "stack size: " << sizeof(strVec) << std::endl;
    const char* str = "a";
    pushSize<const char*>(strVec, str, 0);
    size = 1;
    for(int i = 0; i != 6; ++i)
    {
        size *= 10;
        
        pushSize<const char*>(strVec, str, size);
    }
}