# Capitolo 15 - Smart Pointer

Gli smart pointer sono dei puntatori avanzati. I puntatori sono dei "concetti generali" che indicano "variabili che puntano a indirizzi di memoria". Nei capitoli e nei paragrafi precedenti mi sono riferito spesso alle reference. Ecco, le reference sono un altro nome dei puntatori.

## Differenze reference vs. puntatori smart

Le reference sono dei semplici puntamenti ad indirizzi di memoria. Anche gli smart pointer si comportano in questo modo. Con alcune differenze che vado a elencare:

- si _comportano_ come reference, ma a differenza di queste sono strutture dati:
  - hanno metadata aggiuntivi;
  - hanno capacità aggiuntive;
  - sono proprietari dei dati presenti nell'indirizzo di memoria a cui puntano.

## Tipi di smart pointer comuni

Abbiamo già incontrato alcuni smart pointer:

- le stringhe;
- Vec<T>;
- Box<T>;
