mod approx_fraction;

use approx_fraction::*;

fn main() {
    let num = 23;

    for i in 2..62 {
        let before_approx = approximate_fraction(1, num, i-1);
        let approx = approximate_fraction(1, num, i);
        println!("{:?} ::: {:?} ::: {:?}", (approx.0) as i64 - 2*(before_approx.0) as i64, approx.0 as f64 / approx.1 as f64, approx);
    }
}