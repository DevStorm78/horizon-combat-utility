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

    if game.native_setting(m::ID, "capture_bind").as_deref() != Some("1") {
        // Let vanilla Escape close the screen and let the native key hook
        // handle Right Shift after this pre-hook.
        return;
    }

    if key == "key.keyboard.escape" {
        game.native_set_setting(m::ID, "capture_bind", "0");
        *canceled = true;
        return;
    }
    if key == "key.keyboard.right.shift" {
        // Do not cancel: key.rs sees it and closes the menu.
        game.native_set_setting(m::ID, "capture_bind", "0");
        return;
    }
    if key.starts_with("key.keyboard.") {
        game.native_set_setting(m::ID, "bind", key);
        game.native_set_setting(m::ID, "capture_bind", "0");
        *canceled = true;
    }
}
