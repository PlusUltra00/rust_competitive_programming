pub fn read<T: std::str::FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("invalid input.");
    match buf.trim().parse::<T>() {
        Ok(_ok) => _ok,
        Err(_) => panic!("invalid input")
    }
}

fn main() {
    println!("Hello, world!");
}
