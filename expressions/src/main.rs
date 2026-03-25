fn main() {
    for i in 0..20 {
        println!("{}", i);
    }

    let range = std::ops::Range { start: 0, end: 20 };
    for i in range {
        println!("{}", i);
    }
}
