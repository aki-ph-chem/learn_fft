use num_complex::Complex64;

// is_reverseがtrueのときは逆フーリエ変換を実行する
// ただし最後のNでの規格化はしない
pub fn fft_rec_ref(array: &mut Vec<Complex64>, is_reverse: bool) {
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
    fft_rec_ref(&mut even, is_reverse);
    fft_rec_ref(&mut odd, is_reverse);

    // マージ
    for i in 0..n / 2 {
        let mut rot_factor =
            Complex64::new(0.0, -2.0 * std::f64::consts::PI * i as f64 / n as f64).exp();
        if is_reverse {
            rot_factor = rot_factor.conj();
        }

        array[i] = even[i] + rot_factor * odd[i];
        array[i + n / 2] = even[i] - rot_factor * odd[i];
    }
}

// is_reverseがtrueのときは逆フーリエ変換を実行する
// 最後のNでの規格化も行う
pub fn fft_rec_ref_2(array: &mut Vec<Complex64>, is_reverse: bool, len_original: usize) {
    let n = array.len();
    if n == 1 {
        if is_reverse {
            array[0] /= Complex64::new(len_original as f64, 0.0);
        }
        return;
    }

    let mut even = vec![Complex64::new(0.0, 0.0); n / 2];
    let mut odd = vec![Complex64::new(0.0, 0.0); n / 2];
    for i in 0..n / 2 {
        even[i] = array[i * 2];
        odd[i] = array[i * 2 + 1];
    }

    // 再帰でeven,oddを別々に計算
    fft_rec_ref_2(&mut even, is_reverse, len_original);
    fft_rec_ref_2(&mut odd, is_reverse, len_original);

    // マージ
    for i in 0..n / 2 {
        let mut rot_factor =
            Complex64::new(0.0, -2.0 * std::f64::consts::PI * i as f64 / n as f64).exp();
        if is_reverse {
            rot_factor = rot_factor.conj();
        }

        array[i] = even[i] + rot_factor * odd[i];
        array[i + n / 2] = even[i] - rot_factor * odd[i];
    }
}

fn main() {
    let mut case_3: Vec<Complex64> = vec![1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9]
        .iter()
        .map(|x| Complex64::new(*x, 0.0))
        .collect();
    println!("case_3: {:?}", case_3);

    fft_rec_ref(&mut case_3, false);
    println!("case_3: {:?}", case_3);

    fft_rec_ref(&mut case_3, true);
    case_3 = case_3
        .iter()
        .map(|x| *x / Complex64::new(case_3.len() as f64, 0.0))
        .collect();
    println!("case_3: {:?}", case_3);

    let mut case_4: Vec<Complex64> = vec![1.2, 1.3]
        .iter()
        .map(|x| Complex64::new(*x, 0.0))
        .collect();
    println!("case_4: {:?}", case_4);

    fft_rec_ref(&mut case_4, false);
    println!("case_4: {:?}", case_4);

    fft_rec_ref(&mut case_4, true);
    case_4 = case_4
        .iter()
        .map(|x| *x / Complex64::new(case_4.len() as f64, 0.0))
        .collect();
    println!("case_4: {:?}", case_4);

    let mut case_3: Vec<Complex64> = vec![1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9]
        .iter()
        .map(|x| Complex64::new(*x, 0.0))
        .collect();
    println!("case_3: {:?}", case_3);
    let len_case_3 = case_3.len();

    fft_rec_ref_2(&mut case_3, false, len_case_3);
    println!("case_3: {:?}", case_3);

    fft_rec_ref_2(&mut case_3, true, len_case_3);
    println!("case_3: {:?}", case_3);
}
