// Beautiful Day Counter (Custom HUD) per Horizon.
//
// Questo file è sia la libreria Cargo autonoma (testabile senza Horizon) sia il
// modulo `crate::game::customhud` iniettato nel gioco. È la traduzione di
// `DayHud`, `CustomHudConfig` e `ConfigScreen` della versione Java, senza nulla
// che dipenda dal gioco: calcola cosa disegnare (rettangoli, testi, sprite
// vanilla, icone, widget) con le stesse coordinate intere. `screen.rs` legge il
// gioco, disegna e gestisce l'input.

use std::sync::Mutex;

pub const MOD_ID: &str = "customhud";
pub const TITLE: &str = "Beautiful Day Counter 1.7.0";
pub const SETTING_KEY: &str = "config";
pub const CONFIG_KEY: &str = "open_config";
pub const CONFIG_KEY_DEFAULT: &str = "key.keyboard.h";
pub const LEVEL_UP_SOUND: &str = "minecraft:entity.player.levelup";
pub const CLICK_SOUND: &str = "minecraft:ui.button.click";

// Chiavi dell'atlante GUI di Horizon: gli sprite di `blitSprite` stanno sotto `gui/sprites/`.
const HEART_FULL: &str = "gui/sprites/hud/heart/full";
const HARDCORE_HEART_FULL: &str = "gui/sprites/hud/heart/hardcore_full";
const BOSS_BAR_BACKGROUND: &str = "gui/sprites/boss_bar/blue_background";
const BAR_WHITE: &str = "gui/sprites/boss_bar/white_progress";

const BAR_WIDTH: i32 = 182;
const BAR_HEIGHT: i32 = 5;
const TOP_BAR_Y: i32 = 25;
// Di quanto vengono spostate in basso le bossbar vanilla quando l'HUD è in alto, a scala 100%.
const BOSS_BAR_SHIFT: i32 = 35;

// Colori della barra (RGB) per tick da mezzanotte. Le tappe intermedie (aurora,
// sole del mattino) evitano che l'interpolazione RGB passi per toni grigi o marroni.
const COLOR_TICKS: [i64; 7] = [0, 4500, 6000, 7500, 12000, 18000, 24000];
const CLASSIC: [u32; 7] = [
    0x6633CC, 0xFF7A4D, 0xFFE633, 0xFFF5C0, 0x3399FF, 0xFF4D80, 0x6633CC,
];
const PASTEL: [u32; 7] = [
    0x9F8FD9, 0xFFB59E, 0xFFF0A0, 0xFFFBE0, 0xA8D8FF, 0xFFB3C8, 0x9F8FD9,
];
// Palette Okabe-Ito: distinguibile anche con daltonismo rosso/verde
const COLORBLIND: [u32; 7] = [
    0x2B3A8C, 0xE69F00, 0xF0E442, 0xF5EEA0, 0x56B4E9, 0xD55E00, 0x2B3A8C,
];
const MONO: [u32; 7] = [
    0x3A3A3A, 0x8A8A8A, 0xD0D0D0, 0xEEEEEE, 0xFFFFFF, 0x9A9A9A, 0x3A3A3A,
];

// Finestra in cui si può dormire con tempo sereno (tick del giorno, come in vanilla).
const SLEEP_START: i64 = 12542;
const SLEEP_END: i64 = 23460;

const MOON_SIZE: i32 = 8;

pub const MIN_SCALE: i32 = 75;
pub const MAX_SCALE: i32 = 150;
pub const MIN_OPACITY: i32 = 20;
pub const MAX_OPACITY: i32 = 100;
pub const MAX_OFFSET: i32 = 100;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BarTheme {
    Classic,
    Pastel,
    Colorblind,
    Mono,
}

// Cosa mostrare di ora e barra fuori dall'Overworld, dove il ciclo giorno/notte non c'è.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OtherDimensions {
    Show,
    Unknown,
    Hide,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

// ---- Lingue: i file `assets/customhud/lang/*.json` della versione Java ----

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lang {
    En,
    It,
    De,
    Es,
    Fr,
    Pt,
}

impl Lang {
    // Il codice lingua delle impostazioni del gioco (`it_it`, `en_us`...).
    pub fn from_code(code: &str) -> Self {
        match code.get(..2).unwrap_or("") {
            "it" => Lang::It,
            "de" => Lang::De,
            "es" => Lang::Es,
            "fr" => Lang::Fr,
            "pt" => Lang::Pt,
            _ => Lang::En,
        }
    }
}

