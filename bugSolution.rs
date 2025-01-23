fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let mut iter = vec.iter();

    println!("First element: {:?}", iter.next());
    println!("Second element: {:?}", iter.next());

    // Correct way to handle iterator exhaustion: check if next() returns None
    if let Some(element) = iter.next() {
        println!("Third element: {:?}", element);
    } else {
        println!("Iterator exhausted!");
    }
} 