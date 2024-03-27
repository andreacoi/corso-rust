# Repo per il commit globale degli esempi divisi per capitoli del libro su RUST

Ho realizzato questo repo (open) per raccogliere tutti gli esempi che ho realizzato studiando la documentazione ufficiale di RUST.

## Documentazione di riferimento

La documentazione di riferimento, il codice che trovate qui - ove possibile tradotto in italiano - sono stati realizzati partendo da qui: https://doc.rust-lang.org/book/title-page.html e dalla corrispettiva versione cartacea acquistata qui: https://www.amazon.it/dp/1718503105?psc=1&ref=ppx_yo2ov_dt_b_product_details

## Nomenclatura e modalità di inserimento dei vari paragrafi

Tutti i progettini (divisi in cartelle) fanno riferimento ai singoli paragrafi della documentazione. Tutti i paragrafi (ad eccezione del primo _hello world_) sono realizzati utilizzando `cargo new nome-del-paragrafo`. Volendo organizzare maggiormente il progetto ho deciso, una volta terminato lo studio del paragrafo attuale, di prependere al nome della cartella un numero sequenziale di due o più cifre. Lo faccio una volta terminato lo studio perché cargo non supporta i numeri nella denominazione del progetto. _Es. `cargo new 01-hello-world` non è un nome di progetto valido. Cargo, quindi, restituisce errore durante la creazione dello stesso_.
Ogni progetto cargo crea automaticamente un repo git alla creazione del progetto. Per focalizzarmi sul progetto di formazione e non sui singoli progettini ho deciso di non eseguire commit nelle cartelle di progetto.

## Divisione degli argomenti all'interno dello stesso progetto

Almeno per quanto riguarda i primi progetti, tutti gli esempi vengono racchiusi all'interno di una funzione `main`. Invece di creare un progetto per ogni esempio, divido i vari esempi con delle sezioni di commento che spiegano cosa andrà a fare quella specifica porzione di main. In questo modo ho:

- il main che parla dei tipi dati;
- il main che parla della caratteristica _ownership_;
- ...

## Compilazione dei progetti

Eseguo la compilazione di un progetto per farmi restituire risultati o errori, secondo quanto prevede la documentazione. Lo faccio in entrambi i casi perché così riesco ad assimilare i concetti meglio e il comportamento del linguaggio sull'errore.
**Nota bene: la compilazione dei progetti va a buon fine se e solo se il nome della cartella del progetto è identico a quanto dichiarato durante la creazione del cargo.**
_Es. se creo il progetto con: cargo new hello-world e poi rinomino la cartella in 01-hello-world, la compilazione non andrà a buon fine._
Dato il tipo di sintassi utilizzata e specificata nel paragrafo _Nomenclatura e modalità di inserimento dei vari paragrafi_, se si vogliono rendere funzionanti gli esempi riportati in questo repo, sarà sufficiente rimuovere la parte numerica dal nome del cargo e procedere con cargo run (_Es. 01-hello-world diventa hello-world_).

## Conclusioni e appunti futuri

Se dovessi avere ulteriori appunti durante la fase di apprendimento li riporterò in questo file LEGGIMI. Visto che il repo è pubblico vi prego di essere clementi con il mio modo di scrivere codice RUST (i'm a n00b) e di esserlo ancor di più con la grammatica nei commenti.

## Edits

In data 25/02/24 ho eseguito una revisione nell'organizzazione dei capitoli e dei paragrafi avendo cura di farli corrispondere.

## Note e altri spunti oltre a "The Book"

In data 27/03/24 ho scoperto un repo Github, a cura di Carlo Milanesi, che riporta i paragrafi presenti nel libro ("The Book"), interamente tradotti. Il libro che ho utilizzato per la comprensione dei concetti più complessi è riportato a [questo link](https://carlomilanesi.github.io/linguaggio-rust/README.html). Ringrazio [Carlo Milanesi](https://github.com/carlomilanesi) che nella mia più onesta opinione ha compiuto un vero e proprio miracolo con la traduzione di "The Book".