// Ordine: en_us, it_it, de_de, es_es, fr_fr, pt_br.
type Tr = [&'static str; 6];

const TRANSLATIONS: &[(&str, Tr)] = &[
    ("hud.customhud.day", ["Day: %s", "Giorno: %s", "Tag: %s", "Día: %s", "Jour : %s", "Dia: %s"]),
    ("hud.customhud.night_in", ["Night in %s", "Notte tra %s", "Nacht in %s", "Noche en %s", "Nuit dans %s", "Noite em %s"]),
    ("hud.customhud.day_in", ["Day in %s", "Giorno tra %s", "Tag in %s", "Día en %s", "Jour dans %s", "Dia em %s"]),
    ("key.customhud.open_config", [
        "Open Beautiful Day Counter settings",
        "Apri le impostazioni di Beautiful Day Counter",
        "Beautiful-Day-Counter-Einstellungen öffnen",
        "Abrir ajustes de Beautiful Day Counter",
        "Ouvrir les paramètres de Beautiful Day Counter",
        "Abrir configurações do Beautiful Day Counter",
    ]),
    ("customhud.config.title", ["Beautiful Day Counter"; 6]),
    ("customhud.config.section.elements", ["Elements", "Elementi", "Elemente", "Elementos", "Éléments", "Elementos"]),
    ("customhud.config.section.look", ["Look", "Aspetto", "Aussehen", "Aspecto", "Apparence", "Aparência"]),
    ("customhud.config.section.behavior", ["Behavior", "Comportamento", "Verhalten", "Comportamiento", "Comportement", "Comportamento"]),
    ("customhud.config.days", ["Day counter", "Contatore giorni", "Tageszähler", "Contador de días", "Compteur de jours", "Contador de dias"]),
    ("customhud.config.days.tooltip", [
        "Show the current day with the game mode icon.",
        "Mostra il giorno attuale con l'icona della modalità di gioco.",
        "Zeigt den aktuellen Tag mit dem Spielmodus-Symbol.",
        "Muestra el día actual con el icono del modo de juego.",
        "Affiche le jour actuel avec l'icône du mode de jeu.",
        "Mostra o dia atual com o ícone do modo de jogo.",
    ]),
    ("customhud.config.clock", ["Clock", "Orologio", "Uhr", "Reloj", "Horloge", "Relógio"]),
    ("customhud.config.clock.tooltip", [
        "Show the in-game time.",
        "Mostra l'ora del gioco.",
        "Zeigt die Spielzeit.",
        "Muestra la hora del juego.",
        "Affiche l'heure du jeu.",
        "Mostra o horário do jogo.",
    ]),
    ("customhud.config.bossbar", ["Day/night bar", "Barra giorno/notte", "Tag/Nacht-Leiste", "Barra día/noche", "Barre jour/nuit", "Barra dia/noite"]),
    ("customhud.config.bossbar.tooltip", [
        "Show a colored bar tracking the day/night cycle.",
        "Mostra una barra colorata che segue il ciclo giorno/notte.",
        "Zeigt eine farbige Leiste für den Tag-Nacht-Zyklus.",
        "Muestra una barra de color que sigue el ciclo día/noche.",
        "Affiche une barre colorée qui suit le cycle jour/nuit.",
        "Mostra uma barra colorida que acompanha o ciclo dia/noite.",
    ]),
    ("customhud.config.format", ["Time format", "Formato ora", "Zeitformat", "Formato de hora", "Format horaire", "Formato de hora"]),
    ("customhud.config.format.tooltip", [
        "Use the 12-hour (AM/PM) or 24-hour format.",
        "Usa il formato a 12 ore (AM/PM) o a 24 ore.",
        "12-Stunden- (AM/PM) oder 24-Stunden-Format.",
        "Usa el formato de 12 horas (AM/PM) o de 24 horas.",
        "Utilise le format 12 heures (AM/PM) ou 24 heures.",
        "Usa o formato de 12 horas (AM/PM) ou 24 horas.",
    ]),
    ("customhud.config.format.12h", ["12h"; 6]),
    ("customhud.config.format.24h", ["24h"; 6]),
    ("customhud.config.position", ["Position", "Posizione", "Position", "Posición", "Position", "Posição"]),
    ("customhud.config.position.tooltip", [
        "Place the HUD at the top or above the hotbar.",
        "Posiziona l'HUD in alto o sopra la hotbar.",
        "HUD oben oder über der Schnellleiste anzeigen.",
        "Coloca el HUD arriba o encima de la barra rápida.",
        "Place le HUD en haut ou au-dessus de la barre d'action.",
        "Coloca o HUD no topo ou acima da hotbar.",
    ]),
    ("customhud.config.position.top", ["Top", "In alto", "Oben", "Arriba", "En haut", "Topo"]),
    ("customhud.config.position.bottom", ["Bottom", "In basso", "Unten", "Abajo", "En bas", "Embaixo"]),
    ("customhud.config.night_ticks", ["Night marks", "Tacche notte", "Nachtmarken", "Marcas de noche", "Repères de nuit", "Marcas da noite"]),
    ("customhud.config.night_ticks.tooltip", [
        "Small marks on the bar where the night starts and ends.",
        "Piccole tacche sulla barra dove inizia e finisce la notte.",
        "Kleine Markierungen auf der Leiste, wo die Nacht beginnt und endet.",
        "Pequeñas marcas en la barra donde empieza y termina la noche.",
        "Petits repères sur la barre là où la nuit commence et finit.",
        "Pequenas marcas na barra onde a noite começa e termina.",
    ]),
    ("customhud.config.sleep_icon", ["Bed when sleepy", "Letto se puoi dormire", "Bett zum Schlafen", "Cama para dormir", "Lit pour dormir", "Cama para dormir"]),
    ("customhud.config.sleep_icon.tooltip", [
        "The clock icon turns into a bed when you can sleep.",
        "L'icona dell'orologio diventa un letto quando puoi dormire.",
        "Das Uhr-Symbol wird zum Bett, wenn du schlafen kannst.",
        "El icono del reloj se convierte en una cama cuando puedes dormir.",
        "L'icône de l'horloge devient un lit quand tu peux dormir.",
        "O ícone do relógio vira uma cama quando você pode dormir.",
    ]),
    ("customhud.config.moon_phase", ["Moon phase", "Fase lunare", "Mondphase", "Fase lunar", "Phase lunaire", "Fase da lua"]),
    ("customhud.config.moon_phase.tooltip", [
        "Show the current moon phase. Useful for slimes, cats and farms.",
        "Mostra la fase della luna. Utile per slime, gatti e farm.",
        "Zeigt die aktuelle Mondphase. Nützlich für Schleime, Katzen und Farmen.",
        "Muestra la fase de la luna. Útil para slimes, gatos y granjas.",
        "Affiche la phase de la lune. Utile pour les slimes, les chats et les fermes.",
        "Mostra a fase da lua. Útil para slimes, gatos e farms.",
    ]),
    ("customhud.config.countdown", ["Countdown", "Conto alla rovescia", "Countdown", "Cuenta atrás", "Compte à rebours", "Contagem regressiva"]),
    ("customhud.config.countdown.tooltip", [
        "Show the real time left until night (or until day).",
        "Mostra il tempo reale che manca alla notte (o al giorno).",
        "Zeigt die echte Zeit bis zur Nacht (oder bis zum Tag).",
        "Muestra el tiempo real que falta para la noche (o el día).",
        "Affiche le temps réel restant avant la nuit (ou le jour).",
        "Mostra o tempo real que falta para a noite (ou o dia).",
    ]),
    ("customhud.config.scale", ["Size", "Dimensione", "Größe", "Tamaño", "Taille", "Tamanho"]),
    ("customhud.config.scale.tooltip", [
        "Make the whole HUD bigger or smaller.",
        "Ingrandisce o rimpicciolisce tutto l'HUD.",
        "Macht das ganze HUD größer oder kleiner.",
        "Hace todo el HUD más grande o más pequeño.",
        "Agrandit ou réduit tout le HUD.",
        "Deixa todo o HUD maior ou menor.",
    ]),
    ("customhud.config.opacity", ["Opacity", "Opacità", "Deckkraft", "Opacidad", "Opacité", "Opacidade"]),
    ("customhud.config.opacity.tooltip", [
        "Make text and bar more see-through. Item icons stay solid.",
        "Rende testo e barra più trasparenti. Le icone degli oggetti restano piene.",
        "Macht Text und Leiste durchsichtiger. Gegenstandssymbole bleiben deckend.",
        "Hace el texto y la barra más transparentes. Los iconos de objetos siguen opacos.",
        "Rend le texte et la barre plus transparents. Les icônes d'objets restent opaques.",
        "Deixa o texto e a barra mais transparentes. Os ícones de itens continuam sólidos.",
    ]),
    ("customhud.config.background", ["Background", "Sfondo", "Hintergrund", "Fondo", "Fond", "Fundo"]),
    ("customhud.config.background.tooltip", [
        "A dark see-through panel behind the HUD, easier to read on snow and bright skies.",
        "Un pannello scuro trasparente dietro all'HUD, più leggibile su neve e cielo chiaro.",
        "Ein dunkles, durchsichtiges Feld hinter dem HUD, besser lesbar auf Schnee und hellem Himmel.",
        "Un panel oscuro transparente detrás del HUD, más legible sobre nieve y cielo claro.",
        "Un panneau sombre transparent derrière le HUD, plus lisible sur la neige et un ciel clair.",
        "Um painel escuro transparente atrás do HUD, mais legível na neve e com céu claro.",
    ]),
    ("customhud.config.theme", ["Bar colors", "Colori barra", "Leistenfarben", "Colores de la barra", "Couleurs de la barre", "Cores da barra"]),
    ("customhud.config.theme.tooltip", [
        "Color palette of the day/night bar.",
        "Palette di colori della barra giorno/notte.",
        "Farbpalette der Tag/Nacht-Leiste.",
        "Paleta de colores de la barra día/noche.",
        "Palette de couleurs de la barre jour/nuit.",
        "Paleta de cores da barra dia/noite.",
    ]),
    ("customhud.config.theme.classic", ["Classic", "Classico", "Klassisch", "Clásico", "Classique", "Clássico"]),
    ("customhud.config.theme.pastel", ["Pastel", "Pastello", "Pastell", "Pastel", "Pastel", "Pastel"]),
    ("customhud.config.theme.colorblind", ["Colorblind", "Daltonici", "Farbenblind", "Daltónicos", "Daltoniens", "Daltônicos"]),
    ("customhud.config.theme.mono", ["Mono"; 6]),
    ("customhud.config.offset_x", ["Move sideways", "Sposta di lato", "Seitlich verschieben", "Mover a los lados", "Décaler sur le côté", "Mover para o lado"]),
    ("customhud.config.offset_x.tooltip", [
        "Shift the HUD left (negative) or right (positive).",
        "Sposta l'HUD a sinistra (negativo) o a destra (positivo).",
        "Verschiebt das HUD nach links (negativ) oder rechts (positiv).",
        "Mueve el HUD a la izquierda (negativo) o a la derecha (positivo).",
        "Décale le HUD à gauche (négatif) ou à droite (positif).",
        "Move o HUD para a esquerda (negativo) ou para a direita (positivo).",
    ]),
    ("customhud.config.offset_y", ["Move up/down", "Sposta su/giù", "Hoch/runter", "Mover arriba/abajo", "Monter/descendre", "Mover cima/baixo"]),
    ("customhud.config.offset_y.tooltip", [
        "Shift the HUD up (negative) or down (positive).",
        "Sposta l'HUD in alto (negativo) o in basso (positivo).",
        "Verschiebt das HUD nach oben (negativ) oder unten (positiv).",
        "Mueve el HUD hacia arriba (negativo) o hacia abajo (positivo).",
        "Décale le HUD vers le haut (négatif) ou le bas (positif).",
        "Move o HUD para cima (negativo) ou para baixo (positivo).",
    ]),
    ("customhud.config.dimensions", ["Nether & End", "Nether ed End", "Nether & End", "Nether y End", "Nether et End", "Nether e End"]),
    ("customhud.config.dimensions.tooltip", [
        "There is no day/night cycle there: show the Overworld time, show ??:?? like the vanilla clock, or hide clock and bar.",
        "Lì non c'è il ciclo giorno/notte: mostra l'ora dell'Overworld, ??:?? come l'orologio vanilla, oppure nascondi orologio e barra.",
        "Dort gibt es keinen Tag/Nacht-Zyklus: Oberwelt-Zeit zeigen, ??:?? wie die Vanilla-Uhr, oder Uhr und Leiste ausblenden.",
        "Allí no hay ciclo día/noche: muestra la hora del Overworld, ??:?? como el reloj vanilla, u oculta reloj y barra.",
        "Il n'y a pas de cycle jour/nuit là-bas : afficher l'heure de l'Overworld, ??:?? comme l'horloge vanilla, ou masquer horloge et barre.",
        "Lá não há ciclo dia/noite: mostrar a hora do Overworld, ??:?? como o relógio vanilla, ou esconder relógio e barra.",
    ]),
    ("customhud.config.dimensions.show", ["Overworld time", "Ora Overworld", "Oberwelt-Zeit", "Hora Overworld", "Heure Overworld", "Hora Overworld"]),
    ("customhud.config.dimensions.unknown", ["??:??"; 6]),
    ("customhud.config.dimensions.hide", ["Hide", "Nascondi", "Ausblenden", "Ocultar", "Masquer", "Esconder"]),
    ("customhud.config.milestones", ["Milestones", "Traguardi", "Meilensteine", "Hitos", "Étapes", "Marcos"]),
    ("customhud.config.milestones.tooltip", [
        "Rainbow day counter and special icon on days 50, 100, 365...",
        "Contatore arcobaleno e icona speciale nei giorni 50, 100, 365...",
        "Regenbogen-Zähler und besonderes Symbol an Tag 50, 100, 365...",
        "Contador arcoíris e icono especial en los días 50, 100, 365...",
        "Compteur arc-en-ciel et icône spéciale aux jours 50, 100, 365...",
        "Contador arco-íris e ícone especial nos dias 50, 100, 365...",
    ]),
    ("customhud.config.milestone_sound", ["Milestone sound", "Suono traguardi", "Meilenstein-Ton", "Sonido de hitos", "Son des étapes", "Som dos marcos"]),
    ("customhud.config.milestone_sound.tooltip", [
        "Play a sound when a milestone day starts.",
        "Riproduce un suono quando inizia un giorno traguardo.",
        "Spielt einen Ton, wenn ein Meilenstein-Tag beginnt.",
        "Reproduce un sonido cuando empieza un día hito.",
        "Joue un son quand un jour d'étape commence.",
        "Toca um som quando começa um dia marco.",
    ]),
    ("customhud.config.simple", ["Simple mode", "Modalità semplice", "Einfacher Modus", "Modo simple", "Mode simple", "Modo simples"]),
    ("customhud.config.simple.tooltip", [
        "Show only \"Day: N\", anywhere on the screen: move it with the buttons that appear here.",
        "Mostra solo \"Giorno: N\", dove vuoi sullo schermo: spostalo con i pulsanti che compaiono qui.",
        "Zeigt nur \"Tag: N\", überall auf dem Bildschirm: verschieb es mit den Knöpfen, die hier erscheinen.",
        "Muestra solo \"Día: N\", donde quieras en la pantalla: muévelo con los botones que aparecen aquí.",
        "Affiche seulement « Jour : N », où tu veux à l'écran : déplace-le avec les boutons qui apparaissent ici.",
        "Mostra só \"Dia: N\", onde quiser na tela: mova com os botões que aparecem aqui.",
    ]),
    ("customhud.config.simple_x", ["Position X", "Posizione X", "Position X", "Posición X", "Position X", "Posição X"]),
    ("customhud.config.simple_x.tooltip", [
        "Where the counter goes across the screen: 0% left edge, 100% right edge.",
        "Dove va il contatore in orizzontale: 0% bordo sinistro, 100% bordo destro.",
        "Wo der Zähler waagerecht steht: 0% linker Rand, 100% rechter Rand.",
        "Dónde va el contador en horizontal: 0% borde izquierdo, 100% borde derecho.",
        "Où va le compteur à l'horizontale : 0% bord gauche, 100% bord droit.",
        "Onde fica o contador na horizontal: 0% borda esquerda, 100% borda direita.",
    ]),
    ("customhud.config.simple_y", ["Position Y", "Posizione Y", "Position Y", "Posición Y", "Position Y", "Posição Y"]),
    ("customhud.config.simple_y.tooltip", [
        "Where the counter goes up and down: 0% top edge, 100% bottom edge.",
        "Dove va il contatore in verticale: 0% bordo in alto, 100% bordo in basso.",
        "Wo der Zähler senkrecht steht: 0% oberer Rand, 100% unterer Rand.",
        "Dónde va el contador en vertical: 0% borde superior, 100% borde inferior.",
        "Où va le compteur à la verticale : 0% bord haut, 100% bord bas.",
        "Onde fica o contador na vertical: 0% borda superior, 100% borda inferior.",
    ]),
    ("customhud.config.place", ["Move with mouse", "Sposta col mouse", "Mit der Maus bewegen", "Mover con el ratón", "Déplacer à la souris", "Mover com o mouse"]),
    ("customhud.config.place.tooltip", [
        "Drag the counter wherever you want on the screen.",
        "Trascina il contatore dove vuoi sullo schermo.",
        "Zieh den Zähler an eine beliebige Stelle auf dem Bildschirm.",
        "Arrastra el contador a donde quieras en la pantalla.",
        "Fais glisser le compteur où tu veux à l'écran.",
        "Arraste o contador para onde quiser na tela.",
    ]),
    ("customhud.place.hint", [
        "Drag the counter with the mouse. Arrow keys move it a little. Esc or Done to go back.",
        "Trascina il contatore col mouse. Le frecce lo spostano di poco. Esc o Fatto per tornare.",
        "Zieh den Zähler mit der Maus. Pfeiltasten verschieben ihn ein wenig. Esc oder Fertig zum Zurückkehren.",
        "Arrastra el contador con el ratón. Las flechas lo mueven un poco. Esc o Listo para volver.",
        "Fais glisser le compteur à la souris. Les flèches le déplacent un peu. Échap ou Terminé pour revenir.",
        "Arraste o contador com o mouse. As setas o movem um pouco. Esc ou Concluído para voltar.",
    ]),
    ("customhud.config.reset", ["Reset defaults", "Ripristina predefiniti", "Standard wiederherstellen", "Restablecer", "Réinitialiser", "Restaurar padrão"]),
    ("customhud.mod.description", [
        "Day counter, clock and day/night bar.",
        "Contatore dei giorni, orologio e barra giorno/notte.",
        "Tageszähler, Uhr und Tag/Nacht-Leiste.",
        "Contador de días, reloj y barra día/noche.",
        "Compteur de jours, horloge et barre jour/nuit.",
        "Contador de dias, relógio e barra dia/noite.",
    ]),
];

// `Component.translatable(key)`: la chiave stessa quando manca, come in vanilla.
pub fn tr(key: &str, lang: Lang) -> &str {
    match TRANSLATIONS.iter().find(|(k, _)| *k == key) {
        Some((_, texts)) => texts[lang as usize],
        None => key,
    }
}

// `Component.translatable(key, arg)` per le stringhe con un solo `%s`.
pub fn tr_arg(key: &str, lang: Lang, arg: &str) -> String {
    tr(key, lang).replacen("%s", arg, 1)
}

// I testi vanilla che la schermata usa, letti dalla lingua del gioco.
pub struct Vanilla {
    pub on: String,
    pub off: String,
    pub done: String,
    // `options.generic_value`, "%s: %s".
    pub generic_value: String,
}

// `CommonComponents.optionNameValue(name, value)`.
pub fn option_name_value(vanilla: &Vanilla, name: &str, value: &str) -> String {
    let pattern = &vanilla.generic_value;
    if pattern.contains("%1$s") || pattern.contains("%2$s") {
        pattern.replace("%1$s", name).replace("%2$s", value)
    } else {
        pattern.replacen("%s", name, 1).replacen("%s", value, 1)
    }
}

// ---- Configurazione (`CustomHudConfig`) ----

#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    // Elementi
    pub show_day_counter: bool,
    pub show_clock: bool,
    pub show_bossbar: bool,
    pub show_night_ticks: bool,
    pub show_sleep_icon: bool,
    pub show_moon_phase: bool,
    pub show_night_countdown: bool,
    // Aspetto
    pub scale: i32,
    pub opacity: i32,
    pub show_background: bool,
    pub bar_theme: BarTheme,
    // Comportamento
    pub use_12_hour_format: bool,
    pub position_bottom: bool,
    pub offset_x: i32,
    pub offset_y: i32,
    pub other_dimensions: OtherDimensions,
    pub milestones: bool,
    pub milestone_sound: bool,
    // Modalità semplice: solo "Giorno: N", in un punto qualsiasi dello schermo
    // (percentuale dello spazio libero, da sinistra e dall'alto).
    pub simple_mode: bool,
    pub simple_x: i32,
    pub simple_y: i32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            show_day_counter: true,
            show_clock: true,
            show_bossbar: true,
            show_night_ticks: false,
            show_sleep_icon: false,
            show_moon_phase: false,
            show_night_countdown: false,
            scale: 100,
            opacity: 100,
            show_background: false,
            bar_theme: BarTheme::Classic,
            use_12_hour_format: false,
            position_bottom: false,
            offset_x: 0,
            offset_y: 0,
            other_dimensions: OtherDimensions::Show,
            milestones: true,
            milestone_sound: true,
            simple_mode: false,
            simple_x: 0,
            simple_y: 0,
        }
    }
}

