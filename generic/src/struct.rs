struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let wont_work = Point { x: 2, y: 3.4 };
}
