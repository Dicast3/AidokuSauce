# 🍕AidokuSauce🍕

Prima di cominciare, ecco la risposta alla domanda che non si faceva nessuno a parte il sottoscritto, per correttezza vi rendo partecipi: [ai Team sta bene questa roba?](https://github.com/Dicast3/AidokuSauce/blob/main/Non-dev/Moralit%C3%A0.md)

La mia personalissima repo con solo fonti in italiano realizzata con un bel template ~~rubato~~ preso in prestito dalla [community source](https://github.com/Aidoku-Community/sources).
Ho modificato un paio di cose rispetto al template originale in `models` e in `imp` e adesso tutti i moduli mostrano correttamente i vari manga quando viene chiamata la funzione `get_all_mangas`.
Per le fonti realizzate su Madara ho scoperto che formattando un qualsiasi capitolo aggiungendo alla fine del link questa stringa: `?style=list`, si risolve in toto il problema dei capitoli che mostrano solo la prima immagine. Ho riadattato la funzione `parse_chapter_element` per tutti i moduli madara-based.

Tutte le fonti che _NON_ ho realizzato/curato si trovano qui per uno di questi motivi:
* sono/erano fonti italiane
* vengono usate per lo studio (sto ancora imparando Rust)
* vengono archiviate perché lo spazio di archiviazione non lo pago io :p

## [Roadmap](https://github.com/Dicast3/AidokuSauce/blob/main/Non-dev/Roadmap)


## Installazione

Copia e incolla questi link nelle fonti di Aidoku

Stabile (per tutti)

https://raw.githubusercontent.com/Dicast3/AidokuSauce/refs/heads/main/index.min.json

Sperimentale (per addetti ai lavori)

https://raw.githubusercontent.com/Dicast3/AidokuSauce/refs/heads/main/index2.min.json