impl Config {
    pub fn is_hud_visible(&self) -> bool {
        self.simple_mode || self.show_day_counter || self.show_clock || self.show_bossbar
    }

    // Riporta nei limiti i valori modificati a mano nel file.
    fn sanitize(&mut self) {
        self.scale = self.scale.clamp(MIN_SCALE, MAX_SCALE);
        self.opacity = self.opacity.clamp(MIN_OPACITY, MAX_OPACITY);
        self.offset_x = self.offset_x.clamp(-MAX_OFFSET, MAX_OFFSET);
        self.offset_y = self.offset_y.clamp(-MAX_OFFSET, MAX_OFFSET);
        self.simple_x = self.simple_x.clamp(0, 100);
        self.simple_y = self.simple_y.clamp(0, 100);
    }

    // Una riga `chiave=valore` per opzione, con i nomi dei campi di `customhud.json`.
    // Chiavi sconosciute o valori non validi vengono ignorati.
    pub fn parse(text: &str) -> Self {
        let mut c = Config::default();
        for line in text.lines() {
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            let (key, value) = (key.trim(), value.trim());
            let flag = value == "true";
            let number = value.parse::<i32>().ok();
            match key {
                "showDayCounter" => c.show_day_counter = flag,
                "showClock" => c.show_clock = flag,
                "showBossbar" => c.show_bossbar = flag,
                "showNightTicks" => c.show_night_ticks = flag,
                "showSleepIcon" => c.show_sleep_icon = flag,
                "showMoonPhase" => c.show_moon_phase = flag,
                "showNightCountdown" => c.show_night_countdown = flag,
                "showBackground" => c.show_background = flag,
                "use12HourFormat" => c.use_12_hour_format = flag,
                "positionBottom" => c.position_bottom = flag,
                "milestones" => c.milestones = flag,
                "milestoneSound" => c.milestone_sound = flag,
                "simpleMode" => c.simple_mode = flag,
                "scale" => c.scale = number.unwrap_or(c.scale),
                "opacity" => c.opacity = number.unwrap_or(c.opacity),
                "offsetX" => c.offset_x = number.unwrap_or(c.offset_x),
                "offsetY" => c.offset_y = number.unwrap_or(c.offset_y),
                "simpleX" => c.simple_x = number.unwrap_or(c.simple_x),
                "simpleY" => c.simple_y = number.unwrap_or(c.simple_y),
                "barTheme" => {
                    c.bar_theme = match value {
                        "PASTEL" => BarTheme::Pastel,
                        "COLORBLIND" => BarTheme::Colorblind,
                        "MONO" => BarTheme::Mono,
                        _ => BarTheme::Classic,
                    }
                }
                "otherDimensions" => {
                    c.other_dimensions = match value {
                        "UNKNOWN" => OtherDimensions::Unknown,
                        "HIDE" => OtherDimensions::Hide,
                        _ => OtherDimensions::Show,
                    }
                }
                _ => {}
            }
        }
        c.sanitize();
        c
    }

    pub fn serialize(&self) -> String {
        let theme = match self.bar_theme {
            BarTheme::Classic => "CLASSIC",
            BarTheme::Pastel => "PASTEL",
            BarTheme::Colorblind => "COLORBLIND",
            BarTheme::Mono => "MONO",
        };
        let dims = match self.other_dimensions {
            OtherDimensions::Show => "SHOW",
            OtherDimensions::Unknown => "UNKNOWN",
            OtherDimensions::Hide => "HIDE",
        };
        let lines = [
            format!("showDayCounter={}", self.show_day_counter),
            format!("showClock={}", self.show_clock),
            format!("showBossbar={}", self.show_bossbar),
            format!("showNightTicks={}", self.show_night_ticks),
            format!("showSleepIcon={}", self.show_sleep_icon),
            format!("showMoonPhase={}", self.show_moon_phase),
            format!("showNightCountdown={}", self.show_night_countdown),
            format!("scale={}", self.scale),
            format!("opacity={}", self.opacity),
            format!("showBackground={}", self.show_background),
            format!("barTheme={theme}"),
            format!("use12HourFormat={}", self.use_12_hour_format),
            format!("positionBottom={}", self.position_bottom),
            format!("offsetX={}", self.offset_x),
            format!("offsetY={}", self.offset_y),
            format!("otherDimensions={dims}"),
            format!("milestones={}", self.milestones),
            format!("milestoneSound={}", self.milestone_sound),
            format!("simpleMode={}", self.simple_mode),
            format!("simpleX={}", self.simple_x),
            format!("simpleY={}", self.simple_y),
        ];
        lines.join("\n") + "\n"
    }
}

// ---- La schermata delle impostazioni (`ConfigScreen`) ----

const BUTTON_WIDTH: i32 = 150;
const SECTION_BUTTON_WIDTH: i32 = 100;
const BUTTON_HEIGHT: i32 = 20;
// `HeaderAndFooterLayout(this, 58, 33)`.
const HEADER_HEIGHT: i32 = 58;
const FOOTER_HEIGHT: i32 = 33;
// `HeaderAndFooterLayout.DEFAULT_CONTENT_MARGIN_TOP`.
const CONTENT_MARGIN_TOP: i32 = 30;
// `Tooltip.MAX_WIDTH`.
pub const TOOLTIP_WIDTH: i32 = 170;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Section {
    Elements,
    Look,
    Behavior,
}

const SECTIONS: [Section; 3] = [Section::Elements, Section::Look, Section::Behavior];

impl Section {
    pub fn label_key(self) -> &'static str {
        match self {
            Section::Elements => "customhud.config.section.elements",
            Section::Look => "customhud.config.section.look",
            Section::Behavior => "customhud.config.section.behavior",
        }
    }

    // In modalità semplice restano solo le opzioni che contano per "Giorno: N";
    // i comandi per spostarlo stanno accanto al suo interruttore.
    fn options(self, c: &Config) -> &'static [Opt] {
        if c.simple_mode {
            return match self {
                Section::Elements => &[Opt::Simple, Opt::Place, Opt::SimpleX, Opt::SimpleY],
                Section::Look => &[Opt::Scale, Opt::Opacity, Opt::Background],
                Section::Behavior => &[Opt::Milestones, Opt::MilestoneSound],
            };
        }
        match self {
            Section::Elements => &[
                Opt::Days,
                Opt::Clock,
                Opt::Bossbar,
                Opt::NightTicks,
                Opt::SleepIcon,
                Opt::MoonPhase,
                Opt::Countdown,
                Opt::Simple,
            ],
            Section::Look => &[Opt::Scale, Opt::Opacity, Opt::Background, Opt::Theme],
            Section::Behavior => &[
                Opt::Format,
                Opt::Position,
                Opt::OffsetX,
                Opt::OffsetY,
                Opt::Dimensions,
                Opt::Milestones,
                Opt::MilestoneSound,
            ],
        }
    }
}

// Un'opzione della schermata: un `CycleButton` o un `IntSlider`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opt {
    Days,
    Clock,
    Bossbar,
    NightTicks,
    SleepIcon,
    MoonPhase,
    Countdown,
    Scale,
    Opacity,
    Background,
    Theme,
    Format,
    Position,
    OffsetX,
    OffsetY,
    Dimensions,
    Milestones,
    MilestoneSound,
    Simple,
    SimpleX,
    SimpleY,
    // Non è un valore: apre lo spostamento col mouse.
    Place,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Slider {
    pub min: i32,
    pub max: i32,
    pub step: i32,
    pub suffix: &'static str,
}

