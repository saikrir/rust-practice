fn sort_usernames<'a>(users: &'a Vec<&'a str>) -> Vec<&'a str> {
    let mut local_cp = users.to_vec();

    //local_cp.sort_by(|a, b| a.to_ascii_lowercase().cmp(&b.to_ascii_lowercase()));
    local_cp.sort_by_cached_key(|a: &&str| a.to_ascii_lowercase());

    local_cp
}

#[test]
fn test_sort() {
    let names = vec!["Todd", "amy"];

    println!("sorted {:?}", sort_usernames(&names));
}
