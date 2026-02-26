//pub mod player;

//use crate::modules::player::Player;

use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
pub struct Player {
    view: StillImage,
    move_speed: f32,


}



impl Player{
 pub async fn new(
        asset_path: &str, 
        width: f32, 
        height: f32, 
        x: f32, 
        y: f32,
        stretch_enabled: bool,
        zoom_level: f32) -> Player{

        Player {
            view: StillImage::new(asset_path, width, height, x, y, stretch_enabled, zoom_level).await,
            move_speed: 400.0, // Default speed
        }
        }



#[allow(unused)]
pub fn moveing(&mut self)  -> Vec2 {


     // Direction to move in
    let mut move_dir = vec2(0.0, 0.0);

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
 let movement = move_dir * self.move_speed * get_frame_time();
    // Normalize the movement to prevent faster diagonal movement
    if move_dir.length() > 0.0 {
        move_dir = move_dir.normalize();
    }
 
      
        movement
    // Apply movement based on frame time


}

pub fn draw(&self) {
        // Only draw if the label is visible
       self.view.draw();
        }
    
#[allow(unused)]
 //change speed
    pub fn set_speed(&mut self, move_speed: f32) -> &mut Self {
        self.move_speed = move_speed;
        self
    }

   
    

    // Getter for position as Vec2
    #[allow(unused)]
    pub fn get_position(&self) -> Vec2 {
        Vec2::new(self.view.get_x(), self.view.get_y())
    }
    
    // Getter for visibility
    #[allow(unused)]
  
    pub fn view_player(&self) -> &StillImage {
        &self.view
    }
    // Setter for position
    #[allow(unused)]
    pub fn set_position(&mut self, x: f32, y: f32) -> &mut Self {
        self.view.set_x(x);
        self.view.set_y(y);
        self
    }
     pub fn get_x(&self) -> f32 {
        self.view.get_x()
    }

    #[allow(unused)]
    pub fn set_x(&mut self, x: f32) {
        self.view.set_x(x);
    }

    // Get and set y position
    #[allow(unused)]
    pub fn get_y(&self) -> f32 {
        self.view.get_y()
    }

    #[allow(unused)]
    pub fn set_y(&mut self, y: f32) {
        self.view.set_y(y);
    }
     pub fn pos(&self) -> Vec2 {
        vec2(self.view.get_x(), self.view.get_y())
    }

        pub fn check_collision_x(){

    }


    pub fn check_collision_y(){

    }
}
