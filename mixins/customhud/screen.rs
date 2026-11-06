// Il lato "gioco" di Beautiful Day Counter, iniettato come `crate::game::customhud_screen`:
// legge `Game`, disegna l'HUD e la schermata delle impostazioni (la `ConfigScreen`
// Java) con i widget vanilla di Horizon e ne gestisce l'input. Tutta la logica sta
// in `customhud` (runtime.rs), testabile senza il gioco.
//
// La schermata usa lo stato `Screen::DialogWarning` del gioco, che mette in pausa,
// libera il mouse e sfoca il mondo come ogni schermata vanilla: `draw_screen` ne
// prende il posto quando è aperta, `handle_event` ne riceve l'input.

use super::customhud as hud;
use super::{Game, Screen};
use horizon_client::command::args::GameMode;
use horizon_client::hud::{HudBatch, rgba};
use horizon_client::text::Text;
use horizon_client::ui;
use std::ops::ControlFlow;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

fn load(game: &Game<'_>) -> Option<String> {
    game.native_setting(hud::MOD_ID, hud::SETTING_KEY)
}

fn lang(game: &Game<'_>) -> hud::Lang {
    hud::Lang::from_code(&game.settings.language)
}

fn save(game: &Game<'_>, config: &hud::Config) {
    game.native_set_setting(hud::MOD_ID, hud::SETTING_KEY, &config.serialize());
}

// ARGB (come in Java) -> il colore dei vertici di Horizon.
fn color(argb: u32) -> u32 {
    let [a, r, g, b] = argb.to_be_bytes();
    rgba(r, g, b, a)
}

fn millis() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_millis() as u64)
}

fn text_width(game: &Game<'_>, text: &str) -> i32 {
    game.native_text_width(text).round() as i32
}

// ---- HUD (`DayHud.extract`) ----

pub(super) fn draw_hud(game: &mut Game<'_>, batch: &mut HudBatch) {
    if !game.native_on(hud::MOD_ID) || !game.native_window() {
        return;
    }
    // Nella schermata delle impostazioni l'HUD finirebbe sopra al titolo
    if screen_open(game) {
        return;
    }
    let Some(player) = game.native_player() else {
        return;
    };
    // Sotto la schermata F3 l'HUD finirebbe sopra al testo
    if !game.native_corner_shown(horizon_client::extras::Corner::TopLeft) {
        return;
    }
    let overworld = player.dimension == "minecraft:overworld";
    let config = hud::with_state(|| load(game), |s| s.config.clone());
    let frame = frame(game, batch, overworld);
    let Some(out) = hud::layout(&config, &frame, &|t: &str| text_width(game, t)) else {
        return;
    };
    let play_sound = out.milestone && config.milestone_sound;
    if hud::with_state(|| None, |s| s.celebration_sound(out.days, frame.millis, play_sound)) {
        game.audio.play(hud::LEVEL_UP_SOUND, None, 0.6, 1.0);
    }
    draw_out(game, batch, &out);
}

fn frame(game: &Game<'_>, batch: &HudBatch, overworld: bool) -> hud::Frame {
    hud::Frame {
        day_time: game.world.day_time,
        width: batch.width as i32,
        height: batch.height as i32,
        overworld,
        thundering: game.world.weather.levels(1.0).1 > 0.9,
        mode: match game.mode {
            GameMode::Survival => hud::Mode::Survival,
            GameMode::Creative => hud::Mode::Creative,
            GameMode::Adventure => hud::Mode::Adventure,
            GameMode::Spectator => hud::Mode::Spectator,
        },
        hardcore: game.level.hardcore,
        low_air: game.player.as_ref().is_some_and(|p| p.air < 300),
        status_bar_height: game.health_hud.left_height.max(game.health_hud.right_height),
        millis: millis(),
        lang: lang(game),
    }
}