impl Opt {
    fn key(self) -> &'static str {
        match self {
            Opt::Days => "days",
            Opt::Clock => "clock",
            Opt::Bossbar => "bossbar",
            Opt::NightTicks => "night_ticks",
            Opt::SleepIcon => "sleep_icon",
            Opt::MoonPhase => "moon_phase",
            Opt::Countdown => "countdown",
            Opt::Scale => "scale",
            Opt::Opacity => "opacity",
            Opt::Background => "background",
            Opt::Theme => "theme",
            Opt::Format => "format",
            Opt::Position => "position",
            Opt::OffsetX => "offset_x",
            Opt::OffsetY => "offset_y",
            Opt::Dimensions => "dimensions",
            Opt::Milestones => "milestones",
            Opt::MilestoneSound => "milestone_sound",
            Opt::Simple => "simple",
            Opt::SimpleX => "simple_x",
            Opt::SimpleY => "simple_y",
            Opt::Place => "place",
        }
    }

    pub fn slider(self) -> Option<Slider> {
        match self {
            Opt::Scale => Some(Slider { min: MIN_SCALE, max: MAX_SCALE, step: 5, suffix: "%" }),
            Opt::Opacity => Some(Slider { min: MIN_OPACITY, max: MAX_OPACITY, step: 5, suffix: "%" }),
            Opt::OffsetX | Opt::OffsetY => Some(Slider { min: -MAX_OFFSET, max: MAX_OFFSET, step: 1, suffix: "" }),
            Opt::SimpleX | Opt::SimpleY => Some(Slider { min: 0, max: 100, step: 1, suffix: "%" }),
            _ => None,
        }
    }

    fn flag(self, c: &mut Config) -> Option<&mut bool> {
        Some(match self {
            Opt::Days => &mut c.show_day_counter,
            Opt::Clock => &mut c.show_clock,
            Opt::Bossbar => &mut c.show_bossbar,
            Opt::NightTicks => &mut c.show_night_ticks,
            Opt::SleepIcon => &mut c.show_sleep_icon,
            Opt::MoonPhase => &mut c.show_moon_phase,
            Opt::Countdown => &mut c.show_night_countdown,
            Opt::Background => &mut c.show_background,
            Opt::Format => &mut c.use_12_hour_format,
            Opt::Position => &mut c.position_bottom,
            Opt::Milestones => &mut c.milestones,
            Opt::MilestoneSound => &mut c.milestone_sound,
            Opt::Simple => &mut c.simple_mode,
            _ => return None,
        })
    }

    fn number(self, c: &mut Config) -> Option<&mut i32> {
        Some(match self {
            Opt::Scale => &mut c.scale,
            Opt::Opacity => &mut c.opacity,
            Opt::OffsetX => &mut c.offset_x,
            Opt::OffsetY => &mut c.offset_y,
            Opt::SimpleX => &mut c.simple_x,
            Opt::SimpleY => &mut c.simple_y,
            _ => return None,
        })
    }

    pub fn tooltip_key(self) -> String {
        format!("customhud.config.{}.tooltip", self.key())
    }

    // `CycleButton.cycleValue(delta)`: +1 avanti, -1 indietro.
    pub fn cycle(self, c: &mut Config, delta: i32) {
        match self {
            Opt::Theme => {
                let themes = [
                    BarTheme::Classic,
                    BarTheme::Pastel,
                    BarTheme::Colorblind,
                    BarTheme::Mono,
                ];
                c.bar_theme = step(&themes, c.bar_theme, delta);
            }
            Opt::Dimensions => {
                let modes = [
                    OtherDimensions::Show,
                    OtherDimensions::Unknown,
                    OtherDimensions::Hide,
                ];
                c.other_dimensions = step(&modes, c.other_dimensions, delta);
            }
            _ => {
                if let Some(flag) = self.flag(c) {
                    *flag = !*flag;
                }
            }
        }
    }

    // Posizione dello slider (0..1), come `(value - min) / (max - min)`.
    pub fn slider_fraction(self, c: &Config) -> f64 {
        let Some(s) = self.slider() else {
            return 0.0;
        };
        let mut c = c.clone();
        let value = self.number(&mut c).map_or(s.min, |v| *v);
        (value - s.min) as f64 / (s.max - s.min) as f64
    }

    // `IntSlider.current()` per la posizione `fraction`.
    pub fn slider_value(self, fraction: f64) -> i32 {
        let Some(s) = self.slider() else {
            return 0;
        };
        let steps = (fraction.clamp(0.0, 1.0) * (s.max - s.min) as f64 / s.step as f64).round();
        s.min + steps as i32 * s.step
    }

    // `IntSlider.applyValue`.
    pub fn set_slider(self, c: &mut Config, fraction: f64) {
        let value = self.slider_value(fraction);
        if let Some(n) = self.number(c) {
            *n = value;
        }
    }

    // Il testo del widget, es. "Orologio: ON" o "Dimensione: 110%". `drag` è la
    // posizione dello slider mentre lo si trascina.
    pub fn label(self, c: &Config, lang: Lang, vanilla: &Vanilla, drag: Option<f64>) -> String {
        let name_key = format!("customhud.config.{}", self.key());
        let name = tr(&name_key, lang);
        if self == Opt::Place {
            return name.to_owned();
        }
        let value: String = if let Some(s) = self.slider() {
            let fraction = drag.unwrap_or_else(|| self.slider_fraction(c));
            format!("{}{}", self.slider_value(fraction), s.suffix)
        } else {
            match self {
                Opt::Theme => {
                    let key = match c.bar_theme {
                        BarTheme::Classic => "customhud.config.theme.classic",
                        BarTheme::Pastel => "customhud.config.theme.pastel",
                        BarTheme::Colorblind => "customhud.config.theme.colorblind",
                        BarTheme::Mono => "customhud.config.theme.mono",
                    };
                    tr(key, lang).to_owned()
                }
                Opt::Dimensions => {
                    let key = match c.other_dimensions {
                        OtherDimensions::Show => "customhud.config.dimensions.show",
                        OtherDimensions::Unknown => "customhud.config.dimensions.unknown",
                        OtherDimensions::Hide => "customhud.config.dimensions.hide",
                    };
                    tr(key, lang).to_owned()
                }
                Opt::Format => {
                    let key = if c.use_12_hour_format {
                        "customhud.config.format.12h"
                    } else {
                        "customhud.config.format.24h"
                    };
                    tr(key, lang).to_owned()
                }
                Opt::Position => {
                    let key = if c.position_bottom {
                        "customhud.config.position.bottom"
                    } else {
                        "customhud.config.position.top"
                    };
                    tr(key, lang).to_owned()
                }
                _ => {
                    let mut c = c.clone();
                    let on = self.flag(&mut c).is_some_and(|v| *v);
                    if on { vanilla.on.clone() } else { vanilla.off.clone() }
                }
            }
        };
        option_name_value(vanilla, name, &value)
    }
}

fn step<T: Copy + PartialEq>(list: &[T], current: T, delta: i32) -> T {
    let i = list.iter().position(|v| *v == current).unwrap_or(0) as i32;
    list[(i + delta).rem_euclid(list.len() as i32) as usize]
}

// Cosa fa un widget della schermata.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    Section(Section),
    Opt(Opt),
    Reset,
    Done,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Widget {
    pub target: Target,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Widget {
    // `AbstractWidget.isMouseOver`.
    pub fn contains(&self, cursor: (f32, f32)) -> bool {
        let (cx, cy) = cursor;
        cx >= self.x as f32
            && cx < (self.x + self.w) as f32
            && cy >= self.y as f32
            && cy < (self.y + self.h) as f32
    }
}

// `FrameLayout`: `Mth.lerpInt(0.5, 0, free)`.
fn center(free: i32) -> i32 {
    free.div_euclid(2)
}

// La riga dell'intestazione: larghezza dei tre pulsanti di sezione (spaziatura 4).
const SECTIONS_WIDTH: i32 = 3 * SECTION_BUTTON_WIDTH + 2 * 4;

// Dove va il titolo, largo `title_width`: centrato sopra i pulsanti di sezione.
pub fn title_position(width: i32, title_width: i32) -> (i32, i32) {
    let header_width = SECTIONS_WIDTH.max(title_width);
    let x = center(width - header_width) + center(header_width - title_width);
    // Titolo (9) + 8 + pulsanti (20), centrati nell'intestazione alta 58
    (x, center(HEADER_HEIGHT - (9 + 8 + BUTTON_HEIGHT)))
}

// `ConfigScreen.init` + `HeaderAndFooterLayout.arrangeElements`.
pub fn screen_layout(section: Section, c: &Config, width: i32, height: i32) -> Vec<Widget> {
    let mut widgets = Vec::new();

    let header_y = center(HEADER_HEIGHT - (9 + 8 + BUTTON_HEIGHT));
    let row_x = center(width - SECTIONS_WIDTH);
    for (i, s) in SECTIONS.iter().enumerate() {
        widgets.push(Widget {
            target: Target::Section(*s),
            x: row_x + i as i32 * (SECTION_BUTTON_WIDTH + 4),
            y: header_y + 9 + 8,
            w: SECTION_BUTTON_WIDTH,
            h: BUTTON_HEIGHT,
        });
    }

    // GridLayout a 2 colonne, spaziatura 10 x 4; un'ultima opzione dispari
    // occupa entrambe le colonne, centrata.
    let options = section.options(c);
    let rows = options.len().div_ceil(2) as i32;
    let grid_width = 2 * BUTTON_WIDTH + 10;
    let grid_height = rows * BUTTON_HEIGHT + (rows - 1) * 4;
    let grid_x = center(width - grid_width);
    let grid_y = (HEADER_HEIGHT + CONTENT_MARGIN_TOP).min(height - FOOTER_HEIGHT - grid_height);
    for (i, opt) in options.iter().enumerate() {
        let (row, col) = ((i / 2) as i32, (i % 2) as i32);
        let spans = i == options.len() - 1 && options.len() % 2 == 1;
        let x = if spans {
            grid_x + center(grid_width - BUTTON_WIDTH)
        } else {
            grid_x + col * (BUTTON_WIDTH + 10)
        };
        widgets.push(Widget {
            target: Target::Opt(*opt),
            x,
            y: grid_y + row * (BUTTON_HEIGHT + 4),
            w: BUTTON_WIDTH,
            h: BUTTON_HEIGHT,
        });
    }

    // Piè di pagina: Ripristina e Fatto, spaziatura 8
    let footer_x = center(width - (2 * BUTTON_WIDTH + 8));
    let footer_y = height - FOOTER_HEIGHT + center(FOOTER_HEIGHT - BUTTON_HEIGHT);
    for (i, target) in [Target::Reset, Target::Done].into_iter().enumerate() {
        widgets.push(Widget {
            target,
            x: footer_x + i as i32 * (BUTTON_WIDTH + 8),
            y: footer_y,
            w: BUTTON_WIDTH,
            h: BUTTON_HEIGHT,
        });
    }
    widgets
}

// Dove torna la schermata alla chiusura (`ConfigScreen.parent`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Parent {
    Game,
    Options,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScreenState {
    pub open: bool,
    // Ricorda l'ultima sezione aperta finché il gioco resta acceso.
    pub section: Section,
    pub parent: Parent,
    // Lo slider trascinato e la sua posizione.
    pub drag: Option<(Opt, f64)>,
    // Spostamento col mouse del contatore semplice: aperto, e dove lo si è
    // afferrato rispetto al suo angolo (mentre il tasto è premuto).
    pub placing: bool,
    pub grab: Option<(f32, f32)>,
}

// Cosa deve fare `screen.rs` dopo un input.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Effect {
    pub click_sound: bool,
    pub save: bool,
    pub close: bool,
}

