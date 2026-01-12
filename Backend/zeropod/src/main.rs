use zeropod::run;

#[tokio::main]
async fn main() -> std::io::Result<()>{
    run()
}




//Application app is where all your application logic lives:
// routing, middlewares, request handlers, etc..fn main() {

/*
Future trait: a future stands for a value that may not be there yet.fn main()

Asynchronous programming in Rust is built on top of the
Future trait: a future stands for a value thay may not be there
yet.

Technically main cannot be asynchronous function: who is
in charge to call poll on it?

*/



