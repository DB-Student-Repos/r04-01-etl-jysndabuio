use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut new = BTreeMap::new();
    for (key, list) in h.iter(){
        for x in list.iter().map(|x|x.to_ascii_lowercase()) {
            new.insert(x, *key);
        }
    }
    new
}
