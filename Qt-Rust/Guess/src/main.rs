#[macro_use]
extern crate qmlrs;

fn main() {
    println!("Hello, world!");
    let mut engine = qmlrs::Engine::new();

    //engine.set_property("chat", Chat::new());
    engine.load_local_file("mainwindow.ui");
    engine.exec();
}
