use num_complex::Complex64;

pub fn fft_rec(array: Vec<Complex64>) -> Vec<Complex64> {
    let n = array.len();
    let mut result = vec![Complex64::new(0.0, 0.0); n];
    if n == 1 {
        result[0] = array[0];
        return result;
    }

    let mut even = vec![Complex64::new(0.0, 0.0); n / 2];
    let mut odd = vec![Complex64::new(0.0, 0.0); n / 2];
    for i in 0..n / 2 {
        even[i] = array[i * 2];
        odd[i] = array[i * 2 + 1];
    }

    // 再帰でeven,oddを別々に計算
    let even = fft_rec(even);
    let odd = fft_rec(odd);

    // マージ
    for i in 0..n / 2 {
        let rot_factor = Complex64::new(0.0, 2.0 * std::f64::consts::PI * i as f64 / n as f64);
        result[i] = even[i] + rot_factor * odd[i];
        result[i + n / 2] = even[i] - rot_factor * odd[i];
    }

    result
}

pub fn fft_rec_ref(array: &mut Vec<Complex64>) {
    let n = array.len();
    if n == 1 {
        return;
    }

    let mut even = vec![Complex64::new(0.0, 0.0); n / 2];
    let mut odd = vec![Complex64::new(0.0, 0.0); n / 2];
    for i in 0..n / 2 {
        even[i] = array[i * 2];
        odd[i] = array[i * 2 + 1];
    }

    // 再帰でeven,oddを別々に計算
    fft_rec_ref(&mut even);
    fft_rec_ref(&mut odd);

    // マージ
    for i in 0..n / 2 {
        let rot_factor = Complex64::new(0.0, 2.0 * std::f64::consts::PI * i as f64 / n as f64);
        array[i] = even[i] + rot_factor * odd[i];
        array[i + n / 2] = even[i] - rot_factor * odd[i];
    }
}

fn main() {
    let case_1: Vec<Complex64> = vec![1.2, 1.3, 2.4, 3.1]
        .iter()
        .map(|x| Complex64::new(0.0, *x))
        .collect();
    let case_2: Vec<Complex64> = vec![1.2, 2.3, 1.3, 2.5, 2.4, 3.1, 3.4, 4.1]
        .iter()
        .map(|x| Complex64::new(0.0, *x))
        .collect();

    let res_1 = fft_rec(case_1.clone());
    let res_2 = fft_rec(case_2.clone());

    println!("res_1: {:?}", res_1);
    println!("res_2: {:?}", res_2);

    let mut res_1_2 = case_1.clone();
    let mut res_2_2 = case_2.clone();
    fft_rec_ref(&mut res_1_2);
    fft_rec_ref(&mut res_2_2);

    println!("res_1_2: {:?}", res_1_2);
    println!("res_2_2: {:?}", res_2_2);
}
