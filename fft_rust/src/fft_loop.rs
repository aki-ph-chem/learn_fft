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

fn main() {
    let mut array: Vec<Complex64> = vec![1.0, 2.0, 3.0, 4.0]
        .iter()
        .map(|x| Complex64::new(*x, 0.0))
        .collect();

    // フーリエ変換
    fft_loop_ref(&mut array, false);
    println!("array: {:?}", array);

    // 逆フーリエ変換
    fft_loop_ref(&mut array, true);
    println!("array: {:?}", array);

    let mut array: Vec<Complex64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]
        .iter()
        .map(|x| Complex64::new(*x, 0.0))
        .collect();

    // フーリエ変換
    fft_loop_ref(&mut array, false);
    println!("array:\n {:?}", array);

    // 逆フーリエ変換
    fft_loop_ref(&mut array, true);
    println!("array:\n {:?}", array);
}
