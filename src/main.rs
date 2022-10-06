fn main() {
    let x: i8 = 1;
    let y: i32 = 200;
    println!("{}, {}", x, y);

    let arr: [i32; 4];
    arr = [1, 2, 3, 4];

    println!("{}", arr[0]);

    let alive: bool = true;
    let person: (i8, bool) = (16, alive);

    println!("{}, {}", person.0, person.1);
}
