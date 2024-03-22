# Capitolo 9 - La gestione degli errori in RUST

Gli errori sono parte integrante dello sviluppo software. Non bisogna aver paura di essi. Quello che bisogna fare è capire come gestirli per garantire all'utente un'esperienza d'uso sicura e priva di imprevisti. È necessario gestire i "panic", che fanno crashare il programma (Errori _unrecoverable_) e quelli più soft che causano dei bug nelle logiche di funzionamento (Errori _recoverable_).

Questo capitolo tratta proprio i tipi di errori descritti e le modalità di recupero degli stessi.
