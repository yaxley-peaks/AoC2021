#include <fstream>
#include <iostream>
#include <string>
#include <vector>

int part1(std::vector<int> nums){
    int cnt = 0;
    for (int i = 1; i < nums.size(); i++) {
        if (nums[i] > nums[i - 1])
            cnt++;
    }
    return cnt;
}

int part2(std::vector<int> nums){
    int cnt = 0;
    for(int i = 3; i < nums.size(); i++){
        if(nums[i] > nums[i-3]){
            cnt++;
        }
    }
    return cnt;
}
int main() {
    std::ifstream file("./input.txt");
    std::string word;
    std::vector<int> nums;
    while (std::getline(file, word)) {
        nums.push_back(std::stoi(word));
    }

    std::cout << part1(nums) << std::endl;
    std::cout << part2(nums) << std::endl;
}