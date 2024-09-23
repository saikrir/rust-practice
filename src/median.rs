pub fn median(list: &mut Vec<f64>) -> Option<f64> {
    if list.is_empty() {
        return None;
    }

    list.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("Sorted {:#?} ", list);
    let mid_point = list.len() / 2;
    if list.len() % 2 == 0 {
        Some(list[mid_point] + list[mid_point - 1] / 2f64)
    } else {
        Some(list[mid_point])
    }
}
