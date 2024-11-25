#[cfg(test)]
#[path = "../src/hash_map.rs"]
mod hash_map;
use hash_map::tools::HashMap;

mod tests {
   use super::*;
   
    #[test]
    fn test_insert_item() {
        let mut map = HashMap::new(10);

        map.insert(1, String::from("one"));

        assert_eq!(map.get_first().key.unwrap(), 1); // The oldest should be the last inserted
        assert_eq!(map.get_last().key.unwrap(), 1); // The newest should be the first inserted

        map.insert(2, String::from("two"));

        assert_eq!(map.get_first().key.unwrap(), 1); // The oldest should be the last inserted
        assert_eq!(map.get_last().key.unwrap(), 2); // The newest should be the first inserted

        assert_eq!(map.get(&1).unwrap(), String::from("one")); // The newest should be the first inserted
        assert_eq!(map.get(&2).unwrap(), String::from("two")); // The newest should be the first inserted

        map.insert(3, String::from("three"));

        assert_eq!(map.get_first().key.unwrap(), 1); // The oldest should be the last inserted
        assert_eq!(map.get_last().key.unwrap(), 3); // The newest should be the first inserted

        map.insert(4, String::from("four"));

        assert_eq!(map.get_first().key.unwrap(), 1); // The oldest should be the last inserted
        assert_eq!(map.get_last().key.unwrap(), 4); // The newest should be the first inserted
    }

    #[test]
    fn test_remove_item() {
        let mut map = HashMap::new(10);

        map.remove(1); // should not crash

        map.insert(1, String::from("one"));

        map.remove(1); // should not crash or assert

        assert_eq!(map.get_first().key, None); // The oldest should be empty
        assert_eq!(map.get_last().key, None); // The newest should be empty

        map.insert(2, String::from("two"));

        assert_eq!(map.get_first().key.unwrap(), 2); // The oldest should be the same as the newest
        assert_eq!(map.get_last().key.unwrap(), 2); // The newest should be the same as the oldest

        map.insert(3, String::from("three"));

        assert_eq!(map.get_first().key.unwrap(), 2); // The oldest should be the last inserted
        assert_eq!(map.get_last().key.unwrap(), 3); // The newest should be the first inserted

        map.insert(4, String::from("four"));

        assert_eq!(map.get_first().key.unwrap(), 2); // The oldest should be the last inserted
        assert_eq!(map.get_last().key.unwrap(), 4); // The newest should be the first inserted
        map.display();
    }

}
