use rusty_engine::prelude::*;
use rand::prelude::*;

const PLAYER_SPEED: f32 = 250.0;
const ROAD_SPEED: f32 = 400.0;
struct GameState {
    hit_points: u8,
    lost: bool,
}

fn main() {
    let mut game = Game::new();

    let player_one = game.add_sprite("player_one", SpritePreset::RacingCarBlue);
    player_one.translation.x = -500.0;
    player_one.layer = 10.0;
    player_one.collision = true;

    // Music
    game.audio_manager.play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    // Create road
    for i in 0..10 {
        let road_line = game.add_sprite(format!("road_line_{}", i), SpritePreset::RacingBarrierWhite);
        road_line.translation.x = -600.0 + 150.0 * (i as f32);
    }

    //Create obstacles
    let obstace_presets = vec![
        SpritePreset::RacingBarrelBlue,
        SpritePreset::RacingBarrelRed,
        SpritePreset::RacingBarrelRed
    ];

    for (i, preset) in obstace_presets.into_iter().enumerate() {
        let obstacle = game.add_sprite(format!("obstacle_{}", i), preset);
        obstacle.layer = 5.0;
        obstacle.collision = true;
        obstacle.translation.x = thread_rng().gen_range(800.0..1600.0);
        obstacle.translation.y = thread_rng().gen_range(-300.0..300.0);
    }

    // Health stats
    let health_message = game.add_text("health_message", "Health: 5");
    health_message.translation = Vec2::new(550.0, 320.0);

    game.add_logic(game_logic);
    game.run(GameState {
        hit_points: 5,
        lost: false,
    });
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    
    if game_state.lost {
        return;
    }
    
    let mut direction = 0.0;
    
    if engine.keyboard_state.pressed(KeyCode::Up) {
        direction += 1.0
    }

    if engine.keyboard_state.pressed(KeyCode::Down) {
        direction -= 1.0;
    }

    let player_one = engine.sprites.get_mut("player_one").unwrap();
    player_one.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
    player_one.rotation = direction * 0.15;

    if player_one.translation.y < -360.0 || player_one.translation.y > 360.0 {
        game_state.hit_points = 0;
    }

    // Move road objects
    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("road_line") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;

            if sprite.translation.x < -675.0 {
                sprite.translation.x += 1500.0;
            }
        }

        if sprite.label.starts_with("obstacle") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;

            if sprite.translation.x < -800.0 {
                sprite.translation.x = thread_rng().gen_range(800.0..1600.0);
                sprite.translation.y = thread_rng().gen_range(-300.0..300.0);
            }
        }
    }

    // Health
    for event in engine.collision_events.drain(..) {
        
        if !event.pair.either_contains("player_one") || event.state.is_end() {
            continue;
        }

        if game_state.hit_points > 0 {
            let health_message = engine.texts.get_mut("health_message").unwrap();
            game_state.hit_points -= 1;
            health_message.value = format!("Health: {}", game_state.hit_points);
            engine.audio_manager.play_sfx(SfxPreset::Impact3, 0.5);
        }
    }

    if game_state.hit_points == 0 {
        game_state.lost = true;
        let game_over = engine.add_text("game_over", "GAME OVER");
        game_over.font_size = 128.0;
        engine.audio_manager.stop_music();
        engine.audio_manager.play_sfx(SfxPreset::Jingle3, 0.5);
    }
}