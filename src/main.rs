mod approx_fraction;

use approx_fraction::*;

fn main() {
    let num = 3;
    let len = 63;

    let vec_approx: Vec<_> = (1..len+1).map(|i| approximate_fraction(1, num, i)).collect();

    let vec_iter = vec_approx.iter()
        .enumerate()
        .filter(|x| x.0 < (len-1) as usize);

    for (i, val) in vec_iter {
        let before_approx = val;
        let approx = vec_approx[i+1];
        println!("{:?} ::: {:?} ::: {:?}", approx.0 as i64 - 2*before_approx.0 as i64, approx.0 as f64 / approx.1 as f64, approx);
    }
}