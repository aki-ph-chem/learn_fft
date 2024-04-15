// 参考: https://www.geeksforgeeks.org/program-to-generate-an-array-having-convolution-of-two-given-arrays/

use num_complex::Complex64;

fn bit_reverse(mut x: usize, log2_n: usize) -> usize {
    let mut n = 0;
    for _i in 0..log2_n {
        n <<= 1;
        n |= x & 1;
        x >>= 1;
    }

    n
}

// is_reverseがtrueなら逆フーリエ変換を実行(規格化も行う)
fn fft_loop_ref(array: &mut Vec<Complex64>, is_reverse: bool) {
    let n = array.len();
    let log2_n = (n as f64).log2().floor() as usize;
    let len_arry = Complex64::new(n as f64, 0.0);

    // bit reversal of the given array
    let buff = array.clone();
    for i in 0..n {
        let idx_rev = bit_reverse(i, log2_n);
        array[i] = buff[idx_rev];
        if is_reverse {
            array[i] /= len_arry;
        }
    }

    for s in 1..=log2_n {
        let m = 1 << s;
        let m_2 = m >> 1;
        let mut w = Complex64::new(1.0, 0.0);

        let mut w_m = Complex64::new(0.0, std::f64::consts::PI / m_2 as f64).exp();
        if is_reverse {
            w_m = w_m.conj();
        }

        for j in 0..m_2 {
            let mut k = j;
            while k < n {
                let t = w * array[k + m_2];
                let u = array[k];

                array[k] = u + t;
                array[k + m_2] = u - t;

                k += m;
            }
            w *= w_m;
        }
    }
}

// FFTを利用してtime: O(NlogN)で畳み込み計算を行う
fn conv_fft(mut array_1: Vec<Complex64>, mut array_2: Vec<Complex64>) -> Vec<Complex64> {
    let len_bit = |mut n| {
        let mut length_bit = 0;
        while n > 0 {
            n >>= 1;
            length_bit += 1;
        }

        length_bit
    };
    let (len_array_1, len_array_2) = (array_1.len(), array_2.len());
    let len_total = len_array_1 + len_array_2 - 1;

    let (diff_array_len_1, diff_array_len_2) = if len_total & (len_total - 1) != 0 {
        (
            1 << len_bit(len_total) - len_array_1,
            1 << len_bit(len_total) - len_array_2,
        )
    } else {
        (len_total - len_array_1, len_total - len_array_2)
    };

    array_1 = vec![array_1, vec![Complex64::new(0.0, 0.0); diff_array_len_1]].concat();
    array_2 = vec![array_2, vec![Complex64::new(0.0, 0.0); diff_array_len_2]].concat();

    fft_loop_ref(&mut array_1, false);
    fft_loop_ref(&mut array_2, false);

    let mut result = vec![Complex64::new(0.0, 0.0); len_total];
    for ((idx, v_1), v_2) in array_1.iter().enumerate().zip(array_2.iter()) {
        result[idx] = v_1 * v_2;
    }

    fft_loop_ref(&mut result, true);

    result
}

fn main() {
    let case_1 = (
        vec![1, 2, 3, 4]
            .iter()
            .map(|x| Complex64::new(*x as f64, 0.0))
            .collect::<Vec<Complex64>>(),
        vec![5, 6, 7, 8, 9]
            .iter()
            .map(|x| Complex64::new(*x as f64, 0.0))
            .collect::<Vec<Complex64>>(),
    );

    let res_1 = conv_fft(case_1.0, case_1.1);
    println!("res_1:\n {:?}", res_1);
}
