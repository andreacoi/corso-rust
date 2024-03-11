// dichiarando un modulo in questo modo il compilatore rust andrà a cercare un file chiamato
// front_of_house.rs nella cartella src, in questo specifico caso ./front_of_house.rs
// Moduli figli o "child module"
// Nel momento in cui abbiamo dei moduli figli di un altro modulo, possiamo "estrarli" dal modulo
// padre, creando una cartella chiamata come il modulo padre e nominando il modulo padre come
// mod.rs e i moduli figli con il loro rispettivo nome. OLD STYLE, ANCORA VALIDO MA CONFUSIONARIO.
// Conviene creare un file padre in src come front_of_house.rs e una cartella (sempre front_of_house) contenente i figli. Devono quindi essere presenti entrambi.
mod front_of_house;

