fn main() {
    // 15.1 Utilizzo di Box<T>
    // Box<T> è uno degli smart pointer più comuni e più immediati che possiamo incontrare in
    // Rust. Viene utilizzato in tre occasioni:
    // 1. Quando si utilizza un tipo dato la cui dimensione non può essere conosciuta al momento
    //    della compilazione e si vuole utilizzare un valore di quel tipo in un contesto che
    //    richiede quella esatta dimensione.
    // 2. Quando vuoi trasferire la "proprietà" di una ingente quantità di dati e vuoi essere certo che non venga implementata la capia degli stessi;
    // 3. Quando vuoi acquisire possesso "own" di un valore e ti interessa solo del fatto che quel
    //    tipo implementi un tratto particolare più del tipo che effettivamente quel valore
    //    rappresenta.
    // SINTASSI BASE DI Box<T>
    let b = Box::new(5);
    println!("b: {b}");
    // Box è una reference che punta al volore 5, che viene allocato come i32 nell'heap, quando
    // normalmente dovrebbe essere allocato nello stack.
}
