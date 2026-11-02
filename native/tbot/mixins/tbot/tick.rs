fn official_tbot_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::official_tbot as m;

    let settings = m::Settings::read(|key| game.native_setting(m::ID, key));
    if !settings.enabled {
        return;
    }
    if settings.holding_bind_only
        && game.native_setting(m::ID, "bind_down").as_deref() != Some("1")
    {
        return;
    }
    if game.native_mod_screen() == Some(m::ID) || game.player.is_none() {
        return;
    }

    // Use Horizon's normal crosshair ray and attack implementation. The
    // triggerbot never selects targets behind walls or outside entity reach.
    if game.attack_strength_scale() < 0.9 || game.mob_hit().is_none() {
        return;
    }
    let _ = game.attack();
}
