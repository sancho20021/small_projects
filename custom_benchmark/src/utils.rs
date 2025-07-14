use std::collections::HashMap;

use rand::Rng;

use crate::sorts::Element;

pub fn get_input_array(size: usize) -> Vec<Element> {
    let mut rng = rand::rng();
    (0..size).map(|_| rng.random()).collect()
}

pub fn map_values<K, V1, V2>(map: &HashMap<K, V1>, f: impl Fn(&V1) -> V2) -> HashMap<K, V2>
where
    K: Eq + std::hash::Hash + Clone,
    V2: Clone,
{
    map.iter().map(|(k, v)| (k.clone(), f(v))).collect()
}
