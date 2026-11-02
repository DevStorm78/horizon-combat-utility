fn official_tbot_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::official_tbot as m;

    let settings = m::Settings::read(|key| game.native_setting(m::ID, key));
    use std::sync::atomic::Ordering;

    let in_world = game.native_mod_screen() != Some(m::ID) && game.player.is_some();
    let has_target = game.mob_hit().is_some();
    if !m::should_trigger(
        settings.enabled,
        settings.holding_bind_only,
        m::HOLD_BIND_DOWN.load(Ordering::Relaxed),
        in_world,
        has_target,
        game.attack_strength_scale(),
    ) {
        return;
    }

    // Horizon's normal attack path uses the current crosshair hit and its
    // ordinary reach/game-mode checks; no alternate target selection occurs.
    let _ = game.attack();
}
