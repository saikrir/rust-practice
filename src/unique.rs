use std::{collections::HashSet, hash::Hash};

pub fn unique<T>(list: &Vec<T>)-> Vec<T> where T:Ord, T:Copy, T:Hash{
    
    let mut aSet: HashSet<T> = HashSet::new();

    for i in list {
        aSet.insert(*i);
    }

    aSet.into_iter().collect()
}

#[test]
fn test_uniqe() {
    let input = vec![1,2,1,3,1,3];
    let op = unique(&input);
    assert_eq!(3, op.len())
}