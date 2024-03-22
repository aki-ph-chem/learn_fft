#include <iostream>
#include <vector>
#include <complex>
#include <cmath>

// is_reverseがtrueのとき逆フーリエ変換を実行する
// ただしNで割って規格化を行うことはしない
void fft_rec_ref(std::vector<std::complex<double>>& array, bool is_reverse) {
    auto n = array.size();
    if(n == 1) {
        return;
    }

    std::vector<std::complex<double>> even(n/2);
    std::vector<std::complex<double>> odd(n/2);
    for(auto i = 0; i < n/2; ++i) {
        even[i] = array[i *2];
        odd[i] = array[i *2 + 1];
    }

    // 再帰でeven,oddを別々に計算
    fft_rec_ref(even, is_reverse);
    fft_rec_ref(odd, is_reverse);

    // merge
    for(auto i = 0; i < n / 2; ++i) {
        auto rot_factor = std::exp(std::complex(0.0, -2.0 * M_PI * i /n));
        if(is_reverse) {
            rot_factor = std::conj(rot_factor);
        }

        array[i] = even[i] + rot_factor * odd[i];
        array[i + n /2] = even[i] - rot_factor * odd[i];
    }
}

void normailize(std::vector<std::complex<double>>& array) {
    auto n = array.size();
    for(auto i = 0; i < n; ++i) {
        array[i] /= std::complex((double)n, 0.0);
    }
}

void fft_rec_ref_2(std::vector<std::complex<double>>& array, bool is_reverse, std::size_t len_original) {
    auto n = array.size();
    if(n == 1) {
        if(is_reverse) {
            array[0] /= std::complex((double) len_original, 0.0);
        }
        return;
    }

    std::vector<std::complex<double>> even(n/2);
    std::vector<std::complex<double>> odd(n/2);
    for(auto i = 0; i < n/2; ++i) {
        even[i] = array[i *2];
        odd[i] = array[i *2 + 1];
    }

    // 再帰でeven,oddを別々に計算
    fft_rec_ref_2(even, is_reverse, len_original);
    fft_rec_ref_2(odd, is_reverse, len_original);

    // merge
    for(auto i = 0; i < n / 2; ++i) {
        auto rot_factor = std::exp(std::complex(0.0, -2.0 * M_PI * i /n));
        if(is_reverse) {
            rot_factor = std::conj(rot_factor);
        }

        array[i] = even[i] + rot_factor * odd[i];
        array[i + n /2] = even[i] - rot_factor * odd[i];
    }
}

void show_result(std::vector<std::complex<double>>& result) {
    for(auto& v: result) {
        std::cout << v.real() << " + " << v.imag() << "i" << ",";
    }
    std::cout << std::endl;
}

int main(void) {
    std::vector<std::complex<double>> array_1 = {1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9};
    show_result(array_1);

    // フーリエ変換
    fft_rec_ref(array_1, false);
    show_result(array_1);

    // 逆フーリエ変換
    fft_rec_ref(array_1, true);
    normailize(array_1);
    show_result(array_1);

    std::vector<std::complex<double>> array_2 = {1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9};
    show_result(array_2);

    // フーリエ変換
    fft_rec_ref_2(array_2, false, array_2.size());
    show_result(array_2);

    // 逆フーリエ変換
    fft_rec_ref_2(array_2, true, array_2.size());
    show_result(array_2);
}
