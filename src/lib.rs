#![allow(non_snake_case)]   

use serde::Serialize;
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;


//£ Consts
const RECROOT2: f32 = 0.7071067812; 
const PLAYERSPEED: f32 = 500.0;
const CANVAS_W: f32 = 1280.0;
const CANVAS_H: f32 = 720.0;
const INPUTAMO: usize = 13;

const BOUND_X: f32 = 50.0;
const BOUND_Y: f32 = 150.0;
const BOUND_W: f32 = CANVAS_W/2.0 - BOUND_X*2.0;
const BOUND_H: f32 = CANVAS_H - BOUND_Y - 50.0;
const BOUND: Rect = Rect{x:BOUND_X, y: BOUND_Y, w: BOUND_W, h: BOUND_H};

const BORDERTHICK: f32 = 5.0;

const TOPBOUND: Rect = Rect{
    x: 25.0,
    y: 25.0,
    w: 1230.0,
    h: 100.0
};
const HORSEW: f32 = 150.0;
const HORSEACC: f32 = 20.0;

const CP_SWITCH_T:f32 = 0.2;
const CP_CLICK_TIME: f32 = 0.5;
const CPSPEED: f32 = 200.0;
const CP_REACT_T: f32 = 0.2;

fn getScrR(dist: f32, r: f32) -> f32{
    (-0.048*dist + r).max(0.001)
}
fn pointDist(x1:f32, y1: f32, x2: f32, y2: f32) -> f32{
    ((x1-x2)*(x1-x2) + (y1-y2)*(y1-y2)).sqrt()
}
fn cirCirColi(a: &Circle, b:&Circle) -> bool{
    return pointDist(a.x, a.y, b.x, b.y) <= a.r + b.r
}
fn cirCirEngulf(s: &Circle, b: &Circle) -> bool{
    return pointDist(s.x, s.y, b.x, b.y) <= b.r-s.r;
}

enum Stages{
    Intro,
    Charge,
    Hit,
    End
}


