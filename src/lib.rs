#![allow(non_snake_case)]   
use wasm_bindgen::prelude::*;

const ROOT2: f32 = 1.414213562;
const PLAYERSPEED: f32 = 500.0;
fn inputCheck(input: i32, pos: i8) -> bool{
    (input & (1<<pos)) != 0
}//[w,a,s,d]
struct Player {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    colour: i8,
}
impl Player{
    fn new(x: f32, y: f32, w:f32, h:f32, colour: i8) -> Player{
        Player { x, y, w, h ,colour}
    }
    fn update(&mut self, delta: f32, input: i32){
        let mut dx: f32= 0.0;
        let mut dy: f32 = 0.0;

        if inputCheck(input, 0){
            dy -= 1.0;
        }if inputCheck(input, 1){
            dx -= 1.0;
        }if inputCheck(input, 2){
            dy += 1.0;
        }if inputCheck(input, 3){
            dx += 1.0;
        }

        if dx.abs() + dy.abs() == 2.0{
            dx *= ROOT2;
            dy *= ROOT2
        }

        self.x += dx * PLAYERSPEED * delta;
        self.y += dy * PLAYERSPEED * delta;
    }
    fn draw(&self, drawData: &mut Vec<f32>){
        drawData.push(self.x);
        drawData.push(self.y);
        drawData.push(self.w);
        drawData.push(self.h);
        drawData.push(self.colour as f32);
    }
}

#[wasm_bindgen]
pub struct Game{
    player: Player
}
#[wasm_bindgen]
impl Game{
    #[wasm_bindgen(constructor)]
    pub fn new(px: f32, py: f32) ->Game{
        Game {player: Player::new(px,py,50.0,50.0,0)}
    }
    pub fn update(&mut self,delta: f32,input: i32) -> Vec<f32>{

        self.player.update(delta, input);

        let mut drawData: Vec<f32> = vec![];
        self.player.draw(&mut drawData);
        
        drawData
    }
}