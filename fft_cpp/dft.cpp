#include <cmath>
#include <iostream>
#include <vector>
#include <complex>
//#define DEBUG_PRINT

std::vector<std::complex<double>> dft(std::vector<double>& array) {
    auto n = array.size();
    std::vector<std::complex<double>> result(n,{1.0, 1.0});
    for(auto k = 0; k < n; ++k) {
        for(auto l = 0; l < n; ++l) {
            result[k] += std::exp(std::complex(0.0, -2.0 * M_PI * (k * l) / n )) * array[l];
        }
    }

    return result;
}

void show_result(std::vector<std::complex<double>>& result) {
    for(auto& v: result) {
        std::cout << v.real() << " + " << v.imag() << "i" << ",";
    }
    std::cout << std::endl;
}

int main(void) {
    std::vector<double> array_1 = {1.2, 1.3, 1.4};
    auto res_1 = dft(array_1);
    show_result(res_1);
}
