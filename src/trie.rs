use std::collections::HashMap;

#[derive(Debug)]
struct TrieNode {
    is_leaf: bool,
    children: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_leaf: false,
        }
    }

    fn insert(&mut self, s: &String) {
        let mut curr = self;
        for c in s.chars() {
            // curr.is_leaf = false;
            if !curr.children.contains_key(&c) {
                curr.children.insert(c, TrieNode::new());
            }
            let next: &mut TrieNode = curr.children.get_mut(&c).unwrap();
            curr = next;
        }
        curr.is_leaf = true;
    }

    fn delete(&mut self, s: &String) {
        fn _delete(curr: &mut TrieNode, word: &String, index: usize) -> bool {
            if index == word.chars().count() {
                if !curr.is_leaf {
                    return false;
                }
                curr.is_leaf = false;
                return curr.children.len() == 0;
            }
            let ch = word.chars().nth(index).unwrap();
            if let None = curr.children.get(&ch) {
                return false;
            }
            let char_node = curr.children.get_mut(&ch).unwrap();
            let delete_curr = _delete(char_node, word, index + 1);
            if delete_curr {
                curr.children.remove(&ch);
                return curr.children.len() == 0;
            }
            return false;
        }
        _delete(self, s, 0);
    }

    // TODO remove

    // TODO iterators
}

fn main() {
    // let mut contacts: HashMap<char, &str> = HashMap::new();
    // let mut contacts = Some(Box::new(contacts));

    // match contacts {
    //     Some(mut m) => {
    //         println!("some");
    //         m.insert('d', "798-1364");
    //         m.insert('a', "645-7689");
    //         m.insert('k', "435-8291");
    //         m.insert('r', "956-1745");
    //         for (k, v) in m.iter() {
    //             println!("k = {}, v = {}", k, v);
    //         }
    //         println!("contains key x = {}", m.contains_key(&'x'));
    //         println!("contains key r = {}", m.contains_key(&'r'));
    //         if let Some(x) = m.get(&'r') {
    //             println!("Value of r = `{}", x);
    //         }
    //         println!("{:?}", &m);
    //     }
    //     None => println!("none"),
    // }

    // let s = String::from("ciao zio");
    // for (i, c) in s.chars().enumerate() {
    //     println!("Char [{}] = {}", i, c);
    // }

    let mut my_trie = TrieNode::new();
    let s = String::from("ab");
    my_trie.insert(&s);
    let s = String::from("abc");
    my_trie.insert(&s);
    println!("{:?}", my_trie);
    
    println!("**********");
    
    let s = String::from("ab");
    my_trie.delete(&s);
    println!("{:?}", my_trie);
}
