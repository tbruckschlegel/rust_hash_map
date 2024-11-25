pub mod tools {

    // Our own HashNode struct
    #[derive(Debug)]
    pub struct HashNode<K, V>
    where
        K: Clone,
        V: Clone,
    {
        pub key: Option<K>,
        pub value: Option<V>,

        // P.S. I initially wanted to use a reference / pointer to a node, but that became horrible
        // hash_index used for next and prev nodes
        prev_node_hash_index: Option<usize>,
        next_node_hash_index: Option<usize>,
    }

    impl<K, V> HashNode<K, V>
    where
        K: Clone,
        V: Clone,
    {
        pub fn new(
            key: Option<K>,
            value: Option<V>,
            prev_node_hash_index: Option<usize>,
            next_node_hash_index: Option<usize>,
        ) -> Self {
            HashNode {
                key,
                value,
                prev_node_hash_index,
                next_node_hash_index,
            }
        }

        pub fn new_empty() -> Self {
            HashNode {
                key: None,
                value: None,
                prev_node_hash_index: None,
                next_node_hash_index: None,
            }
        }
    }

    impl<K: Clone, V: Clone> Clone for HashNode<K, V>
    where
        K: Clone,
        V: Clone,
    {
        fn clone(&self) -> Self {
            HashNode {
                key: self.key.clone(),
                value: self.value.clone(),
                prev_node_hash_index: self.prev_node_hash_index.clone(),
                next_node_hash_index: self.next_node_hash_index.clone(),
            }
        }
    }

    // Our own HashMap struct
    pub struct HashMap<K, V>
    where
        K: Clone,
        V: Clone,
    {
        capacity: usize,
        arr: Vec<HashNode<K, V>>,
        // we use hash_index to remember the insert/change history
        latest_node: Option<usize>,
        oldest_node: Option<usize>,
    }

    impl<
            K: std::fmt::Debug + std::cmp::PartialEq + std::hash::Hash,
            V: std::fmt::Debug + std::cmp::PartialEq,
        > HashMap<K, V>
    where
        K: Clone,
        V: Clone,
    {
        pub fn new(capacity: usize) -> Self {
            let empty_node = HashNode::<K, V>::new_empty(); // Empty node to fill the array
            HashMap {
                capacity,
                arr: vec![empty_node.clone(); capacity], // Initialize the array with empty nodes
                latest_node: None,
                oldest_node: None,
            }
        }

        // Hash function to find index for a key
        fn hash_code(&self, key: &K) -> usize
        where
            K: std::hash::Hash,
        {
            // TODO: There should be a way to use a static hasher that can be "reset" instead of re-instantiated every time?

            use std::hash::Hasher;
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            key.hash(&mut hasher);
            let finished_hash = hasher.finish();
            #[cfg(feature = "debug")]
            println!(
                "key: {:?} hash {} hash % capacity {}",
                key,
                finished_hash,
                finished_hash as usize % self.capacity
            );

            finished_hash as usize % self.capacity
        }

        // Function to add key-value pair
        pub fn insert(&mut self, key: K, value: V) {
            let mut hash_index = self.hash_code(&key);

            let mut temp = HashNode::<K, V>::new(
                Some(key.clone()),
                Some(value.clone()),
                self.latest_node.clone(),
                None,
            );

            // Linear probing
            if self.arr[hash_index].key.is_some()
                && self.arr[hash_index].key.as_ref().unwrap() != &key
            // no duplicated keys allowed, so overwrite them
            {
                while hash_index < self.capacity
                    && self.arr[hash_index].key.is_some()
                    && self.arr[hash_index].key.as_ref().unwrap() != &key
                {
                    hash_index += 1;
                }
                debug_assert!(hash_index < self.capacity);
            }

            // Not the first node, update prev node
            if self.latest_node != None {
                if self.arr[self.latest_node.unwrap()].key.as_ref().unwrap() == &key {
                    temp.prev_node_hash_index =
                        self.arr[self.latest_node.unwrap()].prev_node_hash_index;
                    // use old prev
                }

                self.arr[self.latest_node.unwrap()].next_node_hash_index = Some(hash_index);

                if self.arr[self.oldest_node.unwrap()].key.as_ref().unwrap() == &key {
                    // Oldest node overwritten
                    self.oldest_node = self.arr[self.oldest_node.unwrap()].next_node_hash_index;
                }
            } else {
                if self.oldest_node == None {
                    // Init oldest node, since it's empty
                    self.oldest_node = Some(hash_index);
                }
            }

            // Store new last node
            self.latest_node = Some(hash_index);
            #[cfg(feature = "debug")]
            println!(
                "insert:  index = {} key = {:?} value = {:?}",
                hash_index.clone(),
                key.clone(),
                value.clone()
            );

            self.arr[hash_index] = temp;

            #[cfg(feature = "debug")]
            println!("insert op oldest {:?}", &self.oldest_node.as_ref().unwrap());
            #[cfg(feature = "debug")]
            println!("insert op newest {:?}", &self.latest_node.as_ref().unwrap());
        }

        // Function to delete a key-value pair
        pub fn remove(&mut self, key: K) {
            let mut hash_index = self.hash_code(&key);
            let temp = HashNode::<K, V>::new_empty(); // empty node to fill the array

            // Linear probing
            if self.arr[hash_index].key.is_some()
                && self.arr[hash_index].key.as_ref().unwrap() != &key
            {
                while hash_index < self.capacity
                    && self.arr[hash_index].key.is_some()
                    && self.arr[hash_index].key.as_ref().unwrap() != &key
                {
                    hash_index += 1;
                }
                if hash_index >= self.capacity {
                    debug_assert!(hash_index < self.capacity);
                }
            }

            // Remove node and re-link prev and next nodes
            let node_prev = self.arr[hash_index].prev_node_hash_index.clone();
            let node_next = self.arr[hash_index].next_node_hash_index.clone();
            if node_prev != None {
                self.arr[node_prev.unwrap()].next_node_hash_index = node_next;
            }

            if self.oldest_node.is_some()
                && self.arr[self.oldest_node.unwrap()].key.as_ref().unwrap() == &key
            {
                // Oldest node will be removed
                self.oldest_node = self.arr[hash_index].next_node_hash_index;
            }

            if node_next != None {
                self.arr[node_next.unwrap()].prev_node_hash_index = node_prev;
            }

            if self.latest_node.is_some()
                && &key == self.arr[self.latest_node.unwrap()].key.as_ref().unwrap()
            {
                // If we are the latest node, there must be no next node
                debug_assert!(node_next == None);
                // Store new last node
                self.latest_node = node_prev;
            }

            self.arr[hash_index] = temp;
            #[cfg(feature = "debug")]
            println!("remove op oldest {:?}", &(self.oldest_node.as_ref()));
            #[cfg(feature = "debug")]
            println!("remove op newest {:?}", &(self.latest_node.as_ref()));
        }

        // Function to search the value for a given key
        pub fn get(&self, key: &K) -> Option<V> {
            let mut hash_index = self.hash_code(&key);

            // Linear probing
            if self.arr[hash_index].key.is_some()
                && self.arr[hash_index].key.as_ref().unwrap() != key
            {
                while hash_index < self.capacity
                    && self.arr[hash_index].key.is_some()
                    && self.arr[hash_index].key.as_ref().unwrap() != key
                {
                    hash_index += 1;
                }
                if hash_index >= self.capacity {
                    debug_assert!(hash_index < self.capacity);
                }
            }

            return self.arr[hash_index].clone().value;
        }

        // Get the most recent key-value pair that was either inserted or updated and is still present
        pub fn get_last(&self) -> HashNode<K, V> {
            #[cfg(feature = "debug")]
            println!("get_last op oldest {:?}", &(self.oldest_node.as_ref()));
            #[cfg(feature = "debug")]
            println!("get_last op newest {:?}", &(self.latest_node.as_ref()));

            if self.latest_node == None {
                println!("hash map is empty");
                return HashNode::<K, V>::new_empty();
            } else {
                return self.arr[self.latest_node.unwrap()].clone();
            }
        }

        // Get returns the least recent key-value pair that was either inserted or updated and is still present
        pub fn get_first(&self) -> HashNode<K, V> {
            #[cfg(feature = "debug")]
            println!("get_first op oldest {:?}", &(self.oldest_node.as_ref()));
            #[cfg(feature = "debug")]
            println!("get_first op newest {:?}", &(self.latest_node.as_ref()));

            if self.oldest_node == None {
                if self.latest_node == None {
                    println!("hash map is empty");
                    return HashNode::<K, V>::new_empty();
                } else {
                    return self.arr[self.latest_node.unwrap()].clone();
                }
            } else {
                return self.arr[self.oldest_node.unwrap()].clone();
            }
        }

        // Function to display the stored key-value pairs
        pub fn display(&self) {
            for i in 0..self.arr.len() {
                let n = &self.arr[i];
                if n.value != None {
                    println!(
                        "index = {:?} key = {:?} value = {:?} prev = {:?} next = {:?}",
                        i, n.key, n.value, n.prev_node_hash_index, n.next_node_hash_index
                    );
                }
            }
        }
    }
}
