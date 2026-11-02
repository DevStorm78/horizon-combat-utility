fn official_tbot_screen_key(
    game: &crate::game::Game<'_>,
    _screen: &crate::game::native_hooks::NativeScreenView<'_>,
    key: &str,
    repeat: bool,
    _modifiers: u32,
    canceled: &mut bool,
) {
    use crate::game::official_tbot as m;
    if game.native_mod_screen() != Some(m::ID) || repeat { return; }

    let capturing = game.native_setting(m::ID, "capture_bind").as_deref() == Some("1");
    if capturing {
        if key == "key.keyboard.escape" {
            game.native_set_setting(m::ID, "capture_bind", "0");
            *canceled = true;
            return;
        }
        if key == "key.keyboard.right.shift" {
            game.native_set_setting(m::ID, "capture_bind", "0");
            game.native_mod_screen_close(m::ID);
            *canceled = true;
            return;
        }
        if key.starts_with("key.keyboard.") {
            game.native_set_setting(m::ID, "bind", key);
            game.native_set_setting(m::ID, "capture_bind", "0");
            *canceled = true;
        }
        return;
    }

    if key == "key.keyboard.right.shift" || key == "key.keyboard.escape" {
        game.native_mod_screen_close(m::ID);
        *canceled = true;
    }
}
