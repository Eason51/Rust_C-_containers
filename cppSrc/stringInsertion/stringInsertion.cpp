#include <iostream>
#include <string>

 
void pushChar(std::string &str, char elem, int size){
    for(int i = 0; i != size; ++i)
        str.push_back(elem);
    std::cout << "length: " << str.size() << "  capacity: " << str.capacity() << std::endl;
}

void pushString(std::string &str, const std::string &pushedStr, int size){
    for(int i = 0; i != size; ++i)
        str.append(pushedStr);
    std::cout << "length: " << str.size() << "  capacity: " << str.capacity() << std::endl;    
}

int main(){
    std::string str = std::string();
    std::cout << "stack size: " << sizeof(str) << std::endl;    
    pushChar(str, 'a', 0);
    int size = 1;
    for(int i = 0; i != 6; ++i)
    {
        size *= 10;
        pushChar(str, 0, size);
    }

    std::string str2 = std::string();
    std::string newString = std::string();
    std::cout << "stack size: " << sizeof(str) << std::endl;    
    pushString(str2, newString, 0);
    size = 1;
    for(int i = 0; i != 9; ++i)
    {
        size *= 3; // size is initially 1
        std::string newString = std::string(size, 'a');
        pushString(str2, newString, size); //str2 is the tested string
    }

}