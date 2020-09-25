fn compare(a: u32, b: u32) {
    if a >= b {
        return
    }
    panic!("Should fail")
}

fn main() {
    let a = 0;
    let b = 0;
    compare(a, b);
}
