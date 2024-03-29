/*
* Paragrafo 10.2 I generics
* I generics vengono utilizzati per creare definizioni per oggetti come le firme di funzione o le
* struct, che possiamo riutilizzare con molti e differenti tipi di dati concreti.
* Talvolta, quando si scrive una funzione o un tipo di dati, potremmo volere che funzioni per più tipi di argomenti.
* In Rust, lo possiamo fare usando i generici.
* */
fn main() {
    let number_list = vec![34, 445, 222, 876, 9, 10];
    let result = largest_i32(&number_list);
    println!("Il numero più grande della lista è: {}", result);
    let char_list = vec!['a', 'b', 'z', 'y', 'q'];
    // posso chiamarlo di nuovo per via della proprietà ownership
    let result = largest_char(&char_list);
    println!("La lettera più grande della lista è: {}", result);
    // test della nuova funzione con generics su i32:
    // slice di numeri
    let result = largest(&number_list);
    println!(
        "Uso una nuova funzione.\n Il numero più grande della lista è: {}",
        result
    );
    // test della nuova funzione con generics su char
    let result = largest(&char_list);
    println!(
        "Uso una nuova funzione.\n Il char più grande della lista è: {}",
        result
    );

    let integer = Point { x: 10, y: 10 }; // <-- tipi uguali
                                          // Point viene reinizializzato ad ogni dichiarazione.
    let float = Point { x: 10.2, y: 10.3 }; // <-- tipi uguali
}

/* Per comprendere bene i generics scriviamo due funzioni: una che calcoli l'i32 più grande
 * all'interno di un vettore composto da i32 e una che calcoli il char più grande all'interno di un
 * vettore composto da char. Queste due funzioni poi, potranno essere riscritte in un'unica
 * funzione utilizzando i generics. I generics vengono associati ad una proprietà di alcuni
 * linguaggi chiamata POLIMORFISMO.
 */

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/* A questo punto queste due funzioni, possono essere riscritte in una sola, utilizzando i
 * generics, o, come già detto, il polimorfismo.
 * Per utilizzare un generic occorre parametrizzare entrambi i parametri richiesti nelle "funzioni"
 * di cui rimuovere i duplicati. Solitamente, i programmatori Rust utilizzano la lettera T come
 * parametro generico, per convenzione.
 * Le due funzioni, infatti, possono essere riscritte in una in questo modo:
 */

// le due funzioni diventano semplicemente largest e poi usiamo T come parametro.

// Questa signature di funzione restituirà un errore in fase di compilazione. Questo perché nel
// corpo funzione andiamo ad utilizzare una comparazione binaria. Essendo T un tipo generico
// DOBBIAMO GARANTIRE che le funzionalità espresse nel corpo funzione siano applicabili ai tipi che
// andiamo presumibilmente a coinvolgere. In questo specifico caso, il compilatore Rust ci
// consiglia di dichiarare la T nelle virgolette angolari come std::cmp::PartialOrd, che è un trait
// che vedremo più avanti e permette la comparazione di char e di numeri.
// fn largest<T>(list: &[T]) -> &T {

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/* 10.2.2 Generics nelle definizioni delle struct
* I generics possono essere utilizzati allo stesso modo nella definizione di una struct con
* all'interno dei campi generics.
* Anche in questo caso si utilizzerà per convenzione la lettera T.
* N.B. Anche se viene dichiarato <T> come tipo generico della struct, QUALSIASI tipo venga
* utilizzato per un campo della struct dev'essere LO STESSO per TUTTI.
* */

struct Point<T> {
    x: T,
    // se x "dovesse diventare" un i32, anche y dovrà "diventare" un i32.
    y: T,
}
/*
 * È comunque possibile, dichiarare una struct con due generics di tipo diverso, utilizzando la U
 * insieme alla T, come si vede dal codice in basso.
 * È bene comunque specificare che NON È concesso utilizzare "molti generics" in una struct. Se ci
 * si dovesse trovare in tale situazione è consigliabile pensare a un refactoring del codice.
 * */
struct SPoint<T, U> {
    x: T,
    y: U,
}

/* 10.2.3 Generics nelle definizioni Enum */
