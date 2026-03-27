fn main() {
    let x = 5; //immutable - this is a promise to the compiler
    let mut y = 10; //mutable

    //x = 6;
    y = 10; // correct : y was declared mut

    println!("x={}, y={}", x, y);
}
