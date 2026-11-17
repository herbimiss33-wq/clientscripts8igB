//! ClientScripts — experimental native mod.
//! This mod's first integration is a safe diagnostic hook to verify
//! Horizon can compile and load the GitHub project.
//! The .csk runtime is provided separately in the same source tree,
//! but game input automation is NOT wired up yet.

mod scripts;

use mod_api::{self, Hooks, PlayerLoggedInEvent, ServerHost, Text};

#[derive(Default)]
struct ClientScripts;

#[mod_api::horizon_mod]
impl Hooks for ClientScripts {
    fn player_logged_in(&self, host: &mut dyn ServerHost, event: &mut PlayerLoggedInEvent<'_>) {
        if let Some(player) = host.player(event.player) {
            let result = scripts::example_diagnostics();
            let _ = player.tell(host, Text::literal(format!("[ClientScripts] {result}")));
        }
    }
}
