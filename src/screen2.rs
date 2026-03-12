use crate::modules::collision::check_collision;
use crate::modules::label::Label;
use crate::modules::move_wall::MoveWall;
use crate::modules::player::Player;
use crate::modules::scale::{VIRTUAL_RESOLUTION, use_virtual_resolution};
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use macroquad::prelude::*;
pub async fn run() -> String {
   // const VIRTUAL_WIDTH: f32 = 1024.0;
   // const VIRTUAL_HEIGHT: f32 = 768.0;

    let mut amongus = Player::new(
        "assets/amongus.png",
        50.0, // width
        50.0, // height
        70.0, // x position
        0.0,  // y position
        true, // Enable stretching
        1.0,  // Normal zoom (100%)
    )
    .await;

    let mut wall = MoveWall::new(
        "assets/wall.png",
        100.0, // width
        50.0,  // height
        200.0, // x position
        400.0, // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
        "horizontal".to_string(),
    )
    .await;

    let maze = StillImage::new(
        "assets/maze.png",
        1024.0, // width
        768.0,  // height
        0.0,    // x position
        0.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    )
    .await;
    let mut key = StillImage::new(
        "assets/key.png",
        40.0,  // width
        40.0,  // height
        800.0, // x position
        30.0,  // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;
    let mut door = StillImage::new(
        "assets/door.png",
        200.0, // width
        500.0, // height
        -30.0, // x position
        513.0, // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;
    // Speed of movement in pixels per second

    let btn_exit = TextButton::new(950.0, 20.0, 50.0, 50.0, "X", BLUE, GREEN, 30);
    let mut lbl_time = Label::new("Time", 30.0, 20.0, 30);
    let mut lbl_timer = Label::new("timer", 100.0, 20.0, 30);
    lbl_time.with_colors(WHITE, None);
    lbl_timer.with_colors(WHITE, None);
    lbl_time.with_border(BLACK, 0.0);
    lbl_timer.with_border(BLACK, 0.0);
    let start_time = get_time();

    loop {
        use_virtual_resolution(1024.0, 768.0);
       
        let timer = get_time() - start_time;

        if timer != start_time {
            lbl_timer.set_text(format!("{:.0}", timer));
        }
        if timer >= 60.0 {
            return "screen1".to_string();
        }

        clear_background(DARKGRAY);

        // Direction to move in

        if timer > 0.1 {
            // Save old position in case of collision
            amongus.moveing();
            let old_pos = amongus.pos();
            if amongus.move_check_collision_x(&maze) {
                amongus.set_x(old_pos.x);
            }

            if amongus.move_check_collision_y(&maze) {
                amongus.set_y(old_pos.y);
            }
            if check_collision(amongus.view_player(), wall.view_player(), 1) {
                amongus.set_y(wall.get_y() - 50.0);
                if check_collision(amongus.view_player(), &maze, 1) {
                    amongus.set_y(amongus.get_y() + 10.0);
                }
            }
            if amongus.get_x() < 40.0 && amongus.get_y() < 580.0 && amongus.get_y() > 537.0 {
                amongus.set_y(amongus.get_y() + 20.0);
            }

            if btn_exit.click() {
                return "screen1".to_string();
            }
            wall.moveing(100.0, 50.0, VIRTUAL_RESOLUTION.take().0, VIRTUAL_RESOLUTION.take().1);
            if check_collision(wall.view_player(), amongus.view_player(), 1) {
                amongus.set_y(old_pos.y);
                amongus.set_x(old_pos.x);
            }
            if check_collision(amongus.view_player(), &key, 1) {
                key.clear();
                door.clear();
            }
            if check_collision(amongus.view_player(), &door, 1) {
                amongus.set_y(old_pos.y);
            }

            if amongus.get_x() < 70.0 && amongus.get_y() > 880.0 {
                return "screen1".to_string();
            }
        }
        maze.draw();
        amongus.draw();
        lbl_time.draw();
        lbl_timer.draw();
        wall.draw();
        key.draw();
        door.draw();
        next_frame().await;
    }
}
