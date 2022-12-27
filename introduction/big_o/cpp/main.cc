#include <iostream>
#include <vector>

using std::cout, std::endl;
using std::vector;

void print(vector<int> &nums) { // O(n)
    for (int n:nums) 
        cout << n + " ";
    cout << endl;
}

int main(){
    vector<int> numbers{1,2,3,4,5,6,7,8,9,10};
    print(numbers);
}

