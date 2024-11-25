Rust learning...

Creates a data-set of words from the book https://www.gutenberg.org/files/98/98-0.txt.
Implements a fixed sized open addressing hash table by using linear probing to resolve collisions.
The keys are the words from the given data-set and the hash table’s values are integers.
The following methodes are implemented with O(1)-complexity:

◦ insert(key, value)
inserts a new key-value pair or replaces a key’s existing value,

◦ remove(key)
removes the corresponding key-value pair,

◦ get(key)
returns the value of the corresponding key,

◦ get_last()
returns the most recent key-value pair that was either inserted or
updated and is still present,

◦ get_first()
returns the least recent key-value pair that was either inserted or
updated and is still present