// Il valore di uno slider dove si trova il mouse (`setValueFromMouse`).
fn slider_at(w: &Widget, cursor_x: f32) -> f64 {
    let track = (w.w - 8) as f64;
    ((cursor_x as f64 - (w.x + 4) as f64) / track).clamp(0.0, 1.0)
}

impl State {
    // Clic sinistro (`mouseClicked`). `shift` cicla all'indietro, come `CycleButton`.
    pub fn screen_click(&mut self, widgets: &[Widget], cursor: (f32, f32), shift: bool) -> Effect {
        let mut effect = Effect::default();
        let Some(w) = widgets.iter().find(|w| w.contains(cursor)) else {
            return effect;
        };
        match w.target {
            Target::Section(s) => {
                // Il pulsante della sezione aperta è disattivato
                if s != self.screen.section {
                    self.screen.section = s;
                    effect.click_sound = true;
                }
            }
            Target::Opt(Opt::Place) => {
                self.screen.placing = true;
                self.screen.grab = None;
                effect.click_sound = true;
            }
            Target::Opt(opt) => {
                if opt.slider().is_some() {
                    let fraction = slider_at(w, cursor.0);
                    opt.set_slider(&mut self.config, fraction);
                    self.screen.drag = Some((opt, fraction));
                } else {
                    opt.cycle(&mut self.config, if shift { -1 } else { 1 });
                    effect.click_sound = true;
                    effect.save = true;
                }
            }
            Target::Reset => {
                self.config = Config::default();
                effect.click_sound = true;
                effect.save = true;
            }
            Target::Done => {
                effect.click_sound = true;
                effect.close = true;
            }
        }
        effect
    }

    // Il mouse si muove con lo slider premuto (`onDrag`).
    pub fn screen_drag(&mut self, widgets: &[Widget], cursor: (f32, f32)) {
        let Some((opt, _)) = self.screen.drag else {
            return;
        };
        if let Some(w) = widgets.iter().find(|w| w.target == Target::Opt(opt)) {
            let fraction = slider_at(w, cursor.0);
            opt.set_slider(&mut self.config, fraction);
            self.screen.drag = Some((opt, fraction));
        }
    }

    // Rilascio del tasto sinistro: lo slider suona e si salva (`onRelease`).
    pub fn screen_release(&mut self) -> Effect {
        let released = self.screen.drag.take().is_some();
        Effect {
            click_sound: released,
            save: released,
            close: false,
        }
    }

    // Rotellina su un `CycleButton`: su = indietro, giù = avanti.
    pub fn screen_scroll(&mut self, widgets: &[Widget], cursor: (f32, f32), up: bool) -> Effect {
        let hovered = widgets.iter().find(|w| w.contains(cursor));
        if let Some(Widget { target: Target::Opt(opt), .. }) = hovered
            && opt.slider().is_none()
            && *opt != Opt::Place
        {
            opt.cycle(&mut self.config, if up { -1 } else { 1 });
            return Effect {
                save: true,
                ..Effect::default()
            };
        }
        Effect::default()
    }
}

// Va a capo come `Font.split`, entro `max_width` pixel.
pub fn wrap(text: &str, max_width: i32, text_width: &dyn Fn(&str) -> i32) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line = String::new();
    for word in text.split(' ') {
        let candidate = if line.is_empty() {
            word.to_owned()
        } else {
            format!("{line} {word}")
        };
        if !line.is_empty() && text_width(&candidate) > max_width {
            lines.push(std::mem::take(&mut line));
            line = word.to_owned();
        } else {
            line = candidate;
        }
    }
    if !line.is_empty() {
        lines.push(line);
    }
    lines
}

// ---- Stato della mod nel gioco ----

pub struct State {
    pub config: Config,
    pub screen: ScreenState,
    // Per il suono dei traguardi: l'ultimo giorno visto e quando (ms). Una pausa
    // lunga vuol dire mondo nuovo, come il cambio di `ClientLevel` in Java.
    last_seen_days: i64,
    last_frame_millis: u64,
}

static STATE: Mutex<Option<State>> = Mutex::new(None);

// `load` legge la configurazione salvata, solo la prima volta.
pub fn with_state<R>(
    load: impl FnOnce() -> Option<String>,
    f: impl FnOnce(&mut State) -> R,
) -> R {
    let mut guard = STATE.lock().unwrap_or_else(|e| e.into_inner());
    let state = guard.get_or_insert_with(|| State::new(load().map(|t| Config::parse(&t)).unwrap_or_default()));
    f(state)
}

// Il primo vertice delle bossbar vanilla nel frame in corso (vedi `boss_bars.rs`).
static BOSS_BARS_MARK: Mutex<Option<usize>> = Mutex::new(None);

pub fn mark_boss_bars(start: usize) {
    *BOSS_BARS_MARK.lock().unwrap_or_else(|e| e.into_inner()) = Some(start);
}

pub fn take_boss_bars_mark() -> Option<usize> {
    BOSS_BARS_MARK.lock().unwrap_or_else(|e| e.into_inner()).take()
}

impl State {
    pub fn new(config: Config) -> Self {
        State {
            config,
            screen: ScreenState {
                open: false,
                section: Section::Elements,
                parent: Parent::Game,
                drag: None,
                placing: false,
                grab: None,
            },
            last_seen_days: -1,
            last_frame_millis: 0,
        }
    }

    // Suona solo quando il giorno avanza davvero, non quando si entra in un mondo
    // già al traguardo. True se va riprodotto il suono.
    pub fn celebration_sound(&mut self, days: i64, millis: u64, play_sound: bool) -> bool {
        let new_level = millis.saturating_sub(self.last_frame_millis) > 2000;
        self.last_frame_millis = millis;
        if new_level {
            self.last_seen_days = days;
            return false;
        }
        let play = days > self.last_seen_days && play_sound;
        self.last_seen_days = days;
        play
    }
}

// ---- Testi dell'HUD ----

pub fn day_text(days: i64, lang: Lang) -> String {
    tr_arg("hud.customhud.day", lang, &days.to_string())
}

pub fn time_text(time_of_day: i64, use_12h: bool) -> String {
    let hours24 = (time_of_day / 1000 + 6) % 24;
    let minutes = (time_of_day % 1000) * 60 / 1000;
    if use_12h {
        let h = if hours24 % 12 == 0 { 12 } else { hours24 % 12 };
        let suffix = if hours24 < 12 { "AM" } else { "PM" };
        format!("{h}:{minutes:02} {suffix}")
    } else {
        format!("{hours24:02}:{minutes:02}")
    }
}

// "Notte tra 4:20" di giorno, "Giorno tra 1:05" di notte, in tempo reale (minuti:secondi).
pub fn countdown_text(time_of_day: i64, lang: Lang) -> String {
    let night = (SLEEP_START..SLEEP_END).contains(&time_of_day);
    let ticks_left = if night {
        SLEEP_END - time_of_day
    } else {
        (SLEEP_START - time_of_day).rem_euclid(24000)
    };
    let seconds = ticks_left / 20;
    let left = format!("{}:{:02}", seconds / 60, seconds % 60);
    let key = if night { "hud.customhud.day_in" } else { "hud.customhud.night_in" };
    tr_arg(key, lang, &left)
}

pub fn is_milestone(days: i64) -> bool {
    days > 0 && (days == 50 || days == 365 || days % 100 == 0)
}

// Nell'Overworld si dorme di notte o durante un temporale.
pub fn can_sleep(overworld: bool, thundering: bool, time_of_day: i64) -> bool {
    overworld && (thundering || (SLEEP_START..SLEEP_END).contains(&time_of_day))
}

// ---- Colori ----

pub fn bar_color(theme: BarTheme, ticks: i64) -> u32 {
    let colors = match theme {
        BarTheme::Classic => &CLASSIC,
        BarTheme::Pastel => &PASTEL,
        BarTheme::Colorblind => &COLORBLIND,
        BarTheme::Mono => &MONO,
    };
    let mut i = 1;
    while i < COLOR_TICKS.len() - 1 && ticks >= COLOR_TICKS[i] {
        i += 1;
    }
    let t = (ticks - COLOR_TICKS[i - 1]) as f32 / (COLOR_TICKS[i] - COLOR_TICKS[i - 1]) as f32;
    lerp_rgb(colors[i - 1], colors[i], t)
}

fn lerp_rgb(a: u32, b: u32, t: f32) -> u32 {
    let ch = |shift: u32| {
        let (x, y) = (((a >> shift) & 0xFF) as f32, ((b >> shift) & 0xFF) as f32);
        (x + t * (y - x)) as u32 & 0xFF
    };
    (ch(16) << 16) | (ch(8) << 8) | ch(0)
}

// `Mth.hsvToRgb`.
fn hsv_to_rgb(hue: f32, saturation: f32, value: f32) -> u32 {
    let h = (hue * 6.0) as i32 % 6;
    let f = hue * 6.0 - h as f32;
    let p = value * (1.0 - saturation);
    let q = value * (1.0 - f * saturation);
    let t = value * (1.0 - (1.0 - f) * saturation);
    let (r, g, b) = match h {
        0 => (value, t, p),
        1 => (q, value, p),
        2 => (p, value, t),
        3 => (p, q, value),
        4 => (t, p, value),
        _ => (value, p, q),
    };
    let c = |x: f32| ((x * 255.0) as i32).clamp(0, 255) as u32;
    (c(r) << 16) | (c(g) << 8) | c(b)
}

// ---- Disposizione dell'HUD (`DayHud.extract`) ----

// Quello che l'HUD legge dal gioco in questo frame.
pub struct Frame {
    pub day_time: i64,
    // Dimensioni GUI intere, come `guiWidth()`/`guiHeight()`.
    pub width: i32,
    pub height: i32,
    pub overworld: bool,
    pub thundering: bool,
    pub mode: Mode,
    pub hardcore: bool,
    // L'aria sotto il massimo: le bolle occupano la riga sopra la fame.
    pub low_air: bool,
    // Altezza dal fondo occupata dalle barre di stato (0 = sconosciuta).
    pub status_bar_height: i32,
    pub millis: u64,
    pub lang: Lang,
}

