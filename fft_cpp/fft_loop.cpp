#include <array>
#include <iostream>
#include <vector>
#include <complex>

/*
 Iterative Fast Fourier Transformation for polynomial multiplication:

https://www.geeksforgeeks.org/iterative-fast-fourier-transformation-polynomial-multiplication/
 */

void show_result(std::vector<std::complex<double>>& result) {
    for(auto& v: result) {
        std::cout << v << ",";
    }
    std::cout << std::endl;
}

std::size_t bit_reverse(std::size_t x, std::size_t log2_n) {
    std::size_t n = 0;
    for(std::size_t i = 0; i < log2_n; ++i) {
        n <<= 1;
        n |= (x & 1);
        x >>= 1;
    }

    return n;
}

// is_reverseがtrueなら逆フーリエ変換を実行(規格化も行う)
void fft_loop_ref(std::vector<std::complex<double>>& array, bool is_reverse) {
    auto n = array.size();
    auto log2_n = std::log2(n);
    auto len_array = std::complex((double)array.size(), 0.0);

    // bit reversal of th given array
    auto buff = array;
    for(std::size_t i = 0; i < n; ++i) {
        auto idx_rev = bit_reverse(i, log2_n);
        array[i] = buff[idx_rev];
        if(is_reverse) {
            array[i] /= len_array;
        }
    }

    for(std::size_t s = 1; s <= log2_n; ++s) {
        auto m = 1 << s;
        auto m_2 = m >> 1;
        std::complex<double> w(1.0, 0.0);

        std::complex<double> w_m = std::exp(std::complex<double>(0, M_PI / m_2));
        if(is_reverse) {
            w_m = std::conj(w_m);
        }

        for(std::size_t j = 0; j < m_2; ++j) {
            for(std::size_t k = j; k < n; k += m) {

                auto t = w * array[k + m_2];
                auto u = array[k];

                array[k] = u + t;
                array[k + m_2] = u - t;
            }
            w *= w_m;
        } 
    }
}

int main(void) {
    std::vector<std::complex<double>> array = {1.0, 2.0, 3.0, 4.0};
    std::vector<std::complex<double>> array_res(4);

    // フーリエ変換
    auto res_2 = array;
    fft_loop_ref(res_2, false);
    show_result(res_2);

    // 逆フーリエ変換
    fft_loop_ref(res_2, true);
    show_result(res_2);
}