// Disegna nel batch il risultato di `hud::layout`.
fn draw_out(game: &mut Game<'_>, batch: &mut HudBatch, out: &hud::Hud) {
    let start = batch.vertices.len();
    for draw in &out.draws {
        match *draw {
            hud::Draw::Fill { x0, y0, x1, y1, argb } => {
                let (w, h) = ((x1 - x0) as f32, (y1 - y0) as f32);
                batch.fill(&game.gui.atlas, x0 as f32, y0 as f32, w, h, color(argb));
            }
            hud::Draw::Text { ref text, x, y, argb } => {
                let text = Text::literal(text);
                batch.text_styled(&game.gui.atlas, &text, x as f32, y as f32, color(argb), true);
            }
            hud::Draw::Sprite { name, x, y, w, h, u, argb } => {
                if let Some(mut r) = game.gui.atlas.sprites.get(name).copied() {
                    r.u1 = r.u0 + (r.u1 - r.u0) * u;
                    batch.rect_uv(x as f32, y as f32, w as f32, h as f32, r, color(argb));
                }
            }
            hud::Draw::Item { id, x, y } => {
                let item_start = batch.vertices.len();
                let stack = horizon_client::items::ItemStack::new(id, 1);
                let (x, y) = (x as f32, y as f32);
                game.draw_item_model(batch, &stack, x, y);
                batch.scale_since(item_start, x, y, hud::ITEM_SCALE, hud::ITEM_SCALE);
            }
        }
    }
    // Il `pose` Java: scala attorno al bordo più vicino, poi sposta
    let [px, py] = out.pivot;
    batch.scale_since(start, px, py, out.scale, out.scale);
    let [dx, dy] = out.offset;
    for v in &mut batch.vertices[start..] {
        v.pos[0] += dx;
        v.pos[1] += dy;
    }
}

// Dopo le bossbar e prima di minimappa e HUD delle mod: le sposta in basso
// quando l'HUD è in alto, come `pushBossBarShift` in Java.
pub(super) fn shift_boss_bars(game: &Game<'_>, batch: &mut HudBatch) {
    let Some(start) = hud::take_boss_bars_mark() else {
        return;
    };
    if !game.native_on(hud::MOD_ID) || start > batch.vertices.len() {
        return;
    }
    let shift = hud::with_state(|| load(game), |s| hud::boss_bar_shift(&s.config));
    for v in &mut batch.vertices[start..] {
        v.pos[1] += shift as f32;
    }
}

// ---- Schermata delle impostazioni (`ConfigScreen`) ----

fn screen_open(game: &Game<'_>) -> bool {
    let open = hud::with_state(|| load(game), |s| s.screen.open);
    if open && game.screen != Screen::DialogWarning {
        // Il gioco ha cambiato schermata (morte, disconnessione...): è chiusa
        hud::with_state(|| None, |s| {
            s.screen.open = false;
            s.screen.drag = None;
            s.screen.placing = false;
            s.screen.grab = None;
        });
        return false;
    }
    open
}

// `Minecraft.setScreen(new ConfigScreen(parent))`.
pub(super) fn open_screen(game: &mut Game<'_>, parent: hud::Parent) {
    hud::with_state(|| load(game), |s| {
        s.screen.open = true;
        s.screen.parent = parent;
        s.screen.drag = None;
        s.screen.placing = false;
        s.screen.grab = None;
    });
    // I tasti di gioco non devono restare premuti dietro la schermata
    game.keys.clear();
    game.screen = Screen::DialogWarning;
    game.capture(false);
}

// `ConfigScreen.onClose`: salva e torna alla schermata di prima.
fn close_screen(game: &mut Game<'_>) {
    let (config, parent) = hud::with_state(|| None, |s| {
        s.screen.open = false;
        s.screen.drag = None;
        s.screen.placing = false;
        s.screen.grab = None;
        (s.config.clone(), s.screen.parent)
    });
    save(game, &config);
    match parent {
        hud::Parent::Options => game.screen = Screen::Options,
        hud::Parent::Game => {
            game.screen = Screen::Playing;
            game.capture(true);
        }
    }
}

