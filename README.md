# Beautiful Day Counter (Custom HUD) per Horizon

Port nativo in Rust di Custom HUD / Beautiful Day Counter per
[Horizon](https://github.com/horizon-rust/horizon), la riscrittura in Rust di Minecraft
Java Edition 26.3. Su Horizon i jar Fabric/NeoForge non girano: questa è una *native mod*
(`horizon-mixins`) che il launcher compila dentro il gioco.

## Funzioni

È la traduzione diretta di `DayHud` della versione Java: stesse coordinate, stessi
sprite vanilla e stesse icone.

- Contatore dei giorni con l'icona della modalità (cuore, cuore hardcore, erba, spada,
  occhio di ender); mela d'oro o totem nei giorni traguardo, con testo arcobaleno e suono.
  "Traguardo ogni": i classici (50, 365 e ogni 100) oppure ogni 10, 25, 50, 100, 250,
  365, 500 o 1000 giorni.
- Orologio a 24 ore o 12 ore con l'icona dell'orologio, o del letto quando puoi dormire.
- Barra giorno/notte con lo sprite della bossbar vanilla e i 4 temi di colore.
- Conto alla rovescia, fase lunare, tacche della notte, sfondo.
- Meteo: una nuvoletta a pixel dopo l'orologio quando piove, con gocce, neve o fulmine
  secondo il tempo e il bioma dove sei.
- Avviso phantom: dopo 3 giorni senza dormire, la membrana di phantom con i giorni
  dall'ultima dormita (statistica `time_since_rest`, chiesta al server ogni 10 secondi).
- Dimensione, opacità, posizione in alto/in basso, spostamento.
- Le bossbar vanilla scendono sotto l'HUD quando è in alto.
- Nether ed End: ora dell'Overworld, `??:??` oppure nascosti.
- Testi in inglese, italiano, tedesco, spagnolo, francese e portoghese.
- Modalità semplice: solo "Giorno: N", dove vuoi sullo schermo (Posizione X/Y in
  percentuale, oppure trascinato col mouse con "Sposta col mouse", frecce per i ritocchi),
  tutto in Elementi accanto all'interruttore.
- Tasto **H** (modificabile in Options > Mods) per aprire le impostazioni.

La schermata delle impostazioni è la `ConfigScreen` Java: titolo, schede Elementi /
Aspetto / Comportamento, griglia a due colonne di pulsanti e slider vanilla, tooltip,
Ripristina e Fatto, con il mondo sfocato e il gioco in pausa. Si apre col tasto o dal
pulsante nella pagina della mod in **Options > Mods** (come Mod Menu). Le impostazioni
sono salvate in `horizon-mods/.horizon-mod-data/customhud/config`, con i nomi dei campi
di `customhud.json`.

## Struttura

```text
mixins/customhud/
  horizon-mixin.toml   5 moduli + 9 hook HEAD
  runtime.rs           l'HUD e lo stato della mod (riesporta gli altri tre moduli)
  config.rs            la configurazione: valori, limiti, lettura e scrittura
  config_screen.rs     la logica della schermata: widget, etichette, clic, slider
  text.rs              lingue e traduzioni
  screen.rs            lato gioco: disegna HUD e schermata, gestisce l'input
  hud.rs               native_hud: l'HUD
  key.rs               native_key: il tasto apre la schermata
  mods.rs              native_mods: voce in Options > Mods
  settings.rs          native_settings: pulsante che apre la schermata
  settings_click.rs    native_settings_click
  dialog.rs            draw_dialog_warning (annullabile): disegna la schermata
  input.rs             window_event (annullabile): mouse e tastiera della schermata
  boss_bars.rs         draw_boss_bars: segna dove iniziano le bossbar vanilla
  extras.rs            draw_extras: le sposta sotto l'HUD quando è in alto
src/lib.rs             la libreria Cargo autonoma: gli stessi quattro file di logica,
                       fratelli come nel gioco, con i loro test
compatibility.json     revisioni fissate di Horizon e horizon-mixins
```

## Compilare e provare

Il modo più semplice è il launcher di Horizon, che usa una sua toolchain Rust:

```sh
horizon-launcher native-build mixins/customhud --trust-native-code
```

oppure dalla pagina **Mods** del launcher. La prima build richiede alcuni minuti.

Test della logica senza Horizon (serve Rust 1.95):

```sh
cargo test --workspace --locked
```

Gli script in `tools/` (stage/check/build contro un checkout di Horizon) vengono dal
[template](https://github.com/horizon-rust/horizon-native-mod-template) e richiedono
Linux e Python 3.12.

## Pubblicare

`python3 tools/package.py` crea `target/customhud-1.8.0.hnmod`, da pubblicare come
release GitHub e inviare al registro
[horizon-mods](https://github.com/horizon-rust/horizon-mods) come `native-mod`.

Licenza: [MIT](LICENSE).
