use std::thread;

fn main() {
    let numbers = vec![1, 2, 3];
    thread::spawn(move || {
        for n in numbers {
            println!("{n}");
        }
    })
    .join()
    .unwrap();

    println!("Main!!");
}

fn f() {
    println!("Hello thread!!");

    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
