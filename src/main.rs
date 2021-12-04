use std::collections::HashMap;
const UPTO: u64 = 77;

fn fibonacci_dynamic(n: u64, map: &mut HashMap<u64, u64>) -> u64 {
    match n {
        0 | 1 =>1,
        n => {
            if map.contains_key(&n) {
                *map.get(&n).unwrap()
            } else {
                let val = fibonacci_dynamic(n-1, map) + fibonacci_dynamic(n-2, map);
                map.insert(n, val);
                val
            }
        }
    }
}

fn main() {
    let mut map = HashMap::new();
    for i in 1..UPTO {
        println!("{}: {}", i, fibonacci_dynamic(i, &mut map));
    }
}
