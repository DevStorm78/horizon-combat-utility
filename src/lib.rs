use horizon_mod_sdk::{
    Mod, Registration, chat,
    game,
    hud::{self, Corner, Line},
    input,
    storage,
};

const MODULES: [&str; 9] = [
    "triggerbot",
    "wtap",
    "maceswap",
    "auto_lunge",
    "shield_breaker",
    "jump_reset",
    "sprint_reset",
    "cooldown_tracker",
    "click_trainer",
];

const ACTIONS_UNSUPPORTED: [&str; 7] = [
    "triggerbot", "wtap", "maceswap", "auto_lunge",
    "shield_breaker", "jump_reset", "sprint_reset",
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Profile { Minimal, Practice, Custom }

impl Profile {
    fn name(self) -> &'static str {
        match self {
            Self::Minimal => "minimal",
            Self::Practice => "practice",
            Self::Custom => "custom",
        }
    }
}

struct CombatUtility {
    enabled: [bool; MODULES.len()],
    hud_enabled: bool,
    profile: Profile,
    last_cps: (u32, u32),
    last_held: String,
    coach_enabled: bool,
    attack_was_down: bool,
    forward_was_down: bool,
    last_attack_tick: Option<u64>,
    forward_release_tick: Option<u64>,
    wtap_count: u32,
}

impl CombatUtility {
    fn read_bool(key: &str, default: bool) -> bool {
        storage::get(key)
            .and_then(|bytes| String::from_utf8(bytes).ok())
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(default)
    }

    fn write_bool(key: &str, value: bool) {
        if let Err(err) = storage::set(key, if value { b"1" } else { b"0" }) {
            horizon_mod_sdk::warn(&format!("could not save {key}: {err}"));
        }
    }

    fn index(name: &str) -> Option<usize> {
        let normalized = name.trim().to_ascii_lowercase().replace('-', "_");
        MODULES.iter().position(|item| *item == normalized)
    }

    fn unsupported(i: usize) -> bool {
        ACTIONS_UNSUPPORTED.iter().any(|name| *name == MODULES[i])
    }

    fn module_status(&self) -> String {
        MODULES.iter().enumerate().map(|(i, name)| {
            format!("{}={}{}", name, if self.enabled[i] { "ON" } else { "OFF" },
                if Self::unsupported(i) { " [UNSUPPORTED ACTION]" } else { "" })
        }).collect::<Vec<_>>().join(" | ")
    }

    fn refresh_hud(&mut self) {
        if !self.hud_enabled {
            hud::clear();
            return;
        }
        let mut lines = vec![
            Line { text: "Combat Utility".into(), color: 0xE6E6E6 },
            Line { text: format!("Profile: {}", self.profile.name()), color: 0xAAAAAA },
        ];

        if let Some(p) = game::get_player() {
            lines.push(Line {
                text: format!("HP {:.1} | Food {}", p.health, p.food),
                color: if p.health <= 6.0 { 0xFF5555 } else { 0x55DD88 },
            });
            lines.push(Line { text: format!("XYZ {:.1} {:.1} {:.1}", p.x, p.y, p.z), color: 0xAAAAAA });
        } else {
            lines.push(Line { text: "Not in a world".into(), color: 0xAAAAAA });
        }

        let cps = input::clicks_per_second();
        self.last_cps = cps;
        lines.push(Line { text: format!("CPS: attack {} | use {}", cps.0, cps.1), color: 0xE6E6E6 });

        self.last_held = input::held()
            .map(|item| format!("Held: {} (slot {})", item.id, item.slot + 1))
            .unwrap_or_else(|| "Held: empty".into());
        lines.push(Line { text: self.last_held.clone(), color: 0xAAAAAA });

        let enabled = MODULES.iter().enumerate()
            .filter(|(i, _)| self.enabled[*i])
            .map(|(_, name)| *name)
            .collect::<Vec<_>>().join(", ");
        lines.push(Line {
            text: format!("Enabled: {}", if enabled.is_empty() { "none" } else { &enabled }),
            color: 0xE6E6E6,
        });
        lines.push(Line { text: "Automation: unavailable in public mod API".into(), color: 0xFFB347 });
        hud::set_panel(Corner::TopLeft, &lines);
    }

    fn set_module(&mut self, index: usize, value: bool) {
        self.enabled[index] = value;
        Self::write_bool(&format!("mod_{}", MODULES[index]), value);
        self.profile = Profile::Custom;
        chat::show(&format!(
            "{}: {}{}",
            MODULES[index],
            if value { "ON" } else { "OFF" },
            if Self::unsupported(index) { " — preference only; action unsupported" } else { " — telemetry placeholder" }
        ));
    }

    fn profile(&mut self, profile: Profile) {
        self.profile = profile;
        if profile != Profile::Custom {
            for (i, name) in MODULES.iter().enumerate() {
                let on = match profile {
                    Profile::Minimal => *name == "cooldown_tracker",
                    Profile::Practice => *name == "cooldown_tracker" || *name == "click_trainer",
                    Profile::Custom => false,
                };
                self.enabled[i] = on;
                Self::write_bool(&format!("mod_{name}"), on);
            }
        }
        chat::show(&format!("Profile set to {}", profile.name()));
    }

