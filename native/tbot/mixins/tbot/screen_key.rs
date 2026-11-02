fn official_tbot_screen_key(
    game: &crate::game::Game<'_>,
    screen: &crate::game::native_hooks::NativeScreenView<'_>,
    key: &str,
    repeat: bool,
    _modifiers: u32,
    canceled: &mut bool,
) {
    use crate::game::official_tbot as m;
    if game.native_mod_screen() == Some(m::ID) && !repeat
        && (key == "key.keyboard.right.shift" || key == "key.keyboard.escape")
    {
        *canceled = true;
    }
}
