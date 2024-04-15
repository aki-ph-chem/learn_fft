// 参考: https://www.geeksforgeeks.org/program-to-generate-an-array-having-convolution-of-two-given-arrays/
use std::default::Default;
use std::ops::{Add, AddAssign, Div, Mul};

// 1の実装の方がシンプル

// time: O(N^2)
fn convolution_1<T>(array_1: &[T], array_2: &[T]) -> Vec<T>
where
    T: Default + Copy + Add + Mul + Div + AddAssign<<T as Mul>::Output>,
{
    let (len_1, len_2) = (array_1.len(), array_2.len());
    let mut result = vec![Default::default(); len_1 + len_2 - 1];

    for i in 0..len_1 {
        for j in 0..len_2 {
            result[i + j] += array_1[i] * array_2[j];
        }
    }

    result
}

// time: O(N^2)
fn convolution_2<T>(array_1: &[T], array_2: &[T]) -> Vec<T>
where
    T: Default + Copy + Add + Mul + Div + AddAssign<<T as Mul>::Output>,
{
    let (len_1, len_2) = (array_1.len(), array_2.len());
    let mut result = vec![Default::default(); len_1 + len_2 - 1];

    for k in 0..len_1 + len_2 - 1 {
        for i in 0..len_1 {
            if i <= k && k - i < len_2 {
                result[k] += array_1[i] * array_2[k - i];
            }
        }
    }

    result
}

fn main() {
    let case_1 = ([1, 2, 3, 4], [5, 6, 7, 8, 9]);
    println!("res: conv_1: {:?}", convolution_1(&case_1.0, &case_1.1));
    println!("res: conv_2: {:?}", convolution_2(&case_1.0, &case_1.1));
}
