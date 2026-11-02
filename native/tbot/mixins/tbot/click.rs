fn official_tbot_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::official_tbot as m;
    if mod_id != m::ID {
        return;
    }
    let mut settings = m::Settings::read(|key| game.native_setting(m::ID, key));
    settings.click(button);
    for (key, value) in settings.stored() {
        game.native_set_setting(m::ID, key, &value);
    }
}