#[derive(Serialize)]
struct DrawData {
    arr: Vec<f32>,
    strings: Vec<String>
}
fn drawFRect(drawData: &mut DrawData,rect: &Rect,colour: u8){
    drawData.arr.push(0.0);
    drawData.arr.push(rect.x);
    drawData.arr.push(rect.y);
    drawData.arr.push(rect.w);
    drawData.arr.push(rect.h);
    drawData.arr.push(colour as f32);
}
fn drawSRect(drawData: &mut DrawData, rect: &Rect, colour: u8, thickness: f32){
    drawData.arr.push(1.0);
    drawData.arr.push(rect.x);
    drawData.arr.push(rect.y);
    drawData.arr.push(rect.w);
    drawData.arr.push(rect.h);
    drawData.arr.push(colour as f32);
    drawData.arr.push(thickness);
}
fn drawCircle(drawData: &mut DrawData, cir: &Circle, colour: u8){
    drawData.arr.push(2.0);
    drawData.arr.push(cir.x);
    drawData.arr.push(cir.y);
    drawData.arr.push(cir.r);
    drawData.arr.push(colour as f32);
}
fn drawText(drawData: &mut DrawData,text:String, x:f32, y:f32, h: i32, colour: u8, base: u8, align: u8 ){
    drawData.arr.push(3.0);
    drawData.arr.push(drawData.strings.len() as f32);
    drawData.arr.push(x);
    drawData.arr.push(y);
    drawData.arr.push(h as f32);
    drawData.arr.push(colour as f32);
    drawData.arr.push(base as f32);
    drawData.arr.push(align as f32);

    drawData.strings.push(text);
    
}
fn drawImgRect(drawData: &mut DrawData,img:f32 , rect: &Rect){
    drawData.arr.push(4.0);
    drawData.arr.push(img);
    drawData.arr.push(rect.x);
    drawData.arr.push(rect.y);
    drawData.arr.push(rect.w);
    drawData.arr.push(rect.h);
}
fn drawImgCir(drawData: &mut DrawData, img:f32, cir: &Circle){
    drawData.arr.push(4.0);
    drawData.arr.push(img);
    drawData.arr.push(cir.x - cir.r);
    drawData.arr.push(cir.y - cir.r);
    drawData.arr.push(cir.r * 2.0);
    drawData.arr.push(cir.r * 2.0);
}
#[derive(Clone, Copy)]
struct Rect{
    x: f32,
    y: f32,
    w: f32,
    h: f32
}
#[derive(Clone, Copy)]
struct Circle{
    x: f32,
    y: f32,
    r: f32
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
    horseSpeed: f32,
    targetX: f32,
    targetY: f32,
    switchTL: f32,
    cpClickTL: f32,
    cpReactTl: f32,
}
impl Player{ //£Player
    fn new(left: bool) -> Player{
        let mut bounds = BOUND.clone();
        if !left{
            bounds.x += CANVAS_W/2.0
        }
        let x: f32 = bounds.x + BOUND_W/2.0;
        let y: f32 = bounds.y + BOUND_H/2.0;

        let horseX: f32 = if left{TOPBOUND.x} else{TOPBOUND.x+TOPBOUND.w - HORSEW};
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
            targetX: 0.0,
            targetY: 0.0,
            switchTL: 0.0,
            cpClickTL: 0.0,
            cpReactTl: 0.0,
        }
    }
    fn update(&mut self, delta: f32, inputs: &[bool; INPUTAMO]){
        let inpAdd:usize  = if self.left {
            0
        }else{
            INPUTAMO/2
        };

        let mut dx: f32= 0.0;
        let mut dy: f32 = 0.0;

        if inputs[1+inpAdd]{
            dy -= 1.0;
        }if inputs[2+inpAdd]{
            dx -= 1.0;
        }if inputs[3+inpAdd]{
            dy += 1.0;
        }if inputs[4+inpAdd]{
            dx += 1.0;
        }

        if dx.abs() + dy.abs() == 2.0{
            dx *= RECROOT2;
            dy *= RECROOT2;
        }

        if inputs[5+inpAdd]{
            self.canSwap = true;
        }else if self.canSwap{
            self.shieldSelected = !self.shieldSelected;
            self.canSwap = false;
        }


        if !inputs[6+inpAdd]{
            self.canAcc = true;
        }else if self.canAcc{
            self.horseSpeed += HORSEACC;
            self.canAcc = false;
        }
        self.horse.x += self.horseSpeed * delta * if self.left{1.0} else{-1.0};
        if self.shieldSelected{
            let nextX: f32 = self.shield.x + dx * PLAYERSPEED * delta;
            let nextY: f32 = self.shield.y + dy * PLAYERSPEED * delta;

            if nextX-self.shield.r >= self.bounds.x && nextX+self.shield.r <= self.bounds.x + self.bounds.w{
                self.shield.x = nextX;
            }
            if nextY-self.shield.r >= self.bounds.y && nextY+self.shield.r <= self.bounds.y + self.bounds.h{
                self.shield.y = nextY;
            }
            
        }else{
            let nextX: f32 = self.lance.x + dx * PLAYERSPEED * delta;
            let nextY: f32 = self.lance.y + dy * PLAYERSPEED * delta;

            if nextX-self.lance.r >= self.bounds.x && nextX+self.lance.r <= self.bounds.x + self.bounds.w{
                self.lance.x = nextX;
            }
            if nextY-self.lance.r >= self.bounds.y && nextY+self.lance.r <= self.bounds.y + self.bounds.h{
                self.lance.y = nextY;
            }
        }

        
    }
    fn cpUpdate(&mut self, delta: f32, foe: &Player){
        self.cpClickTL -= delta;
        while self.cpClickTL <= 0.0 {
            self.cpClickTL += CP_CLICK_TIME;
            self.horseSpeed += HORSEACC;
        }

        self.horse.x += self.horseSpeed * delta * if self.left{1.0} else{-1.0};

        if self.cpReactTl > 0.0{
            self.cpReactTl -= delta;
            return;
        }

        if self.switchTL <= 0.0{

            self.shieldSelected = !self.shieldSelected;
            if self.shieldSelected{


                self.targetX = if CANVAS_W - foe.lance.x < self.bounds.x + self.bounds.w/2.0{
                    self.bounds.x + self.bounds.w - self.shield.r 
                }else{
                    self.bounds.x + self.shield.r
                };

                self.targetY = if foe.lance.y < self.bounds.y + self.bounds.h/2.0{
                    self.bounds.y + self.bounds.y - self.shield.r 
                }else{
                    self.bounds.y + self.shield.r
                };
                
            }else{
                self.targetX = CANVAS_W - foe.shield.x;
                self.targetY = foe.shield.y;
            }
            self.switchTL = CP_SWITCH_T;
            self.cpReactTl = CP_REACT_T;
        }else{
            self.switchTL -= delta;
            if self.shieldSelected{
                if pointDist(self.shield.x, self.shield.y, self.targetX, self.targetY) <= CPSPEED * 0.5 *delta{
                    self.shield.x = self.targetX;
                    self.shield.y = self.targetY;
                }else{
                    let angle = (self.targetY-self.shield.y).atan2(self.targetX-self.shield.x);
                    self.shield.x += angle.cos() * CPSPEED * delta;
                    self.shield.y += angle.sin() * CPSPEED * delta;
                }
    
            }else{
                if pointDist(self.lance.x, self.lance.y, self.targetX, self.targetY) <= CPSPEED * 0.5 *delta{
                    self.lance.x = self.targetX;
                    self.lance.y = self.targetY;
                }else{
                    let angle = (self.targetY-self.lance.y).atan2(self.targetX-self.lance.x);
                    self.lance.x += angle.cos() * CPSPEED * delta;
                    self.lance.y += angle.sin() * CPSPEED * delta;
                }
                
            }
            

        }

    }
    fn hit(&mut self,flipShield: Circle )-> i32{
        let mut scoreGain = 0;
        if cirCirColi(&self.lance, &flipShield){
            scoreGain = (self.horseSpeed /HORSEACC) as i32;
            if cirCirEngulf(&self.lance, &flipShield){
                scoreGain *= 2
            }
        }
        scoreGain
    }
    fn draw(&self, drawData: &mut DrawData){
        drawSRect(drawData, &self.bounds, 0, BORDERTHICK);

        drawImgRect(drawData, 0.0, &self.horse);
        drawImgCir(drawData, 2.0, &self.shield);
        drawImgCir(drawData, 1.0, &self.lance);
    }
    fn drawShadow(&self, drawData: &mut DrawData, dist: f32){
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
    stage: Stages,
    timeLeft: f32,
    round: u8,
    score1: i32,
    score2: i32,
    scoreGain1: i32,
    scoreGain2: i32,
    isCp: bool
}
#[wasm_bindgen]
impl Game{ //£Game
    #[wasm_bindgen(constructor)]
    pub fn new() ->Self{
        Game {
            player1: Player::new(true),
            player2: Player::new(false),
            stage: Stages::Intro,
            timeLeft: 0.0,
            round: 1,
            score1: 0,
            score2: 0,
            scoreGain1: 0,
            scoreGain2: 0,
            isCp: false
        }
    }
    fn intro(&mut self,drawData: &mut DrawData, inputs: &[bool; INPUTAMO]){
        drawFRect(drawData, & Rect { x: CANVAS_W/2.0, y: CANVAS_H/2.0, w: 100.0, h: 100.0 }, 1);
        if inputs[6] || inputs[5]{
            self.isCp = inputs[5];
            self.stage = Stages::Charge
        }
    }
    fn charge(&mut self,drawData: &mut DrawData, delta: f32,inputs: &[bool; INPUTAMO]){
        self.player1.update(delta,inputs);
        if self.isCp{
            self.player2.cpUpdate(delta, &self.player1);
        }else{
            self.player2.update(delta,inputs);
        }
        

        let horseDist: f32 = self.player2.horse.x - self.player1.horse.x - HORSEW;



        drawSRect(drawData, &TOPBOUND, 0, BORDERTHICK);
        
        self.player1.draw(drawData);
        self.player2.draw(drawData);
        self.player1.drawShadow(drawData,horseDist);
        self.player2.drawShadow(drawData,horseDist);

        drawText(drawData, self.round.to_string(), CANVAS_W/2.0, CANVAS_H, 50, 0, 2, 1);
        drawText(drawData, self.score1.to_string(), 0.0, CANVAS_H, 50, 0, 2, 0);
        drawText(drawData, self.score2.to_string(), CANVAS_W, CANVAS_H, 50, 0, 2, 2);
        
        if horseDist <= 0.0{ //Start hit
            self.stage = Stages::Hit;
            self.timeLeft = 2.0;
            self.round += 1;

            let mut flipShield1: Circle = self.player1.shield;
            flipShield1.x = CANVAS_W - flipShield1.x;

            let mut flipShield2: Circle = self.player2.shield;
            flipShield2.x = CANVAS_W - flipShield2.x;

            
            self.scoreGain1 = self.player1.hit(flipShield2);
            self.scoreGain2 = self.player2.hit(flipShield1);

            self.score1 += self.scoreGain1;
            self.score2 += self.scoreGain2;

        }
    }
    fn hit(&mut self,drawData: &mut DrawData, delta: f32){
        self.timeLeft -= delta;
        if self.timeLeft <= 0.0{
            if self.round > 3 && self.score1 != self.score2{
                self.stage = Stages::End
            }else{
                self.stage = Stages::Charge;
                self.player1 = Player::new(true);
                self.player2 = Player::new(false);
            }
            return;
        }
        
        const TEXT_H: i32 = 50;

        let centerX1 = self.player1.bounds.x + self.player1.bounds.w /2.0;
        drawText(drawData, "Player 1".to_owned(), centerX1, CANVAS_H/2.0-(TEXT_H as f32), TEXT_H, 0, 1, 1);
        drawText(drawData, format!("Score: {}",self.score1), centerX1, CANVAS_H/2.0, TEXT_H, 0, 1, 1);
        drawText(drawData, format!("Gain: {}",self.scoreGain1), centerX1, CANVAS_H/2.0+(TEXT_H as f32), TEXT_H, 0, 1, 1);

        let centerX2 = self.player2.bounds.x + self.player2.bounds.w /2.0;
        drawText(drawData, "Player 2".to_owned(), centerX2, CANVAS_H/2.0-(TEXT_H as f32), TEXT_H, 0, 1, 1);
        drawText(drawData, format!("Score: {}",self.score2), centerX2, CANVAS_H/2.0, TEXT_H, 0, 1, 1);
        drawText(drawData, format!("Gain: {}",self.scoreGain2), centerX2, CANVAS_H/2.0+(TEXT_H as f32), TEXT_H, 0, 1, 1);

        drawText(drawData, ((self.timeLeft*10.0).round()/10.0).to_string(), CANVAS_W/2.0, CANVAS_H/2.0, 50, 0, 1, 1);

    }
    fn end(&mut self,drawData: &mut DrawData,anyInp: bool){
        const TEXT_H:i32 = 80;
        drawText(drawData, format!("Player 1: {}",self.score1), CANVAS_W/2.0,CANVAS_H/2.0 -TEXT_H as f32 ,TEXT_H, 0, 1, 1);
        drawText(drawData, format!("Player 2: {}",self.score2), CANVAS_W/2.0,CANVAS_H/2.0,TEXT_H, 0, 1, 1);

        let wonText = if self.score1 > self.score2{"Player 1 Wins"} 
        else if self.score2 > self.score1{"Player 2 wins"}
        else {"Draw"}.to_owned();

        drawText(drawData, wonText, CANVAS_W/2.0,CANVAS_H/2.0 +TEXT_H as f32 ,TEXT_H, 0, 1, 1);

        if anyInp{
            self.stage = Stages::Intro;
            self.score1 = 0;
            self.score2 = 0;
            self.round = 1;
            self.player1 = Player::new(true);
            self.player2 = Player::new(false);
        }
    }
    pub fn update(&mut self,delta: f32,inputNum: i32) -> JsValue{
        let mut inputs = [false;INPUTAMO];
        for i in  0..inputs.len(){
            inputs[i] = (inputNum & (1<<i)) != 0
        }
        let mut drawData = DrawData{ arr: vec![], strings: vec![]};
        match self.stage{
            Stages::Intro => self.intro(&mut drawData, &inputs),
            Stages::Charge => self.charge(&mut drawData ,delta, &inputs),
            Stages::Hit => self.hit(&mut drawData, delta),
            Stages::End => self.end(&mut drawData,inputs[0]),
            _ => self.charge(&mut drawData,delta, &inputs) 
        };
        to_value(&drawData).unwrap()
    }
}