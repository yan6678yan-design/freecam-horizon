// Adapter contract tests: compile the actual helper files against a small host.
// The separate native.py check validates these same helpers against real Horizon.
#[allow(dead_code)]
mod game {
    pub mod freecam_yannis {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/mixins/freecam_yannis/runtime.rs"
        ));
    }
    pub mod native_hooks {
        pub struct NativeMod {
            pub id: String,
            pub title: String,
            pub description: String,
            pub keys: Vec<[String; 3]>,
        }
    }
    #[derive(Clone)]
    pub struct Player {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub yaw: f32,
        pub pitch: f32,
        pub dimension: &'static str,
    }
    pub struct Game<'a> {
        pub on: bool,
        pub playing: bool,
        pub window: bool,
        pub player: Option<Player>,
        pub held: Vec<&'a str>,
    }
    impl Default for Game<'_> {
        fn default() -> Self {
            Self {
                on: true,
                playing: true,
                window: true,
                player: Some(Player {
                    x: 0.0,
                    y: 64.0,
                    z: 0.0,
                    yaw: 0.0,
                    pitch: 0.0,
                    dimension: "overworld",
                }),
                held: Vec::new(),
            }
        }
    }
    impl Game<'_> {
        pub fn native_on(&self, _: &str) -> bool {
            self.on
        }
        pub fn native_window(&self) -> bool {
            self.window
        }
        pub fn native_playing(&self) -> bool {
            self.playing
        }
        pub fn native_player(&self) -> Option<Player> {
            self.player.clone()
        }
        pub fn native_mod_key(&self, _: &str, _: &str, default: &str) -> String {
            default.into()
        }
        pub fn native_setting(&self, _: &str, _: &str) -> Option<String> {
            None
        }
        pub fn native_set_setting(&self, _: &str, _: &str, _: &str) {}
        pub fn native_mapping(&mut self, key: &str) -> Option<(bool, String)> {
            Some((self.held.contains(&key), key.into()))
        }
    }
}
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/mixins/freecam_yannis/render_hand.rs"
));
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/mixins/freecam_yannis/block_highlight.rs"
));
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/mixins/freecam_yannis/key.rs"
));
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/mixins/freecam_yannis/settings_click.rs"
));
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/mixins/freecam_yannis/body.rs"
));
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/mixins/freecam_yannis/interaction_key.rs"
));
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/mixins/freecam_yannis/client_disconnecting.rs"
));
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/mixins/freecam_yannis/turn.rs"
));
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/mixins/freecam_yannis/movement_input.rs"
));
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/mixins/freecam_yannis/mods.rs"
));
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/mixins/freecam_yannis/frame.rs"
));
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/mixins/freecam_yannis/client_level_unload.rs"
));
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/mixins/freecam_yannis/camera_pose.rs"
));
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/mixins/freecam_yannis/settings.rs"
));
#[test]
fn actual_hooks_route_input_and_restore_camera() {
    use game::freecam_yannis as m;
    let mut game = game::Game::default();
    freecam_key(&mut game, "key.keyboard.f4", true);
    freecam_key(&mut game, "key.keyboard.f4", false);
    assert!(m::detached());
    let mut forward = 1.0;
    let mut strafe = 1.0;
    let mut jump = true;
    let mut sneak = true;
    freecam_movement_input(&game, &mut forward, &mut strafe, &mut jump, &mut sneak);
    assert_eq!((forward, strafe, jump, sneak), (0.0, 0.0, false, false));
    assert!(freecam_turn(&game, 120.0, 20.0).is_break());
    let mut canceled = false;
    let mut swing = true;
    freecam_interaction_key(&game, &mut swing, &mut canceled);
    assert!(canceled && !swing);
    let saved = m::STATE.with(|s| s.borrow().pose);
    for _ in 0..1000 {
        freecam_key(&mut game, "key.keyboard.f6", true);
        assert!(freecam_turn(&game, 120.0, 20.0).is_continue());
        forward = 1.0;
        freecam_movement_input(&game, &mut forward, &mut strafe, &mut jump, &mut sneak);
        assert_eq!(forward, 1.0);
        freecam_key(&mut game, "key.keyboard.f6", true);
        assert_eq!(m::STATE.with(|s| s.borrow().pose), saved);
    }
    game.on = false;
    let mut position = [1.0; 3];
    let mut yaw = 0.0;
    let mut pitch = 0.0;
    freecam_camera_pose(&game, &mut position, &mut yaw, &mut pitch);
    assert_eq!(position, [1.0; 3]);
    freecam_frame(&mut game, std::time::Duration::from_millis(16));
    assert!(!m::active());
}
#[test]
fn menus_servers_and_disconnects_are_safe() {
    use game::freecam_yannis as m;
    let mut g = game::Game::default();
    g.playing = false;
    freecam_key(&mut g, "key.keyboard.f6", true);
    assert!(!m::active());
    g.playing = true;
    g.window = false;
    freecam_key(&mut g, "key.keyboard.f6", true);
    assert!(!m::active());
    g.window = true;
    freecam_key(&mut g, "key.keyboard.f6", true);
    assert!(m::active());
    freecam_client_disconnecting(&g);
    assert!(!m::active());
    g.player = None;
    freecam_key(&mut g, "key.keyboard.f6", true);
    assert!(!m::active());
}
#[test]
fn chord_does_not_toggle_off_the_tripod_on_release() {
    use game::freecam_yannis as m;
    let mut g = game::Game::default();
    freecam_key(&mut g, "key.keyboard.f4", true);
    freecam_key(&mut g, "key.keyboard.1", true);
    freecam_key(&mut g, "key.keyboard.f4", false);
    assert!(m::detached());
    assert!(m::STATE.with(|s| s.borrow().slots[0].is_some()));
    freecam_client_level_unload(&g);
    assert!(!m::active());
}
#[test]
fn settings_and_render_hooks_respect_mod_state() {
    let mut g = game::Game::default();
    let mut mods = Vec::new();
    freecam_mods(&mut mods);
    assert_eq!(mods[0].id, "freecam_yannis");
    let mut buttons = Vec::new();
    freecam_settings(&g, "other_mod", &mut buttons);
    assert!(buttons.is_empty());
    freecam_settings(&g, "freecam_yannis", &mut buttons);
    assert_eq!(buttons.len(), 5);
    freecam_settings_click(&mut g, "freecam_yannis", 0);
    assert_eq!(
        game::freecam_yannis::STATE.with(|s| s.borrow().settings.unwrap().speed),
        20.0
    );
    freecam_key(&mut g, "key.keyboard.f6", true);
    let mut hidden = false;
    freecam_render_hand(&g, &mut hidden);
    assert!(hidden);
    assert_eq!(freecam_body(&g), core::ops::ControlFlow::Break(true));
    let mut outline = false;
    freecam_block_highlight(&g, &mut outline);
    assert!(!outline);
    freecam_key(&mut g, "key.keyboard.f4", true);
    freecam_key(&mut g, "key.keyboard.f4", false);
    freecam_block_highlight(&g, &mut outline);
    assert!(outline);
    g.on = false;
    hidden = false;
    freecam_render_hand(&g, &mut hidden);
    assert!(!hidden);
    assert!(freecam_body(&g).is_continue());
}
