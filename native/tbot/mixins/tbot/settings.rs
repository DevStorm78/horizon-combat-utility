fn official_tbot_settings(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    buttons: &mut Vec<String>,
) {
    use crate::game::official_tbot as m;
    if mod_id != m::ID {
        return;
    }
    let settings = m::settings(|key| game.native_setting(m::ID, key));
    buttons.extend(settings.labels().into_iter().skip(1));
    buttons.push("In-game menu: press Right Shift".into());
}
