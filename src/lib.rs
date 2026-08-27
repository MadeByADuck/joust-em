#![allow(non_snake_case)]   

use wasm_bindgen::prelude::*;

//£ Consts
const RECROOT2: f64 = 0.7071067812; 
const PLAYERSPEED: f64 = 500.0;
const CANVAS_W: f64 = 1280.0;
const CANVAS_H: f64 = 720.0;
const INPUTAMO: usize = 10;

const BOUND_X: f64 = 50.0;
const BOUND_Y: f64 = 150.0;
const BOUND_W: f64 = CANVAS_W/2.0 - BOUND_X*2.0;
const BOUND_H: f64 = CANVAS_H - BOUND_Y - 50.0;
const BOUND: Rect = Rect{x:BOUND_X, y: BOUND_Y, w: BOUND_W, h: BOUND_H};

const BORDERTHICK: f64 = 5.0;

const TOPBOUND: Rect = Rect{
    x: 25.0,
    y: 25.0,
    w: 1230.0,
    h: 100.0
};

fn drawFRect(drawData: &mut Vec<f64>,rect: &Rect,colour: i8){
    drawData.push(0.0);
    drawData.push(rect.x);
    drawData.push(rect.y);
    drawData.push(rect.w);
    drawData.push(rect.h);
    drawData.push(colour as f64);
}
fn drawSRect(drawData: &mut Vec<f64>, rect: &Rect, colour: i8, thickness: f64){
    drawData.push(1.0);
    drawData.push(rect.x);
    drawData.push(rect.y);
    drawData.push(rect.w);
    drawData.push(rect.h);
    drawData.push(colour as f64);
    drawData.push(thickness);
}
fn drawCircle(drawData: &mut Vec<f64>, circle: &Circle, colour: i8){
    drawData.push(2.0);
    drawData.push(circle.x);
    drawData.push(circle.y);
    drawData.push(circle.r);
    drawData.push(colour as f64);
}

#[derive(Clone, Copy)]
struct Rect{
    x: f64,
    y: f64,
    w: f64,
    h: f64
}
#[derive(Clone, Copy)]
struct Circle{
    x: f64,
    y: f64,
    r: f64
}
struct Player {
    left: bool,
    sheild: Circle,
    lance:  Circle,
    bounds: Rect,
    sheildSelected: bool,
    canSwap: bool
}
impl Player{ //£Player
    fn new(left: bool) -> Player{
        let mut bounds = BOUND.clone();
        if !left{
            bounds.x += CANVAS_W/2.0
        }
        let x: f64 = bounds.x + BOUND_W/2.0;
        let y: f64 = bounds.y + BOUND_H/2.0;
        Player {
            left: left,
            sheild: Circle { x, y, r: (80.0) },
            lance: Circle { x, y, r: (50.0) },
            bounds: bounds,
            sheildSelected:false, 
            canSwap:true,
        }
    }
    fn update(&mut self, delta: f64, inputs: &[bool; INPUTAMO]){
        let inpAdd:usize  = if self.left {
            0
        }else{
            5
        };

        let mut dx: f64= 0.0;
        let mut dy: f64 = 0.0;

        if inputs[0+inpAdd]{
            dy -= 1.0;
        }if inputs[1+inpAdd]{
            dx -= 1.0;
        }if inputs[2+inpAdd]{
            dy += 1.0;
        }if inputs[3+inpAdd]{
            dx += 1.0;
        }

        if dx.abs() + dy.abs() == 2.0{
            dx *= RECROOT2;
            dy *= RECROOT2;
        }

        if inputs[4+inpAdd]{
            self.canSwap = true;
        }else if self.canSwap{
            self.sheildSelected = !self.sheildSelected;
            self.canSwap = false;
        }

        if self.sheildSelected{
            let nextX: f64 = self.sheild.x + dx * PLAYERSPEED * delta;
            let nextY: f64 = self.sheild.y + dy * PLAYERSPEED * delta;

            if nextX-self.sheild.r >= self.bounds.x && nextX+self.sheild.r <= self.bounds.x + self.bounds.w{
                self.sheild.x = nextX;
            }
            if nextY-self.sheild.r >= self.bounds.y && nextY+self.sheild.r <= self.bounds.y + self.bounds.h{
                self.sheild.y = nextY;
            }
            
        }else{
            let nextX: f64 = self.lance.x + dx * PLAYERSPEED * delta;
            let nextY: f64 = self.lance.y + dy * PLAYERSPEED * delta;

            if nextX-self.lance.r >= self.bounds.x && nextX+self.lance.r <= self.bounds.x + self.bounds.w{
                self.lance.x = nextX;
            }
            if nextY-self.lance.r >= self.bounds.y && nextY+self.lance.r <= self.bounds.y + self.bounds.h{
                self.lance.y = nextY;
            }
        }

        
    }
    fn draw(&self, drawData: &mut Vec<f64>){
        drawSRect(drawData, &self.bounds, 0, BORDERTHICK);
        drawCircle(drawData, &self.sheild, 2);
        drawCircle(drawData, &self.lance, 1);
    }
    fn drawShadow(&self, drawData: &mut Vec<f64>){
        let mut flipShield = self.sheild.clone();
        let mut flipLance = self.lance.clone();

        flipShield.x = CANVAS_W - flipShield.x;
        flipLance.x = CANVAS_W - flipLance.x;

        drawCircle(drawData, &flipShield, 3);
        drawCircle(drawData, &flipLance, 3);
    }
}

#[wasm_bindgen]
pub struct Game{
    player1: Player,
    player2: Player,
}
#[wasm_bindgen]
impl Game{ //£Game
    #[wasm_bindgen(constructor)]
    pub fn new() ->Self{
        Game {
            player1: Player::new(true),
            player2: Player::new(false),
        }
    }
    pub fn update(&mut self,delta: f64,input: i32) -> Vec<f64>{
        let mut inputs = [false;INPUTAMO];
        for i in  0..inputs.len(){
            inputs[i] = (input & (1<<i)) != 0
        }


        self.player1.update(delta,&inputs);
        self.player2.update(delta,&inputs);

        let mut drawData: Vec<f64> = vec![]; //£Draw
        drawSRect(&mut drawData, &TOPBOUND, 0, BORDERTHICK);
        
        self.player1.draw(&mut drawData);
        self.player2.draw(&mut drawData);
        self.player1.drawShadow(&mut drawData);
        self.player2.drawShadow(&mut drawData);
        
        drawData
    }
}