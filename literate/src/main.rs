use literate::bin_search;

fn main() {
    let target = 'a';
    let collection = ['a', 'b', 'c'];

    println!("Target: {}", target);
    print!("Collection: ");
    for i in 0..collection.len() {
        print!("{}, ", collection[i]);
    }
    println!("");

    match bin_search(&target, &collection) {
        Some(i) => println!("Result: {}", collection[i]),
        None => println!("Not found")
    }
}
