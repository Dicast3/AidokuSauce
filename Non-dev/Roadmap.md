## Pizzareader based
- [x] Nei filtri, al posto di "Autore" correggere con "Valutazione"
- [ ] Fare in modo che i filtri di genere non si rompano
- [x] Fare in modo che si veda lo status per tutti i manga, se presente
- [ ] Aggiungere filtri STATUS nella barra di ricerca

## Madara based
#### ShinobiScans
- [x] Realizzare il modulo
- [x] Realizzare filtri
- [ ] Capire cosa triggera il blocco

Nuovi aggiornamenti per Shinobiscans:
Confrontandomi con i proprietarii del sito è stato confermato che il modulo basato su madara fa scraping con chiamate ajax, troppe, che vengono inevitabilmente bloccte in toto per non far crashare il sito. Ho provato a chiedere in che modo vengono hostati i lavori ma non ho ricevuto risposta (curiosità personale), peccato però è comprensibile.
L'ultima idea che mi è venuta per non far morire questo modulo potrebbe essere raccogliere tutti i link principali (solo i link ai manga) in un file .json, anche hostato qui, e poi creare una funzione che obblighi il modulo a scartare qualsiasi input che non sia corrispondente a quello che sta in lista.
È una soluzione che non approvo perché farebbe perdere tutto il senso di avere un modulo, uno dovrebbe stare come un cane da caccia a vedere se il sito aggiunge roba nuova e aggiornare il .json o fare un automazione, in ogni caso snaturando il progetto e aggiungendo del lavoro extra che potrebbe portare di fatto a niente di guadagnato.

Note per Shinobiscans:
Ogni volta che viene fatta una richiesta ad una pagina non presente sul sito l'host blocca tutti gli accessi per 24 ore, per questo motivo ho deciso di mantenere i filtri e la fonte all'essenziale, se funziona non lo toccare.
Dovrei anche decidere se includere <sub>(e in quel caso implementare un modo per renderizzarle su Aidoku)</sub> le novel(s), per il momento rimarranno lì nella fonte a dare errori, non bloccano il normale funzionamento della fonte

#### BeyondTheAtaraxia
- [x] Realizzare il modulo
- [x] Realizzare filtri
#### Rama Oriental Fansub
- [x] Realizzare il modulo
- [x] Aggiustare il modulo (alcuni capitoli si caricano, altri no, altre volte leggi un singolo capitolo e te li segna tutti)

## Mangathemesia based
#### Studiocomix
- [ ] Realizzare il modulo (login necessario non implementato su Madara)
#### Aine team
- [ ] Realizzare il modulo (login necessario non implementato su Mangathemesia)
#### Scanduzioni
- [x] Realizzare il modulo
- [ ] Mettere a posto l'ordine dei capitoli (l'ordine di lettura viene visualizzato al contrario nel migliore dei casi)
- [ ] Aggiornare i filtri per genere (devo prendere gli id dal sito per ogni tag (id:1056 name:azione), una cosa di questo tipo)
- [ ] Modificare la home e aggiungere filtri genere

## Altri
- [ ] Realizzare il modulo per JJTR
- [ ] Realizzare il modulo per Digital Team
- [ ] Realizzare il modulo per AGC
