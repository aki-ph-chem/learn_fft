# FFT

learn fft

## 配列の長さが2のべき乗でない場合

多くの場合は配列の長さ $n$ に対して次に大きい2のべき乗の数との差だけ末尾に0を追加する。

C++の実装
```C++
// ビット長を求める
std::size_t len_bit(std::size_t n) {
    std::size_t length_bit = 0;
    while(n > 0) {
        n >>= 1;
        ++length_bit;
    }

    return length_bit;
}

// arrayの長さが2のべき乗でない場合は末尾に0を追加
void zero_padding(std::vector<std::complex<double>>& array) {
    auto n = array.size();

    if(!(n&(n-1))){
        return;
    }

    auto diff = ( 1 << len_bit(n)) - n;
    for(auto i = 0; i < diff; ++i) {
        array.push_back(0);
    }
}
```

Rustでの実装

```Rust
// ビット長を求める
fn len_bit(mut n: usize) -> usize {
    let mut length_bit = 0;
    while n > 0 {
        n >>= 1;
        length_bit += 1;
    }

    length_bit
}

// zero padding
pub fn zero_padding(array: &mut Vec<Complex64>) {
    let n = array.len();
    if n & (n - 1) == 0 {
        return;
    }

    let diff = (1 << len_bit(n)) - n;
    for _i in 0..diff {
        array.push(Complex64::new(0.0, 0.0));
    }
}
```