fn vanilla(game: &Game<'_>) -> hud::Vanilla {
    hud::Vanilla {
        on: game.translate("options.on", "ON"),
        off: game.translate("options.off", "OFF"),
        done: game.translate("gui.done", "Done"),
        generic_value: game.translate("options.generic_value", "%s: %s"),
    }
}

// Al posto di `draw_dialog_warning` quando la schermata è aperta.
pub(super) fn draw_screen(game: &mut Game<'_>, batch: &mut HudBatch) -> ControlFlow<()> {
    if !screen_open(game) {
        return ControlFlow::Continue(());
    }
    if hud::with_state(|| None, |s| s.screen.placing) {
        draw_placing(game, batch);
        return ControlFlow::Break(());
    }
    let (w, h) = (batch.width as i32, batch.height as i32);
    let cursor = game.cursor;
    let lang = lang(game);
    let vanilla = vanilla(game);
    let (widgets, config, section, drag) = hud::with_state(|| None, |s| {
        let widgets = hud::screen_layout(s.screen.section, &s.config, w, h);
        s.screen_drag(&widgets, cursor);
        (widgets, s.config.clone(), s.screen.section, s.screen.drag)
    });

    let atlas = &game.gui.atlas;
    // Screen.extractBackground: il mondo è già sfocato, sopra il fondo dei menu
    ui::menu_background(batch, atlas, 0.0, 0.0, w as f32, h as f32, true);

    let title = hud::tr("customhud.config.title", lang);
    let (tx, ty) = hud::title_position(w, text_width(game, title));
    batch.text_styled(atlas, &Text::literal(title), tx as f32, ty as f32, u32::MAX, true);

    let mut hovered_opt = None;
    for widget in &widgets {
        let hover = widget.contains(cursor);
        let (x, y, ww, wh) = (widget.x as f32, widget.y as f32, widget.w as f32, widget.h as f32);
        match widget.target {
            hud::Target::Section(s) => {
                let label = Text::literal(hud::tr(s.label_key(), lang));
                ui::button(batch, atlas, x, y, ww, wh, &label, s != section, hover);
            }
            hud::Target::Opt(opt) => {
                if hover {
                    hovered_opt = Some(opt);
                }
                let dragged = drag.filter(|(d, _)| *d == opt).map(|(_, f)| f);
                let label = Text::literal(&opt.label(&config, lang, &vanilla, dragged));
                if opt.slider().is_some() {
                    let fraction = dragged.unwrap_or_else(|| opt.slider_fraction(&config));
                    let active = hover || dragged.is_some();
                    let dragging = dragged.is_some();
                    ui::slider(
                        batch,
                        atlas,
                        x,
                        y,
                        ww,
                        &label,
                        fraction as f32,
                        active,
                        true,
                        dragging,
                    );
                } else {
                    ui::button(batch, atlas, x, y, ww, wh, &label, true, hover);
                }
            }
            hud::Target::Reset => {
                let label = Text::literal(hud::tr("customhud.config.reset", lang));
                ui::button(batch, atlas, x, y, ww, wh, &label, true, hover);
            }
            hud::Target::Done => {
                let label = Text::literal(&vanilla.done);
                ui::button(batch, atlas, x, y, ww, wh, &label, true, hover);
            }
        }
    }

    // Il tooltip dell'opzione sotto il mouse, a capo a 170 pixel
    if drag.is_none()
        && let Some(opt) = hovered_opt
    {
        let text = hud::tr(&opt.tooltip_key(), lang).to_owned();
        let lines: Vec<Text> = hud::wrap(&text, hud::TOOLTIP_WIDTH, &|t: &str| text_width(game, t))
            .iter()
            .map(|l| Text::literal(l))
            .collect();
        ui::tooltip(batch, atlas, &lines, cursor);
    }
    ControlFlow::Break(())
}

// ---- Spostamento col mouse del contatore semplice ----

// Il riquadro del contatore com'è adesso, in pixel GUI.
fn simple_box(game: &Game<'_>, config: &hud::Config, w: i32, h: i32) -> hud::SimpleBox {
    let days = game.world.day_time.div_euclid(24000);
    let day = hud::day_text(days, lang(game));
    hud::simple_box(config, w, h, text_width(game, &day))
}

