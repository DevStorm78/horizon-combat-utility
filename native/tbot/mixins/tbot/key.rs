fn official_tbot_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::official_tbot as m;
    if pressed && key == "key.keyboard.right.shift" {
        game.native_mod_screen_open(m::ID);
    }
}
