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

fn main() {
    let array = vec![1.2, 1.3, 1.4, 1.5, 1.6, 1.7];
    println!("array_ft: {:?}", dft(&array));
}
