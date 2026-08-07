# HARM2

Workspace Rust (`crates/app_core`, `crates/web`, `crates/desktop`) + frontend Solid/Tiptap servito da Vite.

## Come lavorare in questo repo

**Leggi il sorgente installato prima di proporre codice che usa una libreria.**
`node_modules/<pkg>/dist/index.js` è la verità per la versione che gira qui; la
documentazione online è spesso ferma a versioni precedenti. Vale soprattutto per
Tiptap/ProseMirror, dove le API sono cambiate fra v2 e v3.

**Usa quello che la libreria offre già.** Prima di scrivere posizionamento,
gestione dello stato o cicli di vita a mano, cerca se il pacchetto li espone.
Esempio: `@tiptap/suggestion` fornisce `props.mount`, che fa da solo append,
`computePosition`, `autoUpdate` e chiusura al click fuori.

**DOM diretto quando l'elemento nasce e muore a ogni uso.** Popup, menu
fluttuanti e simili non hanno stato da mantenere: `document.createElement` è più
corto e più chiaro che farli passare per i signal di Solid. Solid serve dove
c'è davvero reattività da conservare.

**Codice breve, senza impalcature difensive.** Niente astrazioni introdotte "per
dopo". Se una cosa si fa in dieci righe, dieci righe.

## Modifiche al codice

Non modificare i file sorgente di tua iniziativa: proponi il codice in chat e
lascia applicare all'utente. Fanno eccezione i file che ti vengono chiesti
esplicitamente.

## Lingua

Italiano, in chat e nei commenti.

## Rust

Gli errori sono classificati una volta sola in `app_core::responses::Error`, e
resi da due newtype (`AppError` per HTML, `ApiError` per JSON). I `From`
esistono perché `?` faccia la conversione da solo: non aggiungere `map_err` ai
punti di chiamata.

I test stanno in `crates/web/src/tests.rs` e girano sulle rotte via
`tower::ServiceExt::oneshot`, su un pool SQLite in memoria con
`max_connections(1)`.
