fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 24, 100, 99];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['a', 'n', 'd', 'i'];

    let result = largest(&char_list);
    println!("The largest number is {}", result);
}
