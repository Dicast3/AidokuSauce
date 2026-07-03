# 🍕AidokuSauce🍕

Prima di cominciare, ecco la risposta alla domanda che non si faceva nessuno a parte il sottoscritto, per correttezza vi rendo partecipi: [ai Team sta bene questa roba?](https://github.com/Dicast3/AidokuSauce/blob/main/Non-dev/Moralit%C3%A0.md)

La mia personalissima repo con solo fonti in italiano realizzata con un bel template ~~rubato~~ preso in prestito dalla [community source](https://github.com/Aidoku-Community/sources).
Ho modificato un paio di cose rispetto al template originale in `models` e in `imp` e adesso tutti i moduli mostrano correttamente i vari manga quando viene chiamata la funzione `get_all_mangas`. \n
Per le fonti realizzate su Madara ho scoperto che formattando un qualsiasi capitolo aggiungendo alla fine del link questa stringa: `?style=list`, si risolve in toto il problema dei capitoli che mostrano solo la prima immagine. Ho riadattato la funzione `parse_chapter_element` per tutti i moduli madara-based.

Tutte le fonti che _NON_ ho realizzato/curato si trovano qui per uno di questi motivi:
* sono/erano fonti italiane
* vengono usate per lo studio (sto ancora imparando Rust)
* vengono archiviate perché lo spazio di archiviazione non lo pago io :p

## Roadmap

### Futuro (Pizzareader based)
- [x] Nei filtri, al posto di "Autore" correggere con "Valutazione"
- [ ] Fare in modo che i filtri di genere non si rompano
- [x] Fare in modo che si veda lo status per tutti i manga, se presente
- [ ] Aggiungere filtri STATUS nella barra di ricerca

### Futuro (Madara e Mangathemesia based)
#### ShinobiScans
- [x] Realizzare il modulo
- [x] Realizzare filtri
- [ ] Capire cosa triggera il blocco

Note per Shinobiscans:
Ogni volta che viene fatta una richiesta ad una pagina non presente sul sito l'host blocca tutti gli accessi per 24 ore, per questo motivo ho deciso di mantenere i filtri e la fonte all'essenziale, se funziona non lo toccare.
Dovrei anche decidere se includere <sub>(e in quel caso implementare un modo per renderizzarle su Aidoku)</sub> le novel(s), per il momento rimarranno lì nella fonte a dare errori, non bloccano il normale funzionamento della fonte

#### BeyondTheAtaraxia
- [x] Realizzare il modulo
- [x] Realizzare filtri
#### Rama Oriental Fansub
- [x] Realizzare il modulo
- [x] Aggiustare il modulo (alcuni capitoli si caricano, altri no, altre volte leggi un singolo capitolo e te li segna tutti)
#### Studiocomix
- [ ] Realizzare il modulo (login necessario non implementato su Madara)
#### Aine team
- [ ] Realizzare il modulo (login necessario non implementato su Mangathemesia)
#### Scanduzioni
- [x] Realizzare il modulo
- [ ] Mettere a posto l'ordine dei capitoli (l'ordine di lettura viene visualizzato al contrario nel migliore dei casi)
- [ ] Aggiornare i filtri per genere (devo prendere gli id dal sito per ogni tag (id:1056 name:azione), una cosa di questo tipo)
- [ ] Modificare la home e aggiungere filtri genere

### Futuro (altri)
- [ ] Realizzare il modulo per JJTR
- [ ] Realizzare il modulo per Digital Team
- [ ] Realizzare il modulo per AGC


## Installazione

Copia e incolla questi link nelle fonti di Aidoku

Stabile (per tutti)

https://raw.githubusercontent.com/Dicast3/AidokuSauce/refs/heads/main/index.min.json

Sperimentale (per addetti ai lavori)

https://raw.githubusercontent.com/Dicast3/AidokuSauce/refs/heads/main/index2.min.json

