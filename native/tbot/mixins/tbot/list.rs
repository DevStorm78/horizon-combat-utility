fn official_tbot_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::official_tbot as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: vec![[m::KEY.into(), "Hold to Activate".into(), m::DEFAULT_KEY.into()]],
    });
}
