#include <cmath>
#include <iostream>
#include <vector>
#include <complex>

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

// time: O(NlogN)
std::vector<std::complex<double>> conv_fft(std::vector<std::complex<double>>&& array_1, std::vector<std::complex<double>>&& array_2) {
    auto len_bit = [](auto n) {
        int length_bit = 0;
        while(n > 0) {
            n >>= 1;
            ++length_bit;
        }

        return length_bit;
    };

    auto [len_1, len_2] = std::pair(array_1.size(), array_2.size());
    auto len_total = len_1 + len_2 - 1;
    auto [len_diff_1, len_diff_2] = std::pair(0, 0);

    if((len_total & (len_total - 1)) != 0) {
        len_total = 1 << len_bit(len_total);
        len_diff_1 = (1 << len_bit(len_total)) - len_1;
        len_diff_2 = (1 << len_bit(len_total)) - len_2;
    } else {
        len_diff_1 = len_total - len_1;
        len_diff_2 = len_total - len_2;
    }

    auto diff_vec_1 = std::vector<std::complex<double>>(len_diff_1);
    auto diff_vec_2 = std::vector<std::complex<double>>(len_diff_2);

    array_1.insert(array_1.end(), diff_vec_1.begin(), diff_vec_1.end());
    array_2.insert(array_2.end(), diff_vec_2.begin(), diff_vec_2.end());

    fft_loop_ref(array_1, false);
    fft_loop_ref(array_2, false);

    auto result = std::vector<std::complex<double>>(len_total);
    for(std::size_t i = 0; i < array_1.size(); ++i) {
        result[i] = array_1[i] * array_2[i];
    }

    fft_loop_ref(result, true);

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
    auto case_1 = std::vector<std::complex<double>>{1, 2, 3, 4};
    auto case_2 = std::vector<std::complex<double>>{5, 6, 7, 8};
    auto res_1 = conv_fft(std::move(case_1), std::move(case_2));
    show_result(res_1);
}