// Il mondo resta visibile (sfocato) con sopra il contatore vero, un bordo che
// mostra dove afferrarlo, una riga di aiuto e Fatto.
fn draw_placing(game: &mut Game<'_>, batch: &mut HudBatch) {
    let (w, h) = (batch.width as i32, batch.height as i32);
    let cursor = game.cursor;
    let config = hud::with_state(|| None, |s| {
        let b = simple_box(game, &s.config, w, h);
        s.place_drag(&b, cursor, w, h);
        s.config.clone()
    });
    let grabbed = hud::with_state(|| None, |s| s.screen.grab.is_some());

    let atlas = &game.gui.atlas;
    let hint = hud::tr("customhud.place.hint", lang(game)).to_owned();
    let lines = hud::wrap(&hint, (w - 40).min(300), &|t: &str| text_width(game, t));
    let top = (h - lines.len() as i32 * 10) / 2;
    for (i, line) in lines.iter().enumerate() {
        let x = (w - text_width(game, line)) / 2;
        let y = top + i as i32 * 10;
        batch.text_styled(atlas, &Text::literal(line), x as f32, y as f32, color(0xFFAAAAAA), true);
    }
    let done = hud::place_done(w, h);
    let label = Text::literal(&vanilla(game).done);
    let (dx, dy, dw, dh) = (done.x as f32, done.y as f32, done.w as f32, done.h as f32);
    ui::button(batch, atlas, dx, dy, dw, dh, &label, true, done.contains(cursor));

    let frame = frame(game, batch, true);
    let out = hud::layout(&config, &frame, &|t: &str| text_width(game, t));
    if let Some(out) = out {
        draw_out(game, batch, &out);
    }
    // Bordo di 1 pixel attorno al riquadro: bianco se afferrato o sotto il mouse
    let b = simple_box(game, &config, w, h);
    let lit = grabbed || b.contains(cursor);
    let edge = color(if lit { 0xFFFFFFFF } else { 0x80FFFFFF });
    let (x0, y0) = (b.x as f32 - 1.0, b.y as f32 - 1.0);
    let (bw, bh) = (b.w as f32 * b.scale + 2.0, b.h as f32 * b.scale + 2.0);
    let atlas = &game.gui.atlas;
    batch.fill(atlas, x0, y0, bw, 1.0, edge);
    batch.fill(atlas, x0, y0 + bh - 1.0, bw, 1.0, edge);
    batch.fill(atlas, x0, y0 + 1.0, 1.0, bh - 2.0, edge);
    batch.fill(atlas, x0 + bw - 1.0, y0 + 1.0, 1.0, bh - 2.0, edge);
}

