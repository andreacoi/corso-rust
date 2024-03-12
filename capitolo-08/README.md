# Capitolo 8 - Le collezioni

Le collezioni sono strutture dati complesse ed aggregate facenti parte della standard library di RUST. La differenza tra gli array e le tuple e le collezioni è che i primi vengono allocati nello stack, le seconde invece assumono dimensioni dinamiche a runtime e pertanto vengono allocati nell'heap e la loro dimensione può cambiare continuamente (crescendo o rimpicciolendosi) a runtime.
I tipi di collezioni, in rust, sono 3:

- I vettori;
- Le stringhe (sì, intendo String::from("blabla"));
- Le hashmap

## I vettori

I vettori consentono di effettuare uno storage seriale (uno di seguito all'altro) di un numero variabile di valori.

## Le stringhe

Le stringhe, quelle dichiarate con String::from("blabla"), sono anch'esse delle collezioni.

## Le hashmap

È la stessa cosa di una mappa JSON, consente di associare una chiave ad un valore. Si tratta di un tipo particolare di struttura dati che solitamente viene chiamata mappa.
