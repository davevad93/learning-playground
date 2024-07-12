#include <iostream>
#include <vector>
#include <chrono>
#include "ThreadPool.h"

void exampleTask(int n) {
    std::this_thread::sleep_for(std::chrono::milliseconds(100 * n));
    std::cout << "Task " << n << " is complete.\n";
}

int main() {
    ThreadPool pool(4);

    std::vector<std::future<void>> results;

    for (int i = 0; i < 8; ++i) {
        results.emplace_back(
            pool.enqueue(exampleTask, i)
        );
    }

    for (auto &&result : results)
        result.get();

    std::cout << "All tasks completed.\n";

    return 0;
}