// Clic sul contatore per afferrarlo, su Fatto per tornare; frecce per i ritocchi;
// Esc torna alle impostazioni.
fn handle_placing(game: &mut Game<'_>, event: &WindowEvent) -> ControlFlow<()> {
    let (w, h) = game.gui_size();
    let (w, h) = (w as i32, h as i32);
    let cursor = game.cursor;
    let mut save_now = false;
    let mut back = false;
    match event {
        WindowEvent::CursorMoved { .. } => return ControlFlow::Continue(()),
        WindowEvent::MouseInput { state, button, .. } => {
            if *button != MouseButton::Left {
                return ControlFlow::Break(());
            }
            if *state == ElementState::Pressed {
                let config = hud::with_state(|| None, |s| s.config.clone());
                let b = simple_box(game, &config, w, h);
                let grabbed = hud::with_state(|| None, |s| s.place_press(&b, cursor));
                if !grabbed && hud::place_done(w, h).contains(cursor) {
                    game.audio.play(hud::CLICK_SOUND, None, 0.25, 1.0);
                    back = true;
                }
            } else {
                save_now = hud::with_state(|| None, |s| s.place_release());
            }
        }
        WindowEvent::KeyboardInput { event: key, .. } => {
            let PhysicalKey::Code(code) = key.physical_key else {
                return ControlFlow::Break(());
            };
            let name = horizon_client::keybinds::key_name(code);
            let window_key = name == Some(game.settings.keys.get("key.screenshot"))
                || name == Some(game.settings.keys.get("key.fullscreen"));
            if window_key {
                return ControlFlow::Continue(());
            }
            if key.state != ElementState::Pressed {
                return ControlFlow::Break(());
            }
            let nudge = match code {
                KeyCode::ArrowLeft => Some((-1, 0)),
                KeyCode::ArrowRight => Some((1, 0)),
                KeyCode::ArrowUp => Some((0, -1)),
                KeyCode::ArrowDown => Some((0, 1)),
                _ => None,
            };
            if let Some((dx, dy)) = nudge {
                hud::with_state(|| None, |s| s.place_nudge(dx, dy));
                save_now = true;
            } else if code == KeyCode::Escape {
                back = true;
            }
        }
        WindowEvent::MouseWheel { .. } => {}
        _ => return ControlFlow::Continue(()),
    }
    if back {
        hud::with_state(|| None, |s| s.place_close());
        save_now = true;
    }
    if save_now {
        let config = hud::with_state(|| None, |s| s.config.clone());
        save(game, &config);
    }
    ControlFlow::Break(())
}

// L'input della finestra mentre la schermata è aperta: clic, trascinamento,
// rotellina ed Esc vanno alla schermata; il movimento del mouse, gli screenshot
// e lo schermo intero restano al gioco.
pub(super) fn handle_event(game: &mut Game<'_>, event: &WindowEvent) -> ControlFlow<()> {
    if !screen_open(game) {
        return ControlFlow::Continue(());
    }
    if hud::with_state(|| None, |s| s.screen.placing) {
        return handle_placing(game, event);
    }
    let (w, h) = game.gui_size();
    let cursor = game.cursor;
    let shift = game.modifiers.shift_key();
    let effect = match event {
        WindowEvent::CursorMoved { .. } => return ControlFlow::Continue(()),
        WindowEvent::MouseInput { state, button, .. } => {
            if *button != MouseButton::Left {
                return ControlFlow::Break(());
            }
            hud::with_state(|| None, |s| {
                if *state == ElementState::Pressed {
                    let widgets = hud::screen_layout(s.screen.section, &s.config, w as i32, h as i32);
                    s.screen_click(&widgets, cursor, shift)
                } else {
                    s.screen_release()
                }
            })
        }
        WindowEvent::MouseWheel { delta, .. } => {
            let dy = match delta {
                MouseScrollDelta::LineDelta(_, y) => *y as f64,
                MouseScrollDelta::PixelDelta(p) => p.y,
            };
            if dy == 0.0 {
                return ControlFlow::Break(());
            }
            hud::with_state(|| None, |s| {
                let widgets = hud::screen_layout(s.screen.section, &s.config, w as i32, h as i32);
                s.screen_scroll(&widgets, cursor, dy > 0.0)
            })
        }
        WindowEvent::KeyboardInput { event: key, .. } => {
            let PhysicalKey::Code(code) = key.physical_key else {
                return ControlFlow::Break(());
            };
            let name = horizon_client::keybinds::key_name(code);
            let window_key = name == Some(game.settings.keys.get("key.screenshot"))
                || name == Some(game.settings.keys.get("key.fullscreen"));
            if window_key {
                return ControlFlow::Continue(());
            }
            if code == KeyCode::Escape && key.state == ElementState::Pressed {
                close_screen(game);
            }
            return ControlFlow::Break(());
        }
        _ => return ControlFlow::Continue(()),
    };
    if effect.click_sound {
        game.audio.play(hud::CLICK_SOUND, None, 0.25, 1.0);
    }
    if effect.save {
        let config = hud::with_state(|| None, |s| s.config.clone());
        save(game, &config);
    }
    if effect.close {
        close_screen(game);
    }
    ControlFlow::Break(())
}
