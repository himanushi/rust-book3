use std::thread;

fn main() {
    thread::spawn(f);
    thread::spawn(f);

    println!("Main!!");
}

fn f() {
    println!("Hello thread!!");

    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
