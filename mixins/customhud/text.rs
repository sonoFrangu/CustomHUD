// Lingue e testi di Beautiful Day Counter: i file `assets/customhud/lang/*.json`
// della versione Java. Diventa `crate::game::customhud_text`.

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
    ("customhud.config.weather", ["Weather", "Meteo", "Wetter", "Tiempo", "Météo", "Clima"]),
    ("customhud.config.weather.tooltip", [
        "A small cloud after the clock while it rains: drops, snowflakes or a lightning bolt, depending on the weather where you are.",
        "Una nuvoletta dopo l'orologio quando piove: gocce, fiocchi di neve o un fulmine, secondo il tempo dove sei.",
        "Eine kleine Wolke nach der Uhr, wenn es regnet: Tropfen, Schneeflocken oder ein Blitz, je nach Wetter an deinem Ort.",
        "Una nubecita después del reloj cuando llueve: gotas, copos de nieve o un rayo, según el tiempo donde estás.",
        "Un petit nuage après l'horloge quand il pleut : gouttes, flocons ou éclair, selon le temps là où tu es.",
        "Uma nuvenzinha depois do relógio quando chove: gotas, flocos de neve ou um raio, conforme o tempo onde você está.",
    ]),
    ("customhud.config.phantom", ["Phantom warning", "Avviso phantom", "Phantom-Warnung", "Aviso de phantoms", "Alerte phantoms", "Aviso de phantoms"]),
    ("customhud.config.phantom.tooltip", [
        "After 3 days without sleep, shows a phantom membrane with the days since you last slept: phantoms can come at night.",
        "Dopo 3 giorni senza dormire mostra una membrana di phantom con i giorni dall'ultima dormita: di notte possono arrivare i phantom.",
        "Nach 3 Tagen ohne Schlaf erscheint eine Phantomhaut mit den Tagen seit dem letzten Schlaf: Nachts können Phantome kommen.",
        "Tras 3 días sin dormir muestra una membrana de phantom con los días desde que dormiste: de noche pueden venir phantoms.",
        "Après 3 jours sans dormir, affiche une membrane de phantom avec les jours depuis ton dernier sommeil : la nuit, des phantoms peuvent venir.",
        "Depois de 3 dias sem dormir, mostra uma membrana de phantom com os dias desde que você dormiu: à noite podem vir phantoms.",
    ]),
    ("customhud.config.scale",["Size", "Dimensione", "Größe", "Tamaño", "Taille", "Tamanho"]),
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
        "Rainbow day counter and special icon on milestone days.",
        "Contatore arcobaleno e icona speciale nei giorni traguardo.",
        "Regenbogen-Zähler und besonderes Symbol an Meilenstein-Tagen.",
        "Contador arcoíris e icono especial en los días hito.",
        "Compteur arc-en-ciel et icône spéciale les jours d'étape.",
        "Contador arco-íris e ícone especial nos dias marco.",
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
    ("customhud.config.milestone_every", ["Milestone every", "Traguardo ogni", "Meilenstein alle", "Hito cada", "Étape tous les", "Marco a cada"]),
    ("customhud.config.milestone_every.tooltip", [
        "Classic: days 50, 365 and every 100. Or pick a milestone every 10, 25, 50... days.",
        "Classico: giorni 50, 365 e ogni 100. Oppure scegli un traguardo ogni 10, 25, 50... giorni.",
        "Klassisch: Tag 50, 365 und alle 100. Oder wähle einen Meilenstein alle 10, 25, 50... Tage.",
        "Clásico: días 50, 365 y cada 100. O elige un hito cada 10, 25, 50... días.",
        "Classique : jours 50, 365 et tous les 100. Ou choisis une étape tous les 10, 25, 50... jours.",
        "Clássico: dias 50, 365 e a cada 100. Ou escolha um marco a cada 10, 25, 50... dias.",
    ]),
    ("customhud.config.milestone_every.classic", ["Classic", "Classico", "Klassisch", "Clásico", "Classique", "Clássico"]),
    ("customhud.config.milestone_every.days", ["%s days", "%s giorni", "%s Tage", "%s días", "%s jours", "%s dias"]),
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
