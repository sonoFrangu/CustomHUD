// Beautiful Day Counter (Custom HUD) per Horizon: l'HUD.
//
// Questo file è il modulo `crate::game::customhud` iniettato nel gioco, e con
// `customhud_text`, `customhud_config` e `customhud_config_screen` anche la
// libreria Cargo autonoma (testabile senza Horizon, vedi `src/lib.rs`). È la
// traduzione di `DayHud` della versione Java, senza nulla che dipenda dal
// gioco: calcola cosa disegnare (rettangoli, testi, sprite vanilla, icone) con le
// stesse coordinate intere. `screen.rs` legge il gioco, disegna e gestisce l'input.

use std::sync::Mutex;

// Tutto quello che serve a `screen.rs` passa da qui, come quando era un file solo.
pub use super::customhud_config::*;
pub use super::customhud_config_screen::*;
pub use super::customhud_text::*;

pub const MOD_ID: &str = "customhud";
pub const TITLE: &str = "Beautiful Day Counter 1.8.0";
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

// Icone del meteo 8x8, riga per riga: c nuvola, k nuvola da temporale, d goccia,
// s neve, b fulmine. DRY: piove nel mondo ma non qui (deserto, savana...).
const RAIN: [&str; 8] = [
    "...cc...", ".ccccc..", ".cccccc.", "cccccccc", ".cccccc.", "..d..d..", ".d..d..d", "d..d..d.",
];
const SNOW: [&str; 8] = [
    "...cc...", ".ccccc..", ".cccccc.", "cccccccc", ".cccccc.", ".s...s..", "...s...s", ".s...s..",
];
const THUNDER: [&str; 8] = [
    "...kk...", ".kkkkk..", ".kkkkkk.", "kkkkkkkk", ".kkbkkk.", "...bb...", "....b...", "...b....",
];
const DRY: [&str; 8] = [
    "...cc...", ".ccccc..", ".cccccc.", "cccccccc", ".cccccc.", "........", "........", "........",
];
const WEATHER_SIZE: i32 = 8;

// Da quanti tick senza dormire possono arrivare i phantom (PhantomSpawner: 3 giorni).
pub const PHANTOM_TICKS: i64 = 72000;

// Il tempo dove si trova il giocatore.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Weather {
    Clear,
    Rain,
    Snow,
    Thunder,
    // Piove nel mondo, ma il bioma qui non ha precipitazioni.
    Dry,
}

impl Weather {
    // `Level.isRaining` (pioggia sopra 0.2), `isThundering` (temporale sopra 0.9) e
    // la precipitazione del bioma: `Some(true)` neve, `Some(false)` pioggia, `None` niente.
    pub fn from_levels(rain: f32, thunder: f32, snow: Option<bool>) -> Self {
        if rain <= 0.2 {
            Weather::Clear
        } else if thunder > 0.9 {
            Weather::Thunder
        } else {
            match snow {
                Some(true) => Weather::Snow,
                Some(false) => Weather::Rain,
                None => Weather::Dry,
            }
        }
    }

