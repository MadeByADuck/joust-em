#![allow(non_snake_case)]   

use wasm_bindgen::prelude::*;


//£ Consts
const RECROOT2: f64 = 0.7071067812; 
const PLAYERSPEED: f64 = 500.0;
const CANVAS_W: f64 = 1280.0;
const CANVAS_H: f64 = 720.0;
const INPUTAMO: usize = 12;

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
const HORSEW: f64 = 100.0;
const HORSEACC: f64 = 20.0;

fn getScrR(dist: f64, r: f64) -> f64{
    (-0.048*dist + r).max(0.001)
}
fn pointDist(x1:f64, y1: f64, x2: f64, y2: f64) -> f64{
    ((x1-x2)*(x1-x2) + (y1-y2)*(y1-y2)).sqrt()
}
fn cirCirColi(a: Circle, b:Circle) -> bool{
    return pointDist(a.x, a.y, b.x, b.y) <= a.r + b.r
}
fn cirCirEngulf(s: Circle, b: Circle) -> bool{
    return pointDist(s.x, s.y, b.x, b.y) <= b.r-s.r;
}

fn drawFRect(drawData: &mut Vec<f64>,rect: &Rect,colour: u8){
    drawData.push(0.0);
    drawData.push(rect.x);
    drawData.push(rect.y);
    drawData.push(rect.w);
    drawData.push(rect.h);
    drawData.push(colour as f64);
}
fn drawSRect(drawData: &mut Vec<f64>, rect: &Rect, colour: u8, thickness: f64){
    drawData.push(1.0);
    drawData.push(rect.x);
    drawData.push(rect.y);
    drawData.push(rect.w);
    drawData.push(rect.h);
    drawData.push(colour as f64);
    drawData.push(thickness);
}
fn drawCircle(drawData: &mut Vec<f64>, circle: &Circle, colour: u8){
    drawData.push(2.0);
    drawData.push(circle.x);
    drawData.push(circle.y);
    drawData.push(circle.r);
    drawData.push(colour as f64);
}
enum Stages{
    Intro,
    Charge,
    Hit,
    End
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
    shield: Circle,
    lance:  Circle,
    horse: Rect,
    bounds: Rect,
    shieldSelected: bool,
    canSwap: bool,
    canAcc: bool,
    horseSpeed: f64,
    score: i32
}
impl Player{ //£Player
    fn new(left: bool) -> Player{
        let mut bounds = BOUND.clone();
        if !left{
            bounds.x += CANVAS_W/2.0
        }
        let x: f64 = bounds.x + BOUND_W/2.0;
        let y: f64 = bounds.y + BOUND_H/2.0;

        let horseX: f64 = if left{TOPBOUND.x} else{TOPBOUND.x+TOPBOUND.w - HORSEW};
        Player {
            left: left,
            shield: Circle { x, y, r: (80.0) },
            lance: Circle { x, y, r: (50.0) },
            horse: Rect { x:horseX, y: TOPBOUND.y, w: HORSEW, h: TOPBOUND.h},
            bounds: bounds,
            shieldSelected:false, 
            canSwap:true,
            canAcc: true,
            horseSpeed: 0.0,
            score: 0
        }
    }
    fn update(&mut self, delta: f64, inputs: &[bool; INPUTAMO]){
        let inpAdd:usize  = if self.left {
            0
        }else{
            INPUTAMO/2
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
            self.shieldSelected = !self.shieldSelected;
            self.canSwap = false;
        }


        if !inputs[5+inpAdd]{
            self.canAcc = true;
        }else if self.canAcc{
            self.horseSpeed += HORSEACC;
            self.canAcc = false;
        }
        self.horse.x += self.horseSpeed * delta * if self.left{1.0} else{-1.0};
        if self.shieldSelected{
            let nextX: f64 = self.shield.x + dx * PLAYERSPEED * delta;
            let nextY: f64 = self.shield.y + dy * PLAYERSPEED * delta;

            if nextX-self.shield.r >= self.bounds.x && nextX+self.shield.r <= self.bounds.x + self.bounds.w{
                self.shield.x = nextX;
            }
            if nextY-self.shield.r >= self.bounds.y && nextY+self.shield.r <= self.bounds.y + self.bounds.h{
                self.shield.y = nextY;
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
        drawCircle(drawData, &self.shield, 2);
        drawCircle(drawData, &self.lance, 1);
        drawFRect(drawData, &self.horse, 1);
    }
    fn drawShadow(&self, drawData: &mut Vec<f64>, dist: f64){
        let mut flipShield = self.shield;
        let mut flipLance = self.lance;

        flipShield.x = CANVAS_W - flipShield.x;
        flipLance.x = CANVAS_W - flipLance.x;

        flipShield.r = getScrR(dist, flipShield.r);
        flipLance.r = getScrR(dist, flipLance.r);

        drawCircle(drawData, &flipShield, 3);
        drawCircle(drawData, &flipLance, 3);
    }
}

#[wasm_bindgen]
pub struct Game{
    player1: Player,
    player2: Player,
    stage: Stages
}
#[wasm_bindgen]
impl Game{ //£Game
    #[wasm_bindgen(constructor)]
    pub fn new() ->Self{
        Game {
            player1: Player::new(true),
            player2: Player::new(false),
            stage: Stages::Charge
        }
    }
    fn Charge(&mut self,delta: f64,inputs: &[bool; INPUTAMO]) -> Vec<f64>{
        self.player1.update(delta,inputs);
        self.player2.update(delta,inputs);

        let horseDist: f64 = self.player2.horse.x - self.player1.horse.x - HORSEW;

        if horseDist <= 0.0{
            self.stage = Stages::Hit;
            let mut flipShield1: Circle = self.player1.shield;
            flipShield1.x = CANVAS_W - flipShield1.x;

            let mut flipShield2: Circle = self.player2.shield;
            flipShield2.x = CANVAS_W - flipShield2.x;

            
            if cirCirColi(self.player1.lance, flipShield2){
                let pointGain: i32 = (self.player1.horseSpeed /HORSEACC) as i32;
                self.player1.score += pointGain;
                if cirCirEngulf(self.player1.lance, flipShield2){
                    self.player1.score += pointGain;
                }
            }
            if cirCirColi(self.player2.lance, flipShield1){
                let pointGain: i32 = (self.player2.horseSpeed /HORSEACC) as i32;
                self.player2.score += pointGain;
                if cirCirEngulf(self.player2.lance, flipShield1){
                    self.player2.score += pointGain;
                }
            }
        }

        let mut drawData: Vec<f64> = vec![]; //£Draw
        drawSRect(&mut drawData, &TOPBOUND, 0, BORDERTHICK);

        self.player1.draw(&mut drawData);
        self.player2.draw(&mut drawData);
        self.player1.drawShadow(&mut drawData,horseDist);
        self.player2.drawShadow(&mut drawData,horseDist);
        
        drawData
    }
    pub fn update(&mut self,delta: f64,inputNum: i32) -> Vec<f64>{
        let mut inputs = [false;INPUTAMO];
        for i in  0..inputs.len(){
            inputs[i] = (inputNum & (1<<i)) != 0
        }

        let drawData = match self.stage{
            Stages::Charge => self.Charge(delta, &inputs),
            _ => self.Charge(delta, &inputs) 
        };
        drawData
    }
}