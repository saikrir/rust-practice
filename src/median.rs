pub fn median(list: &mut Vec<f64>) -> Option<f64> {
    //3 cases
    // 0. sort the list
    // 1. when empty list, return None
    // 2. when odd number of elements, pick the middle element and return it
    // 3. when even number of elements, pick the avg of n-1, n and return it
    let list_len = list.len();
    if list.is_empty() {
        return None;
    }

    let mut local_cp = list.clone();
    local_cp.sort_by(|a,b| a.partial_cmp(b).unwrap());

    let mid_index = list_len/2;
    let med = if list_len % 2 == 1 {
        local_cp[mid_index]
    } else {
        (local_cp[mid_index-1] + local_cp[mid_index])/2.0
    };

    Some(med) 
    
}

#[test]
fn assert_no_elements() {
    let mut input_vec: Vec<f64> = vec![];
    assert_eq!(median(&mut input_vec), None);
}

#[test]
fn assert_odd_elements() {
    let mut input_vec: Vec<f64> = vec![5.0,1.5,2.0,];
    assert_eq!(median(&mut input_vec), Some(2.0));
}

#[test]
fn assert_even_elements() {
    let mut input_vec: Vec<f64> = vec![5.0,1.5,2.0,11.0];
    assert_eq!(median(&mut input_vec), Some(3.5));
}