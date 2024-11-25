mod hash_map;
use hash_map::tools::HashMap;

fn main() {
    
    {
        let mut map = HashMap::new(10);

        map.remove(1); // should not crash

        map.insert(1, String::from("1"));

        map.remove(1); // should not crash

        assert_eq!(map.get_first().key, None); // The oldest should be the last inserted
        assert_eq!(map.get_last().key, None); // The newest should be the first inserted

        let mut h = HashMap::new(10);

        println!("newest: {:?}", h.get_last().key);
        println!("oldest: {:?}", h.get_first().key);
        h.remove(2);
        h.insert(1, String::from("1"));
        h.insert(2, String::from("2"));
        h.insert(2, String::from("3"));
        h.insert(3, String::from("4"));
        h.insert(4, String::from("5"));

        h.display();
        println!("Get value for key 2: {:?}", h.get(&2));

        println!("newest: {:?}", h.get_last().key);
        println!("oldest: {:?}", h.get_first().key);
        h.insert(1, String::from("12"));
        h.remove(2);
        h.display();
        println!("oldest: {:?}", h.get_first().key);
        println!("newest: {:?}", h.get_last().key);
    }

    {
        let mut map = HashMap::new(10);

        map.remove(String::from("1")); // should not crash

        map.insert(String::from("1"), 1);

        map.remove(String::from("1")); // should not crash

        assert_eq!(map.get_first().key, None); // The oldest should be the last inserted
        assert_eq!(map.get_last().key, None); // The newest should be the first inserted

        let mut h = HashMap::new(10);

        println!("newest: {:?}", h.get_last().key);
        println!("oldest: {:?}", h.get_first().key);
        h.remove(String::from("2"));
        h.insert(String::from("1"), 1);
        h.insert(String::from("2"), 2);
        h.insert(String::from("2"), 3);
        h.insert(String::from("3"), 4);
        h.insert(String::from("4"), 5);

        h.display();
        println!("Get value for key 2: {:?}", h.get(&String::from("2")));

        println!("newest: {:?}", h.get_last().key);
        println!("oldest: {:?}", h.get_first().key);
        h.insert(String::from("1"), 12);
        h.remove(String::from("2"));
        h.display();
        println!("oldest: {:?}", h.get_first().key);
        println!("newest: {:?}", h.get_last().key);
    }

    {
        use std::env;
        use std::fs;
        use std::path::Path;

        let path = Path::new("data/data.txt");
        println!(
            "current path {:?} path {:?}",
            env::current_dir(),
            path.canonicalize()
        );
        // Read file into a string
        let content = fs::read_to_string("data/data.txt");

        if content.is_ok() {
            // Convert to ascii lower case and replace all non alphanumeric chars with a whitespace
            let remove_special_chars: String = content
                .unwrap()
                .to_ascii_lowercase()
                .chars()
                .map(|c| if c.is_alphanumeric() { c } else { ' ' })
                .collect();
            // Split into words
            let words: Vec<&str> = remove_special_chars.split_whitespace().collect();

            // Do word counting via the hash map
            let mut map: HashMap<String, i32> = HashMap::new(32000);
            for word in words {
                if let Some(value) = map.get(&word.to_string()) {
                    map.insert(word.to_string(), value + 1);
                } else {
                    map.insert(word.to_string(), 1);
                }
            }

            map.display();
        }
    }
}
