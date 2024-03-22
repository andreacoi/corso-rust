// Per gestire gli errori recuperabili
enum Result<T, E> {
    Ok(T),
    Err(E),
}
fn main() {
    println!("Hello, world!");
}
