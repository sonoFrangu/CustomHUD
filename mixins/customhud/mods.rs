// La voce della mod in Options > Mods, con il tasto che apre le impostazioni.
fn list_customhud(game: &crate::game::Game<'_>, mods: &mut Vec<NativeMod>) {
    use crate::game::customhud as hud;
    let lang = hud::Lang::from_code(&game.settings.language);
    mods.push(NativeMod {
        id: hud::MOD_ID.to_owned(),
        title: hud::TITLE.to_owned(),
        description: hud::tr("customhud.mod.description", lang).to_owned(),
        keys: vec![[
            hud::CONFIG_KEY.to_owned(),
            hud::tr("key.customhud.open_config", lang).to_owned(),
            hud::CONFIG_KEY_DEFAULT.to_owned(),
        ]],
    });
}
