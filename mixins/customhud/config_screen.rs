// La logica della schermata delle impostazioni (`ConfigScreen` della versione
// Java): disposizione dei widget, etichette, clic, trascinamento e rotellina.
// Diventa `crate::game::customhud_config_screen`; `screen.rs` la disegna.

use super::customhud::State;
use super::customhud_config::*;
use super::customhud_text::{Lang, Vanilla, option_name_value, tr, tr_arg};

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
                Section::Behavior => &[Opt::Milestones, Opt::MilestoneSound, Opt::MilestoneEvery],
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
                Opt::Weather,
                Opt::Phantom,
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
                Opt::MilestoneEvery,
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
    Weather,
    Phantom,
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
    MilestoneEvery,
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
            Opt::Weather => "weather",
            Opt::Phantom => "phantom",
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
            Opt::MilestoneEvery => "milestone_every",
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
            Opt::Weather => &mut c.show_weather,
            Opt::Phantom => &mut c.phantom_warning,
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
            Opt::MilestoneEvery => {
                c.milestone_every = step(&MILESTONE_EVERY, c.milestone_every, delta);
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
                Opt::MilestoneEvery => match c.milestone_every {
                    0 => tr("customhud.config.milestone_every.classic", lang).to_owned(),
                    n => tr_arg("customhud.config.milestone_every.days", lang, &n.to_string()),
                },
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

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::customhud_text::*;

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

    fn widget(widgets: &[Widget], target: Target) -> Widget {
        *widgets.iter().find(|w| w.target == target).expect("widget")
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
        // Quarta riga: conto alla rovescia e meteo; quinta: phantom e modalità semplice
        let countdown = widget(&widgets, Target::Opt(Opt::Countdown));
        let weather = widget(&widgets, Target::Opt(Opt::Weather));
        assert_eq!((countdown.x, countdown.y, weather.x, weather.y), (58, 160, 218, 160));
        let phantom = widget(&widgets, Target::Opt(Opt::Phantom));
        let simple = widget(&widgets, Target::Opt(Opt::Simple));
        assert_eq!((phantom.x, phantom.y, simple.x, simple.y), (58, 184, 218, 184));
        // Comportamento: "Traguardo ogni" accanto al suono, righe piene
        let widgets = screen_layout(Section::Behavior, &Config::default(), 427, 240);
        let sound = widget(&widgets, Target::Opt(Opt::MilestoneSound));
        let every = widget(&widgets, Target::Opt(Opt::MilestoneEvery));
        assert_eq!((sound.x, sound.y, every.x, every.y), (58, 160, 218, 160));
        // In modalità semplice è l'ultima, da sola e centrata
        let c = Config { simple_mode: true, ..Config::default() };
        let widgets = screen_layout(Section::Behavior, &c, 427, 240);
        let every = widget(&widgets, Target::Opt(Opt::MilestoneEvery));
        assert_eq!((every.x, every.y), (138, 112));
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
        // 200 - 33 - 116 = 51 < 88
        assert_eq!(widget(&widgets, Target::Opt(Opt::Days)).y, 51);
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
        assert_eq!(Opt::Phantom.label(&c, Lang::It, &v, None), "Avviso phantom: ON");
        assert_eq!(Opt::Weather.label(&c, Lang::En, &v, None), "Weather: OFF");
        assert_eq!(Opt::MilestoneEvery.label(&c, Lang::En, &v, None), "Milestone every: Classic");
        let c = Config { milestone_every: 25, ..c };
        assert_eq!(Opt::MilestoneEvery.label(&c, Lang::It, &v, None), "Traguardo ogni: 25 giorni");
    }

    #[test]
    fn milestone_every_cycles_through_its_choices() {
        let mut c = Config::default();
        Opt::MilestoneEvery.cycle(&mut c, 1);
        assert_eq!(c.milestone_every, 10);
        Opt::MilestoneEvery.cycle(&mut c, -1);
        Opt::MilestoneEvery.cycle(&mut c, -1);
        assert_eq!(c.milestone_every, 1000);
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

}
