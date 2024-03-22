use num_complex::Complex64;

pub fn dft(array: &Vec<f64>) -> Vec<Complex64> {
    let n = array.len();
    let mut result = vec![Complex64::new(0.0, 0.0); n];

    for l in 0..n {
        for (m, v) in array.iter().enumerate() {
            let exp_x = Complex64::new(
                0.0,
                -2.0 * std::f64::consts::PI * (l as f64 * m as f64) / n as f64,
            )
            .exp();

            result[l] += exp_x * v;
        }
    }

    result
}

// is_reverseがtrueのときは逆フーリエ変換を実行する
pub fn dft_2(array: &Vec<Complex64>, is_reverse: bool) -> Vec<Complex64> {
    let n = array.len();
    let mut result = vec![Complex64::new(0.0, 0.0); n];

    for l in 0..n {
        for (m, v) in array.iter().enumerate() {
            let mut rot_factor = Complex64::new(
                0.0,
                -2.0 * std::f64::consts::PI * (l as f64 * m as f64) / n as f64,
            )
            .exp();

            if is_reverse {
                rot_factor = rot_factor.conj() / Complex64::new(n as f64, 0.0);
            }

            result[l] += rot_factor * v;
        }
    }

    result
}

fn main() {
    let array = vec![1.2, 1.3, 1.4, 1.5, 1.6, 1.7];
    println!("array_ft: {:?}", dft(&array));

    // フーリエ変換したものを逆フーリエ変換して確かめる
    let array = vec![1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9]
        .iter()
        .map(|x| Complex64::new(*x, 0.0))
        .collect();
    let res_dft = dft_2(&array, false);
    let res_dft_rev: Vec<f64> = dft_2(&res_dft, true).iter().map(|x| x.re).collect();
    println!("res_dft_rev: {:?}", res_dft_rev);
}
