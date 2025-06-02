#include <algorithm>
#include <cassert>
#include <functional>
#include <iostream>
#include <mutex>
#include <numeric>
#include <thread>
#include <vector>

const int NUMBERS = 10000;
const size_t CHUNK_SIZE = 100;

std::mutex lock;

int vec_sum(std::vector<int> &v);

void worker_function(const std::vector<int> &v, size_t start, size_t end,
                     int &total);

int main() {
  std::vector<int> v;
  for (int i = 0; i < NUMBERS; i++) {
    v.push_back(i);
  }
  int sum = vec_sum(v);
  int expected = std::reduce(v.begin(), v.end());

  if (sum != expected) {
    std::cout << "vec_sum test failed: expected " << expected << " got " << sum
              << std::endl;
  } else {
    std::cout << "vec_sum test passed!" << std::endl;
  }
}

int vec_sum(std::vector<int> &v) {
  int total = 0;

  std::vector<std::thread> threads;
  for (size_t start = 0; start < v.size(); start += CHUNK_SIZE) {
    size_t end = std::min(start + CHUNK_SIZE, v.size());
    threads.emplace_back(worker_function, v, start, end, std::ref(total));
  }

  for (auto &t : threads) {
    if (t.joinable()) {
      t.join();
    }
  }

  return total;
}

void worker_function(const std::vector<int> &v, size_t start, size_t end,
                     int &total) {
  int local_total = 0;
  for (size_t i = start; i < end; i++) {
    local_total += v[i];
  }
  // simplest solution: lock mutex before mutating shared state
  std::lock_guard<std::mutex> guard(lock);
  total += local_total;
}
