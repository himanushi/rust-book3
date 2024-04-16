use std::thread;

fn main() {
    let message = "Hello from the thread!".to_string();

    let handle = thread::spawn(move || {
        println!("{}", message);
    });

    handle.join().unwrap();
}
