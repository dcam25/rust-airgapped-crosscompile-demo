use std::env;

fn main() {
    let mut counter: u64 = 0;

    match env::args().nth(1).as_deref() {
        Some("count") => {
            let target: u64 = env::args()
                .nth(2)
                .and_then(|s| s.parse().ok())
                .unwrap_or(10);

            while counter < target {
                counter += 1;
                println!("{counter}");
            }
        }
        _ => println!("Hello, world!"),
    }
}
