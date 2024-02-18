fn main() {
    let numero = 7;
    // le condizioni di un if devono sempre essere valori bool, nel caso in cui non lo siano, il
    // codice restituisce un errore in fase di compilazione.
    // Questa è una funzione chiave di RUST. Infatti il linguaggio è super stringente da questo
    // punto di vista e NON tenterà un cast di tipo (es. da int a bool).
    if numero < 5 {
        println!("Hello, world!");
    } else {
        println!("Goodbye cruel world!");
    }
    // condizioni multiple - sintassi più comoda del PHP, non c'è necessità di parentesi.
    // RUST esegue il codice finché non trova la prima espressione VERA. POI INTERROMPE
    // L'ESECUZIONE DI QUEL BYTECODE.
    if numero % 2 == 0 {
        println!("D2");
    } else if numero % 3 == 0 {
        println!("D3");
    } else if numero % 4 == 0 {
        println!("D4");
    } else if numero % 5 == 0 {
        println!("D5");
    } else {
        println!("Primo!");
    }
    // Using if in a let Statement
    // È possibile utilizzare un if statement all'interno di una dichiarazione variabile con let.
    //
    let condition = true;
    // il blocco emette il valore dell'espressione tra le parentesi graffe solo se il tipo degli
    // output viene dichiarato sia nell'if sia nell'else. E deve essere necessariamente lo stesso
    // tipo. In questo caso infatti emette questo errore
    // let numero2 = if condition { 5 } else { "six" };
    //                                         ^^^^^ expected integer, found `&str`
    //                           ^^^^^ expected because of this
    // Il primo tipo, infatti è un u32 (5), quell'espressione else, una striga. Questo non può
    // funzionare. Tradotto in breve: if ed else hanno tipi incompatibili.
    // dichiarazione valida:
    let numero2 = if condition { 5 } else { 6 };

    println!("Numero 2 è uguale a: {numero2}");

    // con questo tipo di costrutto il valore è ritornato dalle espressione dopo l'if e dopo l'else
    // in una sola riga.

    // Iterazioni con i loop
    // Questo loop produce un loop infinito di stampa della stringa.
    // loop {
    //   println!("gino");
    //}
    // È possibile interrompere il ciclo loop inserendo un break nel codice.
    loop {
        println!("gino");
        break;
    }

    // In questo snippet, inizializzo counter come variabile mutabile a 0.
    // Poi ASSOCIO a result il risultato di un loop che si comporta in questo modo:
    // ad ogni giro aumenta counter di 1. Se counter == 10, interrompi il ciclo raddoppiando
    // counter.
    // all'uscita mostra il risultato del contatore.

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Il contatore è: {result}");

    // In questo snippet è possibile assegnare un nome ad ogni singolo loop con questa sintassi:
    // 'nome_loop: loop {}
    // è utile in caso di loop innestati. Come si può vedere più avanti nel codice, il break per
    // interrompere il codice riporta il nome del loop, in questo modo break 'primo_contatore;

    let mut contatore = 0;
    'primo_contatore: loop {
        println!("contatore: {contatore}");
        let mut rimanenti = 10;
        loop {
            println!("rimanenti:{rimanenti}");
            if rimanenti == 9 {
                break;
            }
            if contatore == 2 {
                break 'primo_contatore;
            }
            rimanenti -= 1;
        }
        contatore += 1;
    }
    println!("Stato contatore: {contatore}");

    // In questo snippet utilizzo il costrutto while. Il costrutto while è utile per effettuare dei
    // cicli con una valutazione previa del valore utilizzato per il check. Se il valore è vero
    // eseguo, altrimenti fermo. A differenza del costrutto con il loop che prima esegue e poi
    // verifica lo stato, questo prima verifica lo stato e poi esegue.

    let mut numero3 = 3;

    while numero3 != 0 {
        println!("Numero: {numero3}");
        numero3 -= 1;
    }
    println!("stop.");

    // for-loop --> foreach
    // Benché sia possibile iterare all'interno di un array CON UNA DIMENSIONE DEFINITA utilizzando
    // un while loop, a volte questo tipo di approccio potrebbe evolvere in un panic. Questo
    // succede se si cerca di iterare su una lista di elementi senza conoscere il limite della
    // lista. Ci viene in aiuto il ciclo for, con il quale è possibile iterare degli elementi di
    // una lista, quando questi non sono definiti.
}