// Colori ARGB come in Java; coordinate in pixel GUI.
#[derive(Clone, Debug, PartialEq)]
pub enum Draw {
    // `GuiGraphics.fill(x0, y0, x1, y1, color)`.
    Fill { x0: i32, y0: i32, x1: i32, y1: i32, argb: u32 },
    // Testo con ombra.
    Text { text: String, x: i32, y: i32, argb: u32 },
    // Sprite GUI vanilla; `u` è la frazione di larghezza dello sprite da usare.
    Sprite { name: &'static str, x: i32, y: i32, w: i32, h: i32, u: f32, argb: u32 },
    // Icona di un oggetto 16x16 ridotta al 60%.
    Item { id: &'static str, x: i32, y: i32 },
}

// Il risultato di `layout`: i disegni e la trasformazione da applicare dopo
// (scala attorno a `pivot`, poi spostamento di `offset`), come il `pose` Java.
pub struct Hud {
    pub draws: Vec<Draw>,
    pub pivot: [f32; 2],
    pub scale: f32,
    pub offset: [f32; 2],
    // Giorni passati, per il suono dei traguardi.
    pub days: i64,
    pub milestone: bool,
}

pub const ITEM_SCALE: f32 = 0.6;

// Di quanto spostare le bossbar vanilla perché non finiscano sotto l'HUD in alto
// (0 = non spostarle).
pub fn boss_bar_shift(c: &Config) -> i32 {
    if !c.is_hud_visible() || c.position_bottom || c.simple_mode {
        return 0;
    }
    ((BOSS_BAR_SHIFT as f32 * c.scale as f32 / 100.0).round() as i32 + c.offset_y).max(0)
}

// Distanza dal fondo dello schermo. Sale ancora se le barre di stato sono più
// alte (molti cuori di assorbimento, armatura).
fn bottom_offset(f: &Frame) -> i32 {
    match f.mode {
        Mode::Spectator => 15,
        Mode::Creative => 45,
        Mode::Survival | Mode::Adventure => {
            let legacy = if f.low_air { 70 } else { 55 };
            if f.status_bar_height > 0 {
                legacy.max(f.status_bar_height + 16)
            } else {
                legacy
            }
        }
    }
}

// Calcola cosa disegnare. `text_width` misura un testo in pixel GUI.
pub fn layout(c: &Config, f: &Frame, text_width: &dyn Fn(&str) -> i32) -> Option<Hud> {
    if c.simple_mode {
        return Some(simple_layout(c, f, text_width));
    }
    // Fuori dall'Overworld l'ora non ha senso: si può nascondere o mostrare "??:??"
    let hide_time = !f.overworld && c.other_dimensions == OtherDimensions::Hide;
    let unknown_time = !f.overworld && c.other_dimensions == OtherDimensions::Unknown;
    let time_extras = !hide_time && !unknown_time;

    let show_days = c.show_day_counter;
    let show_clock = c.show_clock && !hide_time;
    let show_bar = c.show_bossbar && !hide_time;
    let show_countdown = c.show_night_countdown && time_extras;
    let show_moon = c.show_moon_phase && time_extras;
    if !show_days && !show_clock && !show_bar {
        return None;
    }

    let time_of_day = f.day_time.rem_euclid(24000);
    let days = f.day_time.div_euclid(24000);

    let day = day_text(days, f.lang);
    let time = if unknown_time {
        "??:??".to_owned()
    } else {
        time_text(time_of_day, c.use_12_hour_format)
    };
    let countdown = countdown_text(time_of_day, f.lang);
    let (day_w, time_w, countdown_w) = (text_width(&day), text_width(&time), text_width(&countdown));

    let milestone = c.milestones && is_milestone(days);

    let alpha = (c.opacity as f32 * 2.55).round() as u32;
    let white = alpha << 24 | 0xFFFFFF;
    let center_x = f.width / 2;
    let bar_x = (f.width - BAR_WIDTH) / 2;
    let bar_y = if c.position_bottom {
        f.height - bottom_offset(f)
    } else {
        TOP_BAR_Y
    };
    let text_y = bar_y - 12;

    let mut total = 0;
    if show_days {
        total += 13 + day_w;
    }
    if show_clock {
        total += if total > 0 { 15 } else { 0 } + 13 + time_w;
    }
    if show_countdown {
        total += if total > 0 { 6 } else { 0 } + countdown_w;
    }
    if show_moon {
        total += if total > 0 { 6 } else { 0 } + MOON_SIZE;
    }
    let has_text = total > 0;

    // Scala attorno al bordo dello schermo più vicino, poi sposta di quanto scelto
    let anchor_y = if c.position_bottom { bar_y + BAR_HEIGHT } else { 0 };
    let mut draws = Vec::new();

    if c.show_background {
        let width = total.max(if show_bar { BAR_WIDTH } else { 0 });
        let top = if has_text { text_y - 4 } else { bar_y - 3 };
        let bottom = if show_bar { bar_y + BAR_HEIGHT + 3 } else { text_y + 11 };
        draws.push(Draw::Fill {
            x0: center_x - width / 2 - 4,
            y0: top,
            x1: center_x + (width + 1) / 2 + 4,
            y1: bottom,
            argb: (alpha * 0x90 / 0xFF) << 24,
        });
    }

    let mut x = (f.width - total) / 2;

    if show_days {
        let mut color = white;
        if milestone {
            let hue = (f.millis % 4000) as f32 / 4000.0;
            color = hsv_to_rgb(hue, 0.7, 1.0) & 0xFFFFFF | alpha << 24;
        }
        mode_icon(&mut draws, f, x, text_y, days, milestone, alpha);
        x += 13;
        draws.push(Draw::Text { text: day, x, y: text_y, argb: color });
        x += day_w;
    }

    if show_clock {
        if show_days {
            x += 15;
        }
        let sleep_icon =
            c.show_sleep_icon && time_extras && can_sleep(f.overworld, f.thundering, time_of_day);
        let id = if sleep_icon { "minecraft:red_bed" } else { "minecraft:clock" };
        draws.push(Draw::Item { id, x, y: text_y - 1 });
        x += 13;
        draws.push(Draw::Text { text: time, x, y: text_y, argb: white });
        x += time_w;
    }

    if show_countdown {
        if show_days || show_clock {
            x += 6;
        }
        draws.push(Draw::Text { text: countdown, x, y: text_y, argb: alpha << 24 | 0xBBBBBB });
        x += countdown_w;
    }

    if show_moon {
        if show_days || show_clock || show_countdown {
            x += 6;
        }
        moon(&mut draws, x, text_y, days.rem_euclid(8) as i32, alpha);
    }

    if show_bar {
        draws.push(Draw::Sprite {
            name: BOSS_BAR_BACKGROUND,
            x: bar_x,
            y: bar_y,
            w: BAR_WIDTH,
            h: BAR_HEIGHT,
            u: 1.0,
            argb: white,
        });
        if !unknown_time {
            let since_midnight = (time_of_day + 6000) % 24000;
            let progress = (BAR_WIDTH as i64 * since_midnight / 24000) as i32;
            if progress > 0 {
                draws.push(Draw::Sprite {
                    name: BAR_WHITE,
                    x: bar_x,
                    y: bar_y,
                    w: progress,
                    h: BAR_HEIGHT,
                    u: progress as f32 / BAR_WIDTH as f32,
                    argb: alpha << 24 | bar_color(c.bar_theme, since_midnight),
                });
            }
            if c.show_night_ticks {
                for tick in [SLEEP_START, SLEEP_END] {
                    let tx = bar_x + (BAR_WIDTH as i64 * ((tick + 6000) % 24000) / 24000) as i32;
                    draws.push(Draw::Fill {
                        x0: tx,
                        y0: bar_y - 1,
                        x1: tx + 1,
                        y1: bar_y + BAR_HEIGHT + 1,
                        argb: (alpha * 0xB0 / 0xFF) << 24 | 0xFFFFFF,
                    });
                }
            }
        }
    }

    Some(Hud {
        draws,
        pivot: [center_x as f32, anchor_y as f32],
        scale: c.scale as f32 / 100.0,
        offset: [c.offset_x as f32, c.offset_y as f32],
        days,
        milestone,
    })
}

// Margine attorno al testo della modalità semplice, anche senza sfondo, così il
// testo non tocca il bordo e non si sposta quando lo sfondo si accende.
const SIMPLE_PADDING: i32 = 3;

// Il riquadro del contatore semplice: angolo in pixel GUI e dimensioni prima della
// scala (`scale` le moltiplica attorno all'angolo).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SimpleBox {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub scale: f32,
}

impl SimpleBox {
    // Il cursore è sopra il riquadro, scala compresa.
    pub fn contains(&self, (cx, cy): (f32, f32)) -> bool {
        let (x, y) = (self.x as f32, self.y as f32);
        cx >= x && cx < x + self.w as f32 * self.scale && cy >= y && cy < y + self.h as f32 * self.scale
    }
}

// Spazio libero lungo un asse: lo schermo meno il riquadro ingrandito.
fn free_space(screen: i32, size: i32, scale: f32) -> f32 {
    (screen as f32 - size as f32 * scale).max(0.0)
}

pub fn simple_box(c: &Config, width: i32, height: i32, text_w: i32) -> SimpleBox {
    let scale = c.scale as f32 / 100.0;
    let w = text_w + 2 * SIMPLE_PADDING;
    // Testo alto 8 più l'ombra
    let h = 9 + 2 * SIMPLE_PADDING;
    let place = |screen: i32, size: i32, percent: i32| {
        (free_space(screen, size, scale) * percent as f32 / 100.0).round() as i32
    };
    SimpleBox {
        x: place(width, w, c.simple_x),
        y: place(height, h, c.simple_y),
        w,
        h,
        scale,
    }
}

impl State {
    // Spostamento col mouse: il tasto premuto sopra il contatore lo afferra.
    pub fn place_press(&mut self, b: &SimpleBox, cursor: (f32, f32)) -> bool {
        if !b.contains(cursor) {
            return false;
        }
        self.screen.grab = Some((cursor.0 - b.x as f32, cursor.1 - b.y as f32));
        true
    }

    // Il mouse si muove col contatore afferrato: l'angolo segue il cursore e
    // diventa la percentuale dello spazio libero, sempre dentro lo schermo.
    pub fn place_drag(&mut self, b: &SimpleBox, cursor: (f32, f32), width: i32, height: i32) {
        let Some((gx, gy)) = self.screen.grab else {
            return;
        };
        let percent = |pos: f32, screen: i32, size: i32| {
            let free = free_space(screen, size, b.scale);
            if free <= 0.0 {
                0
            } else {
                (pos / free * 100.0).round().clamp(0.0, 100.0) as i32
            }
        };
        self.config.simple_x = percent(cursor.0 - gx, width, b.w);
        self.config.simple_y = percent(cursor.1 - gy, height, b.h);
    }

    // Rilascio: true se il contatore era afferrato (va salvato).
    pub fn place_release(&mut self) -> bool {
        self.screen.grab.take().is_some()
    }

    // Le frecce spostano il contatore di un punto percentuale.
    pub fn place_nudge(&mut self, dx: i32, dy: i32) {
        self.config.simple_x = (self.config.simple_x + dx).clamp(0, 100);
        self.config.simple_y = (self.config.simple_y + dy).clamp(0, 100);
    }

