use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Simple Platformer".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut player_x = 100.0;
    let mut player_y = 400.0;
    let mut player_vel_y = 0.0;
    let mut on_ground = false;

    loop {
        // Clear screen with light blue background
        clear_background(SKYBLUE);

        // Get delta time
        let dt = get_frame_time();

        // Handle input
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            player_x -= 150.0 * dt;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            player_x += 150.0 * dt;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            player_y += 150.0 * dt;
        }
        if (is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up)) && on_ground {
            player_vel_y = -300.0;
        }

        // Apply gravity
        player_vel_y += 500.0 * dt;
        player_y += player_vel_y * dt;

        // Simple ground collision
        if player_y > 500.0 {
            player_y = 500.0;
            player_vel_y = 0.0;
            on_ground = true;
        } else {
            on_ground = false;
        }

        // Keep player on screen
        if player_x < 0.0 { player_x = 0.0; }
        if player_x > 750.0 { player_x = 750.0; }

        // Draw ground
        draw_rectangle(0.0, 500.0, 800.0, 100.0, GREEN);

        // Draw player
        draw_rectangle(player_x, player_y, 50.0, 50.0, RED);

        // Draw simple instructions
        draw_text("WASD or Arrow Keys to move", 10.0, 30.0, 24.0, BLACK);
        draw_text("W or Up to jump", 10.0, 60.0, 24.0, BLACK);

        // Update frame
        next_frame().await;
    }
}