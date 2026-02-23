use core::time;

use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
use crate::modules::collision::check_collision;
use crate::modules::text_button::TextButton;
use crate::modules::label::Label;
use crate::modules::grid::draw_grid;
pub async fn run() -> String {
    
    let mut amongus = StillImage::new(
        "assets/amongus.png",
        50.0,  // width
        50.0,  // height
        70.0,  // x position
        0.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;

    let maze = StillImage::new(
        "assets/maze.png",
        1024.0,  // width
        768.0,  // height
        0.0,  // x position
        0.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    // Speed of movement in pixels per second
const MOVE_SPEED: f32 = 400.0;

 let btn_exit = TextButton::new(
        950.0,
        20.0,
        50.0,
        50.0,
        "X",
        BLUE,
        GREEN,
        30
    );
     let mut lbl_time = Label::new("Time", 30.0, 20.0, 30);
      let mut lbl_timer = Label::new("timer", 100.0, 20.0, 30);
      lbl_time.with_colors(WHITE, None);
      lbl_timer.with_colors(WHITE, None);
      lbl_time.with_border(BLACK, 0.0);
      lbl_timer.with_border(BLACK, 0.0);
      let start_time = get_time();

    loop {
        let mut timer = get_time() - start_time;



 lbl_timer.set_text(format!("{:.0}", timer));

if timer >= 60.0 {
            return "screen1".to_string();
        }


        clear_background(DARKGRAY);
        

    // Direction to move in
    let mut move_dir = vec2(0.0, 0.0);
if timer>0.1{
    // Keyboard input
    if is_key_down(KeyCode::D) {
        move_dir.x += 1.0;
    }
    if is_key_down(KeyCode::A) {
        move_dir.x -= 1.0;
    }
    if is_key_down(KeyCode::S) {
        move_dir.y += 1.0;
    }
    if is_key_down(KeyCode::W) {
        move_dir.y -= 1.0;
    }

    // Normalize the movement to prevent faster diagonal movement
    if move_dir.length() > 0.0 {
        move_dir = move_dir.normalize();
    }

    // Apply movement based on frame time
    let movement = move_dir * MOVE_SPEED * get_frame_time();

    // Save old position in case of collision
    let old_pos = amongus.pos();

    // Move X first
    if movement.x != 0.0 {
      
        amongus.set_x(amongus.get_x() + movement.x);
          
        if check_collision(&amongus, &maze, 1) {
            amongus.set_x(old_pos.x); // Undo if collision happens
        }//println!("X Position: {}", amongus.get_x());
    }

    // Move Y next
    if movement.y != 0.0 {
        amongus.set_y(amongus.get_y() + movement.y);
        
        if check_collision(&amongus, &maze, 1)  {
            amongus.set_y(old_pos.y); // Undo if collision happens
        }//println!("Y Position: {}", amongus.get_y());
    }
if amongus.get_x() < 40.0 && amongus.get_y() < 580.0 && amongus.get_y() > 537.0 {
            amongus.set_y(amongus.get_y() + 20.0);
            
        }

     if btn_exit.click() {
            return "screen1".to_string();
        }



        if amongus.get_x() <70.0 && amongus.get_y() > 880.0 {
            return "screen1".to_string();
        }}
        maze.draw();
        amongus.draw();
        lbl_time.draw();
        lbl_timer.draw();
        
        next_frame().await;
    }
}