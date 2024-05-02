#include <iostream>
#include <vector>

// time: O(N^2)
template<class T>
std::vector<T> convolution_1(const std::vector<T>& array_1, const std::vector<T>& array_2) {
    auto [len_1, len_2] = std::pair(array_1.size(), array_2.size());
    std::vector<T> result(len_1 + len_2 - 1);

    for(std::size_t i = 0; i < len_1; ++i) {
        for(std::size_t j = 0; j < len_2; ++j) {
            result[i + j] += array_1[i] * array_2[j];
        }
    }

    return result;
}

template<class T>
void show_result(const std::vector<T>& result) {
    for(auto& v: result) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
}

int main(void) {
    auto case_1 = std::vector{1, 2, 3, 4};
    auto case_2 = std::vector{5, 6, 7, 8};
    auto res_1 = convolution_1(case_1, case_2);

    show_result(res_1);
}
