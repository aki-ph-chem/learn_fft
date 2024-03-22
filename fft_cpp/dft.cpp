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

// is_reverse がtrueのとき逆フーリエ変換を実行する
std::vector<std::complex<double>> dft_2(const std::vector<std::complex<double>>& array, bool is_reverse) {
    auto n = array.size();
    std::vector<std::complex<double>> result(n, {0.0, 0.0});

    for(auto k = 0; k < n; ++k) {
        for(auto l = 0; l < n; ++l) {
            auto rot_factor = std::exp(std::complex(0.0, -2.0 * M_PI * (k * l) / n ));
            if(is_reverse) {
                rot_factor = std::conj(rot_factor) / std::complex((double)n, 0.0);
            }

            result[k] += rot_factor * array[l];
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
    /*
    std::vector<double> array_1 = {1.2, 1.3, 1.4};
    auto res_1 = dft(array_1);
    show_result(res_1);
    */

    std::vector<std::complex<double>> array_2 = {1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9};
    show_result(array_2);
    auto array_2_ft = dft_2(array_2, false); 
    show_result(array_2_ft);
    auto array_2_ft_rev_ft = dft_2(array_2_ft, true); 
    show_result(array_2_ft_rev_ft);
}
