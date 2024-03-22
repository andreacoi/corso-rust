use std::fs::File;
// Per gestire gli errori recuperabili RUST utilizza la enum Result<T, E> dove T ed E sono
// parametri di tipo generico. Solitamente è un modo per stabilire il comportamento del programma
// nei due stati previsti:
// - se OK comportati in un certo modo,
// - se ERR comportati in un altro.

enum Result<T, E> {
    Ok(T),
    Err(E),
}
fn main() {
    let greetings_file = File::open("file.txt");
}
