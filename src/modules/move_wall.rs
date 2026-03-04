/*

pub mod move_wall;

use crate::modules::move_wall::MoveWall;
*/
use macroquad::prelude::*;
use crate::modules::still_image::StillImage;

pub struct MoveWall {
    view: StillImage,
    move_speed: f32,
    movement: Vec2,
    direction: String,
    


}



impl MoveWall{
 pub async fn new(
        asset_path: &str, 
        width: f32, 
        height: f32, 
        x: f32, 
        y: f32,
        stretch_enabled: bool,
        zoom_level: f32,
        direction: String) -> MoveWall{

      MoveWall {
            view: StillImage::new(asset_path, width, height, x, y, stretch_enabled, zoom_level).await,
            move_speed: 200.0, // Default speed
            movement: Vec2::ZERO,
            direction,
            
        }
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
     #[allow(unused)]
     pub fn pos(&self) -> Vec2 {
        vec2(self.view.get_x(), self.view.get_y())
    }
    #[allow(unused)]
    pub fn get_speed(&self) -> f32 {
        self.move_speed
    }
   
 #[allow(unused)]
    pub fn moveing(&mut self, width: f32, hight: f32)   {
        if self.direction == "horizontal" {
            self.movement.x = self.move_speed * get_frame_time();
            self.movement.y = 0.0;
            self.set_x(self.get_x()+self.movement.x);
            if self.get_x() > screen_width()-width|| self.get_x() < 0.0{
                self.move_speed = self.move_speed * -1.0;
                
            }
        } else  if self.direction == "vertical" {
            self.movement.y = self.move_speed * get_frame_time();
            self.movement.x = 0.0;
             self.set_y(self.get_y()+self.movement.y);
            if self.get_y() > screen_height()-hight|| self.get_y() < 0.0{
                self.move_speed = self.move_speed * -1.0;
                
            }
        }
       
    }

    }