    fn help() -> &'static str {
        "Commands: /combat status | modules | toggle <module> | on <module> | off <module> | profile <minimal|practice|custom> | coach <on|off> | hud <on|off>"
    }

    fn command_inner(&mut self, args: &str) -> Result<Option<String>, String> {
        let mut words = args.split_whitespace();
        let sub = words.next().unwrap_or("status").to_ascii_lowercase();
        match sub.as_str() {
            "help" => Ok(Some(Self::help().into())),
            "status" => Ok(Some(format!(
                "Combat Utility | profile={} | HUD={} | attack CPS={}, use CPS={} | W-tap coach={} (detected {})",
                self.profile.name(), if self.hud_enabled { "on" } else { "off" },
                self.last_cps.0, self.last_cps.1,
                if self.coach_enabled { "on" } else { "off" }, self.wtap_count
            ))),
            "modules" => Ok(Some(self.module_status())),
            "toggle" | "on" | "off" => {
                let name = words.next().ok_or("Usage: /combat toggle <module>")?;
                let i = Self::index(name).ok_or_else(|| format!("Unknown module '{name}'. Try /combat modules"))?;
                let value = if sub == "toggle" { !self.enabled[i] } else { sub == "on" };
                self.set_module(i, value);
                Ok(None)
            }
            "profile" => {
                match words.next().unwrap_or("minimal").to_ascii_lowercase().as_str() {
                    "minimal" => self.profile(Profile::Minimal),
                    "practice" => self.profile(Profile::Practice),
                    "custom" => self.profile = Profile::Custom,
                    _ => return Err("Usage: /combat profile <minimal|practice|custom>".into()),
                }
                Ok(None)
            }
            "coach" => {
                match words.next().unwrap_or("on").to_ascii_lowercase().as_str() {
                    "on" => self.coach_enabled = true,
                    "off" => self.coach_enabled = false,
                    _ => return Err("Usage: /combat coach <on|off>".into()),
                }
                Self::write_bool("coach_enabled", self.coach_enabled);
                Ok(Some(format!("W-tap input coach {}", if self.coach_enabled { "enabled" } else { "disabled" })))
            }
            "hud" => {
                match words.next().unwrap_or("on").to_ascii_lowercase().as_str() {
                    "on" => self.hud_enabled = true,
                    "off" => self.hud_enabled = false,
                    _ => return Err("Usage: /combat hud <on|off>".into()),
                }
                Self::write_bool("hud_enabled", self.hud_enabled);
                self.refresh_hud();
                Ok(Some(format!("HUD {}", if self.hud_enabled { "enabled" } else { "disabled" })))
            }
            _ => Ok(Some(Self::help().into())),
        }
    }
}

impl Mod for CombatUtility {
    fn init() -> Result<(Self, Registration), String> {
        let enabled = MODULES.map(|name| Self::read_bool(&format!("mod_{name}"), false));
        let hud_enabled = Self::read_bool("hud_enabled", true);
        let profile = if enabled.iter().any(|v| *v) { Profile::Custom } else { Profile::Minimal };
        Ok((Self {
            enabled, hud_enabled, profile, last_cps: (0, 0), last_held: String::new(),
            coach_enabled: Self::read_bool("coach_enabled", true),
            attack_was_down: false, forward_was_down: false,
            last_attack_tick: None, forward_release_tick: None, wtap_count: 0,
        }, Registration::new().command("combat").on_tick()))
    }

    fn tick(&mut self, tick: u64) {
        // Drain input events so the API's event queue does not accumulate.
        let _events = input::poll();
        let attack_down = input::mapping("key.attack").is_some_and(|m| m.down);
        let forward_down = input::mapping("key.forward").is_some_and(|m| m.down);

        // Observe the player's own W-tap rhythm. Horizon's public mod API
        // does not allow a mod to synthesize attack or movement inputs.
        if self.coach_enabled {
            if attack_down && !self.attack_was_down {
                self.last_attack_tick = Some(tick);
                self.forward_release_tick = None;
            }
            if let Some(attack_tick) = self.last_attack_tick {
                let elapsed = tick.saturating_sub(attack_tick);
                if !forward_down && self.forward_was_down && elapsed <= 4 {
                    self.forward_release_tick = Some(tick);
                }
                if forward_down && !self.forward_was_down {
                    if let Some(release_tick) = self.forward_release_tick.take() {
                        if tick > release_tick && tick.saturating_sub(release_tick) <= 5 {
                            self.wtap_count = self.wtap_count.saturating_add(1);
                        }
                    }
                }
                if elapsed > 9 {
                    self.last_attack_tick = None;
                    self.forward_release_tick = None;
                }
            }
        }
        self.attack_was_down = attack_down;
        self.forward_was_down = forward_down;
        if tick % 5 == 0 {
            self.refresh_hud();
        }
    }

    fn command(&mut self, _name: &str, args: &str) -> Result<Option<String>, String> {
        self.command_inner(args)
    }
}

horizon_mod_sdk::export_mod!(CombatUtility);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_names_accept_hyphens() {
        assert_eq!(CombatUtility::index("shield-breaker"), Some(4));
        assert_eq!(CombatUtility::index("wtap"), Some(1));
    }

    #[test]
    fn module_names_reject_unknown_values() {
        assert_eq!(CombatUtility::index("aimbot"), None);
    }
}
