use arrow::{array::Int32Array, compute::sum};

fn main() {
    let data = Int32Array::from_iter(0..100);
    println!("{:?}", data)
}
