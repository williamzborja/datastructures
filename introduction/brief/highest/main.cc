#include <algorithm>
#include <iostream>
#include <string>
#include <unordered_map>

using std::string, std::min_element, std::unordered_map;

char highest_freq(string &input) {
  unordered_map<char, int> freq; // hash map
  for (auto c : input)           // fill map with frequencies
    freq[c] += 1;

  // get max key and value pair
  auto pair = min_element(freq.begin(), freq.end());
  return pair->first; // return key
}

int main() {
  string input = "aaabbc";
  auto result = highest_freq(input);
  printf("%c\n", result);
}
