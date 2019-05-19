// Return the number in the middle of left and right
// Used to handle overflow
fn middle(left: u64, right: u64) -> u64 {
    // last part is one iff left andr right are both odd
    (left>>1) + (right>>1) + (left&right&1)
}

fn recursive_approximate_fraction(p: u64, q: u64, exponent: u64, lbound: u64, ubound: u64) -> u64 {
    if ubound == lbound + 1 {
        if 1<<exponent * p.pow(2) > q*lbound + q>>1 {
            return ubound
        } else {
            return lbound
        }
    }

    let middle_bound = middle(lbound, ubound);

    if 1<<exponent * p > middle_bound * q {
        recursive_approximate_fraction(p, q, exponent, middle_bound, ubound)
    } else {
        recursive_approximate_fraction(p, q, exponent, lbound, middle_bound)
    }
}

// Return k in [0, 2^exponent] such that k/2^exponent is as close
// as possible to the fraction p/q
pub fn approximate_fraction(p: u64, q: u64, exponent: u64) -> (u64, u64) {  
    return (
        recursive_approximate_fraction(p, q, exponent, 0, 1<<exponent),
        1<<exponent
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn middle_test() {
        assert_eq!(middle(2, 4), 3);
        assert_eq!(middle(2, 5), 3);
        assert_eq!(middle(2, 6), 4);
        assert_eq!(middle(7, 1), 4);
        assert_eq!(middle(u64::max_value(), u64::max_value()-2), u64::max_value()-1);
    }
 

    #[test]
    fn approximate() {
        let (test, _) = approximate_fraction(1, 4, 4);
        assert_eq!(test, 4);
        
        let (test, _) = approximate_fraction(1, 3, 4);
        assert_eq!(test, 6);
    }
}