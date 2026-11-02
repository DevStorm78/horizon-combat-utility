fn official_tbot_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::official_tbot as m;
    if !pressed || key != "key.keyboard.right.shift" { return; }
    if game.native_mod_screen() == Some(m::ID) {
        game.native_mod_screen_close(m::ID);
    } else {
        // A previous session may have ended while bind capture was active.
        game.native_set_setting(m::ID, "capture_bind", "0");
        game.native_mod_screen_open(m::ID);
    }
}
