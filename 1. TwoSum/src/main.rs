mod foo;
use crate::foo::{my_struct, TwoSum};

fn main() {
    let a  = TwoSum::two_sum(vec![3,2,4], 6);
    println!("{:?}", a);
}
