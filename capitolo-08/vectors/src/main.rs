fn main() {
    // I Vettori
    // Un vettore è una struttura dati che permette di memorizzare dati dello stesso tipo in
    // maniera consequenziale. Per creare un vettore si utilizza la seguente sintassi:
    let v: Vec<i32> = Vec::new();
    // Questo è il modo pià corretto, perchè è ben scritto, riporta il tipo degli elementi che vi
    // saranno contenuti. Tuttavia è possibile utilizzare la macro vec! per inizializzare un
    // vettore per inferenza (deduzione del tipo a carico del compilatore derivanti dai valori
    // dichiarati nel vettore). In questo caso, non avendo specificato dei valori nel vettore
    // l'inferenza è impossibile.
    // Inizializzazione vettore per inferenza
    let v1 = vec![1, 2, 3];
    // La sintassi è molto più stringata e semplice. Specificando come valori 1,2,3 Rust riesce a
    // capire che ci troviamo davanti a un vettore probabilmente <i32>.
    // Altre note
    // Nelle virgolette a spina di pesce possiamo specificare qualsiasi tipo.
}
