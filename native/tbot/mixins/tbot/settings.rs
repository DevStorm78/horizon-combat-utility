fn official_tbot_settings(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    buttons: &mut Vec<String>,
) {
    use crate::game::official_tbot as m;
    if mod_id != m::ID {
        return;
    }
    let game = &*game;
    buttons.extend(m::Settings::read(|key| game.native_setting(m::ID, key)).labels());
}
