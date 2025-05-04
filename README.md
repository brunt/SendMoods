# SendMoods

## Send whatever fits your mood 🤪

Inspired by [Sendme](https://www.iroh.computer/sendme), this aims to be a cross-platform 👉👈peer-to-peer👉👈 file transfer application for the people ~~and their moods~~.

This application is very WIP, currently exploring possibilities with wasm compilation and in-browser operation

### What I'm tryna do

* chunk the file in some way
* send chunks to send function and incrementally do whatever it's doing

### Things to study
* Can the send function accept and work with chunks?
* wasm alternative to `tokio::fs::create_dir_all(&blobs_data_dir).await?;` (browser storage?)
* put off streaming large files for later, get an MVP working
* can iroh blobs use browser local storage? (not yet)
* remake import() function in sendme
* use indexeddb instead of localstorage
* use a webworker to not block the UI on upload



