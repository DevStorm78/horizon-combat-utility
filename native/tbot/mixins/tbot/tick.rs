fn official_tbot_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::official_tbot as m;

    // native_tick runs on both the window's Game and the integrated/dedicated server Game.
    // The client-role guard and thread-local hold state keep server ticks out of this path.
    if !game.native_window() {
        return;
    }

    let in_world = game.native_playing() && game.player.is_some();
    let module_on = game.native_on(m::ID);
    if !module_on || !in_world {
        m::set_bind_down(false);
        return;
    }

    let settings = m::settings(|key| game.native_setting(m::ID, key));
    let Some((target_index, distance)) = game.mob_hit() else {
        return;
    };
    let Some(target) = game.mobs.get(target_index) else {
        return;
    };
    let target_is_player = &*target.kind.id == "minecraft:player";

    if !m::should_attack(
        module_on,
        in_world,
        m::bind_down(),
        &settings,
        target_is_player,
        distance,
        game.attack_strength_scale(),
    ) {
        return;
    }

    // Horizon's ordinary attack implementation enforces block and reach checks,
    // applies normal damage/knockback/crit/sweep handling, and swings the arm.
    let _ = game.attack();
}