    // Esc o Fatto: si torna alle impostazioni.
    pub fn place_close(&mut self) {
        self.screen.placing = false;
        self.screen.grab = None;
    }
}

// Il pulsante Fatto dello spostamento, in basso al centro come nella schermata.
pub fn place_done(width: i32, height: i32) -> Widget {
    Widget {
        target: Target::Done,
        x: center(width - BUTTON_WIDTH),
        y: height - FOOTER_HEIGHT + center(FOOTER_HEIGHT - BUTTON_HEIGHT),
        w: BUTTON_WIDTH,
        h: BUTTON_HEIGHT,
    }
}

// Modalità semplice: solo "Giorno: N" (arcobaleno nei traguardi), nel punto scelto.
// La percentuale è dello spazio libero, così a 0% e 100% il riquadro tocca il
// bordo e resta tutto sullo schermo a ogni dimensione.
fn simple_layout(c: &Config, f: &Frame, text_width: &dyn Fn(&str) -> i32) -> Hud {
    let days = f.day_time.div_euclid(24000);
    let day = day_text(days, f.lang);
    let milestone = c.milestones && is_milestone(days);
    let alpha = (c.opacity as f32 * 2.55).round() as u32;
    let mut color = alpha << 24 | 0xFFFFFF;
    if milestone {
        let hue = (f.millis % 4000) as f32 / 4000.0;
        color = hsv_to_rgb(hue, 0.7, 1.0) & 0xFFFFFF | alpha << 24;
    }

    let scale = c.scale as f32 / 100.0;
    let b = simple_box(c, f.width, f.height, text_width(&day));
    let (x, y) = (b.x, b.y);
    let (box_w, box_h) = (b.w, b.h);

    let mut draws = Vec::new();
    if c.show_background {
        draws.push(Draw::Fill {
            x0: x,
            y0: y,
            x1: x + box_w,
            y1: y + box_h,
            argb: (alpha * 0x90 / 0xFF) << 24,
        });
    }
    draws.push(Draw::Text {
        text: day,
        x: x + SIMPLE_PADDING,
        y: y + SIMPLE_PADDING,
        argb: color,
    });
    Hud {
        draws,
        pivot: [x as f32, y as f32],
        scale,
        offset: [0.0, 0.0],
        days,
        milestone,
    }
}

fn mode_icon(
    draws: &mut Vec<Draw>,
    f: &Frame,
    x: i32,
    y: i32,
    days: i64,
    milestone: bool,
    alpha: u32,
) {
    let id = match f.mode {
        Mode::Survival => {
            if milestone {
                let id = if days >= 500 {
                    "minecraft:totem_of_undying"
                } else {
                    "minecraft:golden_apple"
                };
                draws.push(Draw::Item { id, x, y: y - 2 });
            } else {
                // Ombra del cuore (nero semitrasparente) + cuore
                let heart = if f.hardcore { HARDCORE_HEART_FULL } else { HEART_FULL };
                let shadow = (alpha / 2) << 24;
                draws.push(Draw::Sprite { name: heart, x: x + 1, y, w: 9, h: 9, u: 1.0, argb: shadow });
                let lit = alpha << 24 | 0xFFFFFF;
                draws.push(Draw::Sprite { name: heart, x, y: y - 1, w: 9, h: 9, u: 1.0, argb: lit });
            }
            return;
        }
        Mode::Creative => "minecraft:grass_block",
        Mode::Adventure => "minecraft:iron_sword",
        Mode::Spectator => "minecraft:ender_eye",
    };
    draws.push(Draw::Item { id, x, y: y - 1 });
}

// Luna 8x8 disegnata riga per riga: parte illuminata chiara, parte in ombra scura.
// Fase 0 = luna piena, 4 = luna nuova (come in vanilla); 1-3 calante, 5-7 crescente.
fn moon(draws: &mut Vec<Draw>, x: i32, y: i32, phase: i32, alpha: u32) {
    let lit = alpha << 24 | 0xF2EFD9;
    let dark = (alpha * 3 / 4) << 24 | 0x2A2A3A;
    let terminator = (phase as f64 * std::f64::consts::PI / 4.0).cos();
    let center = x + MOON_SIZE / 2;
    let mut row = |from: i32, to: i32, top: i32, argb: u32| {
        if from < to {
            draws.push(Draw::Fill { x0: from, y0: top, x1: to, y1: top + 1, argb });
        }
    };
    for r in 0..MOON_SIZE {
        let dy = r as f64 - (MOON_SIZE - 1) as f64 / 2.0;
        let half = ((MOON_SIZE * MOON_SIZE) as f64 / 4.0 - dy * dy).sqrt().round() as i32;
        let split = ((terminator * half as f64).round() as i32).clamp(-half, half);
        let top = y + r;
        if phase <= 4 {
            // Calante: la luce resta a sinistra
            row(center - half, center + split, top, lit);
            row(center + split, center + half, top, dark);
        } else {
            // Crescente: la luce cresce da destra
            row(center - half, center - split, top, dark);
            row(center - split, center + half, top, lit);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(day_time: i64) -> Frame {
        Frame {
            day_time,
            width: 400,
            height: 240,
            overworld: true,
            thundering: false,
            mode: Mode::Survival,
            hardcore: false,
            low_air: false,
            status_bar_height: 0,
            millis: 0,
            lang: Lang::En,
        }
    }

    fn width(text: &str) -> i32 {
        text.chars().count() as i32 * 6
    }

    fn vanilla() -> Vanilla {
        Vanilla {
            on: "ON".into(),
            off: "OFF".into(),
            done: "Done".into(),
            generic_value: "%s: %s".into(),
        }
    }

    fn texts(hud: &Hud) -> Vec<String> {
        hud.draws
            .iter()
            .filter_map(|d| match d {
                Draw::Text { text, .. } => Some(text.clone()),
                _ => None,
            })
            .collect()
    }

    fn items(hud: &Hud) -> Vec<&'static str> {
        hud.draws
            .iter()
            .filter_map(|d| match d {
                Draw::Item { id, .. } => Some(*id),
                _ => None,
            })
            .collect()
    }

    fn widget(widgets: &[Widget], target: Target) -> Widget {
        *widgets.iter().find(|w| w.target == target).expect("widget")
    }

    #[test]
    fn clock_matches_vanilla_time() {
        assert_eq!(time_text(0, false), "06:00");
        assert_eq!(time_text(6000, false), "12:00");
        assert_eq!(time_text(18000, false), "00:00");
        assert_eq!(time_text(18000, true), "12:00 AM");
        assert_eq!(time_text(13500, true), "7:30 PM");
    }

    #[test]
    fn texts_follow_the_language() {
        assert_eq!(day_text(3, Lang::It), "Giorno: 3");
        assert_eq!(day_text(3, Lang::Fr), "Jour : 3");
        assert_eq!(Lang::from_code("pt_br"), Lang::Pt);
        assert_eq!(Lang::from_code("ja_jp"), Lang::En);
        assert_eq!(countdown_text(12542 - 200, Lang::En), "Night in 0:10");
        assert_eq!(countdown_text(23460 - 400, Lang::It), "Giorno tra 0:20");
        assert_eq!(tr("missing.key", Lang::It), "missing.key");
    }

    #[test]
    fn every_option_has_a_name_and_tooltip_in_every_language() {
        let simple = Config { simple_mode: true, ..Config::default() };
        for (section, c) in SECTIONS.iter().flat_map(|s| [(*s, Config::default()), (*s, simple.clone())]) {
            assert_ne!(tr(section.label_key(), Lang::De), section.label_key());
            for opt in section.options(&c) {
                let name = format!("customhud.config.{}", opt.key());
                for lang in [Lang::En, Lang::It, Lang::De, Lang::Es, Lang::Fr, Lang::Pt] {
                    assert_ne!(tr(&name, lang), name);
                    assert_ne!(tr(&opt.tooltip_key(), lang), opt.tooltip_key());
                }
            }
        }
    }

    #[test]
    fn milestones() {
        assert!(is_milestone(50) && is_milestone(365) && is_milestone(200));
        assert!(!is_milestone(0) && !is_milestone(150));
    }

    #[test]
    fn config_round_trips_and_is_sanitized() {
        let c = Config {
            use_12_hour_format: true,
            bar_theme: BarTheme::Mono,
            offset_y: -20,
            scale: 125,
            other_dimensions: OtherDimensions::Unknown,
            milestone_sound: false,
            simple_mode: true,
            simple_x: 80,
            simple_y: 5,
            ..Config::default()
        };
        assert_eq!(Config::parse(&c.serialize()), c);
        let odd = Config::parse("opacity=5\nnonsense\nfoo=bar\noffsetX=999\nscale=10\nsimpleY=150");
        assert_eq!((odd.opacity, odd.offset_x, odd.scale, odd.simple_y), (20, 100, 75, 100));
    }

    #[test]
    fn screen_layout_matches_header_and_footer_layout() {
        // 427x240 è la GUI di una finestra 1280x720 a scala 3
        let widgets = screen_layout(Section::Elements, &Config::default(), 427, 240);
        // Intestazione: titolo (9) + 8 + pulsanti, centrati in 58 -> y 10 e 27
        assert_eq!(title_position(427, 120), (153, 10));
        let elements = widget(&widgets, Target::Section(Section::Elements));
        assert_eq!((elements.x, elements.y, elements.w), (59, 27, 100));
        let behavior = widget(&widgets, Target::Section(Section::Behavior));
        assert_eq!(behavior.x, 59 + 2 * 104);
        // Griglia a y = 58 + 30, colonne da 150 a 10 di distanza
        let days = widget(&widgets, Target::Opt(Opt::Days));
        let clock = widget(&widgets, Target::Opt(Opt::Clock));
        assert_eq!((days.x, days.y, clock.x), (58, 88, 218));
        let bossbar = widget(&widgets, Target::Opt(Opt::Bossbar));
        assert_eq!(bossbar.y, 112);
        // Quarta riga: conto alla rovescia e modalità semplice
        let countdown = widget(&widgets, Target::Opt(Opt::Countdown));
        let simple = widget(&widgets, Target::Opt(Opt::Simple));
        assert_eq!((countdown.x, countdown.y, simple.x, simple.y), (58, 160, 218, 160));
        // In modalità semplice Elementi ha l'interruttore e, accanto, i comandi per spostarlo
        let c = Config { simple_mode: true, ..Config::default() };
        let widgets = screen_layout(Section::Elements, &c, 427, 240);
        assert_eq!(widget(&widgets, Target::Opt(Opt::Simple)).x, 58);
        assert_eq!(widget(&widgets, Target::Opt(Opt::Place)).x, 218);
        assert!(!widgets.iter().any(|w| w.target == Target::Opt(Opt::Days)));
        // Piè di pagina alto 33
        let reset = widget(&widgets, Target::Reset);
        let done = widget(&widgets, Target::Done);
        assert_eq!((reset.x, reset.y, done.x), (59, 213, 217));
    }

    #[test]
    fn small_windows_push_the_grid_up() {
        let widgets = screen_layout(Section::Elements, &Config::default(), 320, 200);
        // 200 - 33 - 92 = 75 < 88
        assert_eq!(widget(&widgets, Target::Opt(Opt::Days)).y, 75);
    }

    #[test]
    fn labels_match_cycle_buttons_and_sliders() {
        let c = Config::default();
        let v = vanilla();
        assert_eq!(Opt::Clock.label(&c, Lang::En, &v, None), "Clock: ON");
        assert_eq!(Opt::NightTicks.label(&c, Lang::It, &v, None), "Tacche notte: OFF");
        assert_eq!(Opt::Scale.label(&c, Lang::It, &v, None), "Dimensione: 100%");
        assert_eq!(Opt::OffsetX.label(&c, Lang::En, &v, None), "Move sideways: 0");
        assert_eq!(Opt::Theme.label(&c, Lang::It, &v, None), "Colori barra: Classico");
        assert_eq!(Opt::Format.label(&c, Lang::En, &v, None), "Time format: 24h");
        let french = Vanilla { generic_value: "%s : %s".into(), ..vanilla() };
        assert_eq!(Opt::Position.label(&c, Lang::Fr, &french, None), "Position : En haut");
    }

    #[test]
    fn sliders_snap_to_their_step() {
        assert_eq!(Opt::Scale.slider_value(0.0), 75);
        assert_eq!(Opt::Scale.slider_value(1.0), 150);
        // 75 + round(0.34 * 75 / 5) * 5 = 75 + 5 * 5
        assert_eq!(Opt::Scale.slider_value(0.34), 100);
        assert_eq!(Opt::OffsetY.slider_value(0.5), 0);
        assert!((Opt::Opacity.slider_fraction(&Config::default()) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn clicks_drags_and_scrolls() {
        let mut s = State::new(Config::default());
        let widgets = screen_layout(Section::Elements, &Config::default(), 427, 240);
        let clock = widget(&widgets, Target::Opt(Opt::Clock));
        let at = |w: Widget| ((w.x + 5) as f32, (w.y + 5) as f32);

        let e = s.screen_click(&widgets, at(clock), false);
        assert!(!s.config.show_clock && e.click_sound && e.save);

        // Sezione già aperta: disattivata
        let elements = widget(&widgets, Target::Section(Section::Elements));
        assert_eq!(s.screen_click(&widgets, at(elements), false), Effect::default());
        let look = widget(&widgets, Target::Section(Section::Look));
        assert!(s.screen_click(&widgets, at(look), false).click_sound);
        assert_eq!(s.screen.section, Section::Look);

        // Slider della dimensione: clic all'inizio, trascina in fondo, rilascia
        let widgets = screen_layout(Section::Look, &Config::default(), 427, 240);
        let scale = widget(&widgets, Target::Opt(Opt::Scale));
        let e = s.screen_click(&widgets, (scale.x as f32, (scale.y + 5) as f32), false);
        assert_eq!((s.config.scale, e.save), (75, false));
        s.screen_drag(&widgets, ((scale.x + scale.w) as f32, (scale.y + 5) as f32));
        assert_eq!(s.config.scale, 150);
        let e = s.screen_release();
        assert!(e.save && e.click_sound && s.screen.drag.is_none());

        // Rotellina su "Colori barra": giù = avanti, su = indietro
        let theme = widget(&widgets, Target::Opt(Opt::Theme));
        s.screen_scroll(&widgets, at(theme), false);
        assert_eq!(s.config.bar_theme, BarTheme::Pastel);
        s.screen_scroll(&widgets, at(theme), true);
        s.screen_scroll(&widgets, at(theme), true);
        assert_eq!(s.config.bar_theme, BarTheme::Mono);
        // Maiuscolo + clic cicla all'indietro
        s.screen_click(&widgets, at(theme), true);
        assert_eq!(s.config.bar_theme, BarTheme::Colorblind);

        let reset = widget(&widgets, Target::Reset);
        assert!(s.screen_click(&widgets, at(reset), false).save);
        assert_eq!(s.config, Config::default());
        let done = widget(&widgets, Target::Done);
        assert!(s.screen_click(&widgets, at(done), false).close);
    }

    #[test]
    fn tooltips_wrap_at_170_pixels() {
        let lines = wrap(tr("customhud.config.background.tooltip", Lang::En), TOOLTIP_WIDTH, &width);
        assert!(lines.len() > 1);
        assert!(lines.iter().all(|l| width(l) <= TOOLTIP_WIDTH));
        assert_eq!(lines.join(" "), tr("customhud.config.background.tooltip", Lang::En));
    }

    #[test]
    fn bar_colors_hit_their_stops() {
        assert_eq!(bar_color(BarTheme::Classic, 0), 0x6633CC);
        assert_eq!(bar_color(BarTheme::Classic, 6000), 0xFFE633);
        assert_eq!(bar_color(BarTheme::Mono, 23999) >> 16, 0x3A);
    }

    #[test]
    fn default_hud_matches_java_layout() {
        let hud = layout(&Config::default(), &frame(24000 * 5 + 6000), &width).unwrap();
        assert_eq!(texts(&hud), ["Day: 5", "12:00"]);
        assert_eq!(items(&hud), ["minecraft:clock"]);
        // total = 13 + 36 + 15 + 13 + 30 = 107; x = (400 - 107) / 2 = 146
        assert!(hud.draws.contains(&Draw::Sprite {
            name: HEART_FULL,
            x: 146,
            y: 12,
            w: 9,
            h: 9,
            u: 1.0,
            argb: 0xFFFFFFFF,
        }));
        assert!(hud.draws.contains(&Draw::Text {
            text: "Day: 5".into(),
            x: 159,
            y: 13,
            argb: 0xFFFFFFFF,
        }));
        // Barra: sfondo + metà avanzamento (mezzogiorno)
        assert!(hud.draws.iter().any(|d| matches!(d, Draw::Sprite { name: BAR_WHITE, w: 91, .. })));
        assert_eq!((hud.pivot, hud.scale), ([200.0, 0.0], 1.0));
    }

    #[test]
    fn mode_and_milestone_icons() {
        let mut f = frame(24000 * 100);
        assert_eq!(items(&layout(&Config::default(), &f, &width).unwrap())[0], "minecraft:golden_apple");
        f.day_time = 24000 * 600;
        assert_eq!(items(&layout(&Config::default(), &f, &width).unwrap())[0], "minecraft:totem_of_undying");
        f.mode = Mode::Creative;
        assert_eq!(items(&layout(&Config::default(), &f, &width).unwrap())[0], "minecraft:grass_block");
        f.day_time = 0;
        f.mode = Mode::Survival;
        f.hardcore = true;
        let hud = layout(&Config::default(), &f, &width).unwrap();
        assert!(hud.draws.iter().any(|d| matches!(d, Draw::Sprite { name: HARDCORE_HEART_FULL, .. })));
    }

    #[test]
    fn bed_when_sleepy() {
        let c = Config { show_sleep_icon: true, ..Config::default() };
        assert_eq!(items(&layout(&c, &frame(13000), &width).unwrap()), ["minecraft:red_bed"]);
        assert_eq!(items(&layout(&c, &frame(1000), &width).unwrap()), ["minecraft:clock"]);
    }

    #[test]
    fn in_nether_follows_setting() {
        let mut c = Config::default();
        let mut f = frame(1000);
        f.overworld = false;
        c.other_dimensions = OtherDimensions::Hide;
        assert_eq!(texts(&layout(&c, &f, &width).unwrap()), ["Day: 0"]);
        c.other_dimensions = OtherDimensions::Unknown;
        assert_eq!(texts(&layout(&c, &f, &width).unwrap()), ["Day: 0", "??:??"]);
    }

    #[test]
    fn bottom_position_clears_status_bars() {
        let c = Config { position_bottom: true, ..Config::default() };
        let mut f = frame(0);
        let y = |f: &Frame| match layout(&c, f, &width).unwrap().draws.iter().find(|d| matches!(d, Draw::Text { .. })) {
            Some(Draw::Text { y, .. }) => *y,
            _ => panic!("no text"),
        };
        assert_eq!(y(&f), 240 - 55 - 12);
        f.status_bar_height = 59;
        assert_eq!(y(&f), 240 - 75 - 12);
        f.mode = Mode::Creative;
        assert_eq!(y(&f), 240 - 45 - 12);
    }

    #[test]
    fn boss_bars_move_only_when_hud_is_on_top() {
        let mut c = Config::default();
        assert_eq!(boss_bar_shift(&c), 35);
        c.scale = 150;
        c.offset_y = -10;
        assert_eq!(boss_bar_shift(&c), 43);
        c.position_bottom = true;
        assert_eq!(boss_bar_shift(&c), 0);
    }

    #[test]
    fn celebration_sound_only_when_the_day_advances() {
        let mut s = State::new(Config::default());
        // Entrando in un mondo già al traguardo non suona
        assert!(!s.celebration_sound(100, 10_000, true));
        assert!(!s.celebration_sound(100, 10_016, true));
        assert!(s.celebration_sound(101, 10_032, true));
        assert!(!s.celebration_sound(102, 10_048, false));
    }

    #[test]
    fn simple_mode_shows_only_the_day_where_chosen() {
        let mut c = Config {
            simple_mode: true,
            show_day_counter: false,
            show_clock: false,
            show_bossbar: false,
            ..Config::default()
        };
        // In alto a sinistra: solo il testo, nessuna icona né barra
        let hud = layout(&c, &frame(24000 * 7 + 1000), &width).unwrap();
        assert_eq!(texts(&hud), ["Day: 7"]);
        assert_eq!(hud.draws.len(), 1);
        assert_eq!(hud.draws[0], Draw::Text { text: "Day: 7".into(), x: 3, y: 3, argb: 0xFFFFFFFF });
        assert_eq!((hud.pivot, hud.offset), ([0.0, 0.0], [0.0, 0.0]));
        // In basso a destra il riquadro (36 + 6) x 15 tocca i bordi, anche ingrandito
        c.simple_x = 100;
        c.simple_y = 100;
        let hud = layout(&c, &frame(24000 * 7), &width).unwrap();
        assert_eq!(hud.pivot, [358.0, 225.0]);
        c.scale = 150;
        let hud = layout(&c, &frame(24000 * 7), &width).unwrap();
        assert_eq!(hud.pivot, [337.0, 218.0]);
        // Sfondo dietro al riquadro, arcobaleno nei traguardi, bossbar ferme
        c.show_background = true;
        let hud = layout(&c, &frame(24000 * 100), &width).unwrap();
        assert!(matches!(hud.draws[0], Draw::Fill { .. }));
        assert!(hud.milestone);
        assert_eq!(boss_bar_shift(&c), 0);
    }

    #[test]
    fn dragging_places_the_simple_counter() {
        let mut s = State::new(Config { simple_mode: true, ..Config::default() });
        // "Sposta col mouse", accanto all'interruttore, apre lo spostamento
        let widgets = screen_layout(Section::Elements, &s.config, 427, 240);
        let place = widget(&widgets, Target::Opt(Opt::Place));
        assert_eq!(Opt::Place.label(&s.config, Lang::It, &vanilla(), None), "Sposta col mouse");
        assert!(s.screen_click(&widgets, ((place.x + 5) as f32, (place.y + 5) as f32), false).click_sound);
        assert!(s.screen.placing);

        // Riquadro 42x15 in alto a sinistra: fuori non si afferra
        let b = simple_box(&s.config, 400, 240, 36);
        assert!(!s.place_press(&b, (100.0, 100.0)));
        assert!(s.place_press(&b, (10.0, 5.0)));
        // Afferrato a (10, 5): l'angolo va a (189, 117.5) -> metà dello spazio libero
        s.place_drag(&b, (189.0 + 10.0, 112.5 + 5.0), 400, 240);
        assert_eq!((s.config.simple_x, s.config.simple_y), (53, 50));
        // Oltre il bordo resta dentro lo schermo
        s.place_drag(&b, (900.0, -50.0), 400, 240);
        assert_eq!((s.config.simple_x, s.config.simple_y), (100, 0));
        assert!(s.place_release() && !s.place_release());
        s.place_nudge(1, -1);
        assert_eq!((s.config.simple_x, s.config.simple_y), (100, 0));
        s.place_nudge(-3, 2);
        assert_eq!((s.config.simple_x, s.config.simple_y), (97, 2));
        s.place_close();
        assert!(!s.screen.placing);
    }

    #[test]
    fn nothing_when_everything_is_off() {
        let c = Config {
            show_day_counter: false,
            show_clock: false,
            show_bossbar: false,
            ..Config::default()
        };
        assert!(layout(&c, &frame(0), &width).is_none());
    }
}
