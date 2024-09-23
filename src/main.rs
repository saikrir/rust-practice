mod median;

use median::median;

fn main() {
    let mut my_vec: Vec<f64> = vec![17f64, 4f64, 83f64, 3f64, 2f64, 56f64, 11f64];
    let median_val = median(&mut my_vec);

    println!("median value {}", median_val.unwrap_or_default())
}