    fn icon(self) -> Option<&'static [&'static str; 8]> {
        match self {
            Weather::Clear => None,
            Weather::Rain => Some(&RAIN),
            Weather::Snow => Some(&SNOW),
            Weather::Thunder => Some(&THUNDER),
            Weather::Dry => Some(&DRY),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Survival,
    Creative,
    Adventure,
    Spectator,
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
    pub weather: Weather,
    // Tick dall'ultima dormita (statistica `time_since_rest`).
    pub since_rest: i64,
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
    let weather = if c.show_weather { f.weather.icon() } else { None };
    let show_weather = weather.is_some();
    // I phantom inseguono solo chi è in sopravvivenza o avventura
    let can_attract = matches!(f.mode, Mode::Survival | Mode::Adventure);
    let show_phantom = c.phantom_warning && can_attract && f.since_rest >= PHANTOM_TICKS;
    let phantom = (f.since_rest / 24000).to_string();

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
    let phantom_w = text_width(&phantom);

    let milestone = c.milestones && c.is_milestone(days);

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
    if show_weather {
        total += if total > 0 { 6 } else { 0 } + WEATHER_SIZE;
    }
    if show_countdown {
        total += if total > 0 { 6 } else { 0 } + countdown_w;
    }
    if show_moon {
        total += if total > 0 { 6 } else { 0 } + MOON_SIZE;
    }
    if show_phantom {
        total += if total > 0 { 6 } else { 0 } + 13 + phantom_w;
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

    if let Some(icon) = weather {
        if show_days || show_clock {
            x += 6;
        }
        pixels(&mut draws, icon, x, text_y, alpha);
        x += WEATHER_SIZE;
    }

    if show_countdown {
        if show_days || show_clock || show_weather {
            x += 6;
        }
        draws.push(Draw::Text { text: countdown, x, y: text_y, argb: alpha << 24 | 0xBBBBBB });
        x += countdown_w;
    }

    if show_moon {
        if show_days || show_clock || show_weather || show_countdown {
            x += 6;
        }
        moon(&mut draws, x, text_y, days.rem_euclid(8) as i32, alpha);
        x += MOON_SIZE;
    }

    if show_phantom {
        if show_days || show_clock || show_weather || show_countdown || show_moon {
            x += 6;
        }
        draws.push(Draw::Item { id: "minecraft:phantom_membrane", x, y: text_y - 1 });
        x += 13;
        draws.push(Draw::Text { text: phantom, x, y: text_y, argb: alpha << 24 | 0xFF5555 });
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


// Modalità semplice: solo "Giorno: N" (arcobaleno nei traguardi), nel punto scelto.
// La percentuale è dello spazio libero, così a 0% e 100% il riquadro tocca il
// bordo e resta tutto sullo schermo a ogni dimensione.
fn simple_layout(c: &Config, f: &Frame, text_width: &dyn Fn(&str) -> i32) -> Hud {
    let days = f.day_time.div_euclid(24000);
    let day = day_text(days, f.lang);
    let milestone = c.milestones && c.is_milestone(days);
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

// Un'icona a pixel (una stringa per riga), unendo i pixel uguali vicini.
fn pixels(draws: &mut Vec<Draw>, rows: &[&str], x: i32, y: i32, alpha: u32) {
    for (r, line) in rows.iter().enumerate() {
        let line = line.as_bytes();
        let mut start = 0;
        while start < line.len() {
            let c = line[start];
            let mut end = start + 1;
            while end < line.len() && line[end] == c {
                end += 1;
            }
            if let Some(rgb) = pixel_color(c) {
                let top = y + r as i32;
                draws.push(Draw::Fill {
                    x0: x + start as i32,
                    y0: top,
                    x1: x + end as i32,
                    y1: top + 1,
                    argb: alpha << 24 | rgb,
                });
            }
            start = end;
        }
    }
}

fn pixel_color(c: u8) -> Option<u32> {
    match c {
        b'c' => Some(0xE6E9EF),
        b'k' => Some(0x7C8091),
        b'd' => Some(0x5DA9E9),
        b's' => Some(0xFFFFFF),
        b'b' => Some(0xFFD83D),
        _ => None,
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
            weather: Weather::Clear,
            since_rest: 0,
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
            simple_x: 0,
            simple_y: 0,
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
        let mut s = State::new(Config { simple_mode: true, simple_x: 0, simple_y: 0, ..Config::default() });
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
    fn weather_icon_follows_the_sky_and_the_biome() {
        assert_eq!(Weather::from_levels(0.1, 1.0, Some(false)), Weather::Clear);
        assert_eq!(Weather::from_levels(1.0, 0.0, Some(false)), Weather::Rain);
        assert_eq!(Weather::from_levels(1.0, 0.0, Some(true)), Weather::Snow);
        assert_eq!(Weather::from_levels(1.0, 0.0, None), Weather::Dry);
        assert_eq!(Weather::from_levels(1.0, 1.0, Some(true)), Weather::Thunder);

        let c = Config { show_weather: true, ..Config::default() };
        let mut f = frame(24000 * 5 + 6000);
        let fills = |hud: &Hud| hud.draws.iter().filter(|d| matches!(d, Draw::Fill { .. })).count();
        assert_eq!(fills(&layout(&c, &f, &width).unwrap()), 0);
        f.weather = Weather::Rain;
        let hud = layout(&c, &f, &width).unwrap();
        // total = 107 + 6 + 8 = 121; x = (400 - 121) / 2 = 139; dopo l'ora: 139 + 107 + 6
        let first = hud.draws.iter().find(|d| matches!(d, Draw::Fill { .. }));
        assert_eq!(first, Some(&Draw::Fill { x0: 252 + 3, y0: 13, x1: 252 + 5, y1: 14, argb: 0xFFE6E9EF }));
        // Pioggia: 5 righe di nuvola (1 tratto ciascuna) + 3 di gocce (2 + 3 + 3)
        assert_eq!(fills(&hud), 5 + 8);
        // Spento, nessuna icona anche sotto il temporale
        f.weather = Weather::Thunder;
        assert_eq!(fills(&layout(&Config::default(), &f, &width).unwrap()), 0);
    }

    #[test]
    fn phantom_warning_after_three_days_without_sleep() {
        let c = Config::default();
        let mut f = frame(24000 * 5 + 6000);
        f.since_rest = PHANTOM_TICKS - 1;
        assert_eq!(items(&layout(&c, &f, &width).unwrap()), ["minecraft:clock"]);
        f.since_rest = 24000 * 4 + 100;
        let hud = layout(&c, &f, &width).unwrap();
        assert_eq!(items(&hud), ["minecraft:clock", "minecraft:phantom_membrane"]);
        assert_eq!(texts(&hud), ["Day: 5", "12:00", "4"]);
        // total = 107 + 6 + 13 + 6 = 132; x = 134; membrana a 134 + 107 + 6
        assert!(hud.draws.contains(&Draw::Item { id: "minecraft:phantom_membrane", x: 247, y: 12 }));
        assert!(hud.draws.contains(&Draw::Text { text: "4".into(), x: 260, y: 13, argb: 0xFFFF5555 }));
        // Né in creativa né se spento
        f.mode = Mode::Creative;
        assert!(!items(&layout(&c, &f, &width).unwrap()).contains(&"minecraft:phantom_membrane"));
        f.mode = Mode::Adventure;
        let off = Config { phantom_warning: false, ..c };
        assert!(!items(&layout(&off, &f, &width).unwrap()).contains(&"minecraft:phantom_membrane"));
    }

    #[test]
    fn custom_milestones_change_the_rainbow_days() {
        let c = Config { milestone_every: 10, ..Config::default() };
        assert!(layout(&c, &frame(24000 * 30), &width).unwrap().milestone);
        assert!(!layout(&c, &frame(24000 * 365), &width).unwrap().milestone);
        assert!(layout(&Config::default(), &frame(24000 * 365), &width).unwrap().milestone);
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
