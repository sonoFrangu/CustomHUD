// La configurazione (`CustomHudConfig` della versione Java): valori, limiti,
// lettura e scrittura. Diventa `crate::game::customhud_config`.

pub const MIN_SCALE: i32 = 75;
pub const MAX_SCALE: i32 = 150;
pub const MIN_OPACITY: i32 = 20;
pub const MAX_OPACITY: i32 = 100;
pub const MAX_OFFSET: i32 = 100;
// Le scelte di "Traguardo ogni", nell'ordine del pulsante; 0 = i classici.
pub const MILESTONE_EVERY: [i32; 9] = [0, 10, 25, 50, 100, 250, 365, 500, 1000];

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
    pub show_weather: bool,
    pub phantom_warning: bool,
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
    // Ogni quanti giorni c'è un traguardo; 0 = i classici (50, 365 e ogni 100).
    pub milestone_every: i32,
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
            show_weather: false,
            phantom_warning: true,
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
            milestone_every: 0,
            simple_mode: false,
            simple_x: 50,
            simple_y: 77,
        }
    }
}

impl Config {
    pub fn is_hud_visible(&self) -> bool {
        self.simple_mode || self.show_day_counter || self.show_clock || self.show_bossbar
    }

    // Il giorno è un traguardo: i classici (50, 365 e ogni 100) o uno ogni
    // `milestone_every` giorni.
    pub fn is_milestone(&self, days: i64) -> bool {
        if days <= 0 {
            return false;
        }
        if self.milestone_every > 0 {
            return days % self.milestone_every as i64 == 0;
        }
        days == 50 || days == 365 || days % 100 == 0
    }

    // Riporta nei limiti i valori modificati a mano nel file.
    fn sanitize(&mut self) {
        self.scale = self.scale.clamp(MIN_SCALE, MAX_SCALE);
        self.opacity = self.opacity.clamp(MIN_OPACITY, MAX_OPACITY);
        self.offset_x = self.offset_x.clamp(-MAX_OFFSET, MAX_OFFSET);
        self.offset_y = self.offset_y.clamp(-MAX_OFFSET, MAX_OFFSET);
        self.simple_x = self.simple_x.clamp(0, 100);
        self.simple_y = self.simple_y.clamp(0, 100);
        if !MILESTONE_EVERY.contains(&self.milestone_every) {
            self.milestone_every = 0;
        }
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
                "showWeather" => c.show_weather = flag,
                "phantomWarning" => c.phantom_warning = flag,
                "milestoneEvery" => c.milestone_every = number.unwrap_or(c.milestone_every),
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
            format!("showWeather={}", self.show_weather),
            format!("phantomWarning={}", self.phantom_warning),
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
            format!("milestoneEvery={}", self.milestone_every),
            format!("simpleMode={}", self.simple_mode),
            format!("simpleX={}", self.simple_x),
            format!("simpleY={}", self.simple_y),
        ];
        lines.join("\n") + "\n"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn milestones() {
        let mut c = Config::default();
        assert!(c.is_milestone(50) && c.is_milestone(365) && c.is_milestone(200));
        assert!(!c.is_milestone(0) && !c.is_milestone(150));
        c.milestone_every = 25;
        assert!(c.is_milestone(25) && c.is_milestone(150));
        assert!(!c.is_milestone(0) && !c.is_milestone(365) && !c.is_milestone(110));
    }

    #[test]
    fn simple_counter_starts_above_the_hotbar_like_java() {
        let c = Config::default();
        assert_eq!((c.simple_x, c.simple_y), (50, 77));
        // Una config senza le chiavi nuove le prende predefinite
        let old = Config::parse("showClock=false");
        assert!(old.phantom_warning && !old.show_weather && old.milestone_every == 0);
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
            show_weather: true,
            phantom_warning: false,
            milestone_every: 365,
            ..Config::default()
        };
        assert_eq!(Config::parse(&c.serialize()), c);
        let odd = Config::parse("opacity=5\nnonsense\nfoo=bar\noffsetX=999\nscale=10\nsimpleY=150\nmilestoneEvery=7");
        assert_eq!((odd.opacity, odd.offset_x, odd.scale, odd.simple_y), (20, 100, 75, 100));
        assert_eq!(odd.milestone_every, 0);
    }
}
