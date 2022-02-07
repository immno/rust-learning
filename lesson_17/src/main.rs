use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    explain("empty", &map);

    map.insert("a", 1);
    explain("added 1", &map);

    map.insert("b", 2);
    map.insert("c", 3);
    explain("added 3", &map);

    map.insert("d", 4);
    explain("added 4", &map);

    assert_eq!(map.get(&"a"), Some(&1));
    assert_eq!(map.get_key_value(&"b"), Some((&"b", &2)));

    map.remove(&"a");
    assert_eq!(map.contains_key(&"a"), false);
    assert_eq!(map.get(&"a"), None);
    explain("remove", &map);

    map.shrink_to_fit();
    explain("shrinked", &map);
}


fn explain<K, V>(name: &str, map: &HashMap<K, V>) {
    println!("{}: len: {}, capacity: {}", name, map.len(), map.capacity());
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::mem;

    fn explain<K, V>(name: &str, map: HashMap<K, V>) -> HashMap<K, V> {
        let arr: [usize; 6] = unsafe { mem::transmute(map) };
        println!("{}:bucket_mask 0x{:x}, ctrl 0x{:x}, growth_left: {}, items: {}",
                 name, arr[2], arr[3], arr[4], arr[5]
        );
        unsafe { std::mem::transmute(arr) }
    }

    #[test]
    fn test() {
        let map = HashMap::new();
        let mut map = explain("empty", map);
        map.insert('a', 1);
        let mut map = explain("added 1", map);
        map.insert('b', 2);
        map.insert('c', 3);
        let mut map = explain("added 3", map);
        map.insert('d', 4);
        let mut map = explain("added 4", map);
        map.remove(&'a');
        explain("final", map);
    }

    #[derive(Debug, Hash, PartialEq, Eq)]
    struct Student<'a> {
        name: &'a str,
        age: u8,
    }

    impl<'a> Student<'a> {
        pub fn new(name: &'a str, age: u8) -> Self {
            Self { name, age }
        }
    }

    #[test]
    fn example1() {
        let mut hasher = DefaultHasher::new();
        let student = Student::new("Mno", 18);

        student.hash(&mut hasher);
        let mut map = HashMap::new();
        map.insert(student, vec!["Math", "Writing"]);
        println!("hash: 0x{:x}, map: {:?}", hasher.finish(), map);
    }

    fn explain2<K, V>(name: &str, map: BTreeMap<K, V>) -> BTreeMap<K, V> {
        let arr: [usize; 3] = unsafe { mem::transmute(map) };
        println!("{}: height: {}, root node: 0x{:x}, len: 0x{:x}",
                 name, arr[0], arr[1], arr[2]
        );
        unsafe { std::mem::transmute(arr) }
    }

    #[test]
    fn example2() {
        let map = BTreeMap::new();
        let mut map = explain2("empty", map);

        for i in 0..16usize {
            map.insert(format!("Mno {}", i), i);
        }

        let mut map = explain2("added", map);
        map.remove("Mno 1");
        let map = explain2("remove 1", map);

        for item in map.iter() {
            println!("{:?}", item);
        }
    }

}