fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
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

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['a', 'n', 'd', 'i'];

    let result = largest_char(&char_list);
    println!("The largest number is {}", result);
}
