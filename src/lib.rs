#![allow(non_snake_case)]   

use serde::Serialize;
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;


//£ Consts
const RECROOT2: f32 = 0.7071067812; 
const PLAYERSPEED: f32 = 500.0;

const CANVAS: Rect = Rect{x:0.0, y:0.0, w: 1280.0, h: 720.0};
const INPUTAMO: usize = 13;

const BOUND_X: f32 = 50.0;
const BOUND_Y: f32 = 150.0;
const BOUND_W: f32 = CANVAS.w/2.0 - BOUND_X*2.0;
const BOUND_H: f32 = CANVAS.h - BOUND_Y - 50.0;
const BOUND: Rect = Rect{x:BOUND_X, y: BOUND_Y, w: BOUND_W, h: BOUND_H}; 

const TOPBOUND: Rect = Rect{
    x: 25.0,
    y: 25.0,
    w: 1230.0,
    h: 100.0
};
const HORSEW: f32 = 150.0;
const HORSEACC: f32 = 20.0;

const HORSE_ANI1: [f32; 3] = [
    1.0, 3.0, 5.0
];
const HORSE_ANI2: [f32; 3] = [
    2.0, 4.0, 6.0
];
const LANCE_IMG: f32 = 8.0;
const SHIELD_IMG: f32 = 9.0;

const BG_IMG: f32 = 0.0;
const INTRO_IMG: f32 = 7.0;
const RESULT_IMG: f32 = 10.0;

const CRASH_AU: f32 = 0.0;
const HORSE1_AU: f32 = 1.0;
const HORSE2_AU: f32 = 2.0;
const FANFARE_AU: f32 = 3.0;

const CP_SWITCH_T:f32 = 0.2;

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
#[derive(PartialEq, Eq)]
enum IntroStages{
    First,
    Click,
    Speed,
    Delay
}

#[derive(Serialize)]
struct OutputData {
    arr: Vec<f32>,
    strings: Vec<String>
}


fn drawCircle(outputData: &mut OutputData, cir: &Circle, colour: u8){
    outputData.arr.push(0.0);
    outputData.arr.push(cir.x);
    outputData.arr.push(cir.y);
    outputData.arr.push(cir.r);
    outputData.arr.push(colour as f32);
}

fn drawImgRect(outputData: &mut OutputData,img:f32 , rect: &Rect){
    outputData.arr.push(1.0);
    outputData.arr.push(img);
    outputData.arr.push(rect.x);
    outputData.arr.push(rect.y);
    outputData.arr.push(rect.w);
    outputData.arr.push(rect.h);
}
fn drawImgCir(outputData: &mut OutputData, img:f32, cir: &Circle){
    outputData.arr.push(1.0);
    outputData.arr.push(img);
    outputData.arr.push(cir.x - cir.r);
    outputData.arr.push(cir.y - cir.r);
    outputData.arr.push(cir.r * 2.0);
    outputData.arr.push(cir.r * 2.0);
}
fn drawText(outputData: &mut OutputData,text:String, x:f32, y:f32, h: i32, colour: u8, base: u8, align: u8 ){
    outputData.arr.push(2.0);
    outputData.arr.push(outputData.strings.len() as f32);
    outputData.arr.push(x);
    outputData.arr.push(y);
    outputData.arr.push(h as f32);
    outputData.arr.push(colour as f32);
    outputData.arr.push(base as f32);
    outputData.arr.push(align as f32);

    outputData.strings.push(text);
}
fn drawPrompt(outputData: &mut OutputData, text: &String){
    outputData.arr.push(3.0);
    outputData.arr.push(outputData.strings.len() as f32);

    outputData.strings.push(text.to_owned()); 
}

fn makeSound(outputData: &mut OutputData, sound: f32){
    outputData.arr.push(4.0);
    outputData.arr.push(sound)
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
    horseFrame: usize,
    hFrameTimeLeft: f32,
}
impl Player{ //£Player
    fn new(left: bool) -> Player{
        let mut bounds = BOUND.clone();
        if !left{
            bounds.x += CANVAS.w/2.0
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
            canSwap:false,
            canAcc: false,
            horseSpeed: 0.0,
            targetX: 0.0,
            targetY: 0.0,
            switchTL: 0.0,
            cpClickTL: 0.0,
            cpReactTl: 0.0,
            horseFrame: 2,
            hFrameTimeLeft: 0.0,
        }
    }
    fn update(&mut self,delta: f32, inputs: &[bool; INPUTAMO]){
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
    fn cpUpdate(&mut self, delta: f32, foe: &Player, clickTime: f32, speed: f32, delay: f32){
        self.cpClickTL -= delta;
        while self.cpClickTL <= 0.0 {
            self.cpClickTL += clickTime;
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


                self.targetX = if CANVAS.w - foe.lance.x < self.bounds.x + self.bounds.w/2.0{
                    self.bounds.x + self.bounds.w - self.shield.r 
                }else{
                    self.bounds.x + self.shield.r
                };

                self.targetY = if foe.lance.y < self.bounds.y + self.bounds.h/2.0{
                    self.bounds.y + self.bounds.h - self.shield.r 
                }else{
                    self.bounds.y + self.shield.r
                };
                
            }else{
                self.targetX = CANVAS.w - foe.shield.x;
                self.targetY = foe.shield.y;
            }
            self.switchTL = CP_SWITCH_T;
            self.cpReactTl = delay;
        }else{
            self.switchTL -= delta;
            if self.shieldSelected{
                if pointDist(self.shield.x, self.shield.y, self.targetX, self.targetY) <= speed * 0.5 *delta{
                    self.shield.x = self.targetX;
                    self.shield.y = self.targetY;
                }else{
                    let angle = (self.targetY-self.shield.y).atan2(self.targetX-self.shield.x);
                    self.shield.x += angle.cos() * speed * delta;
                    self.shield.y += angle.sin() * speed * delta;
                }
    
            }else{
                if pointDist(self.lance.x, self.lance.y, self.targetX, self.targetY) <= speed * 0.5 *delta{
                    self.lance.x = self.targetX;
                    self.lance.y = self.targetY;
                }else{
                    let angle = (self.targetY-self.lance.y).atan2(self.targetX-self.lance.x);
                    self.lance.x += angle.cos() * speed * delta;
                    self.lance.y += angle.sin() * speed * delta;
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
    fn draw(&mut self, outputData: &mut OutputData,delta: f32){
        if self.horseSpeed>0.0{
            if self.hFrameTimeLeft > 0.0{
                self.hFrameTimeLeft -= delta * self.horseSpeed;
            }else{
                if self.horseFrame == 1{
                    makeSound(outputData, if self.left{HORSE1_AU} else {HORSE2_AU});
                }
                self.horseFrame = (self.horseFrame + 1) % 2;
                self.hFrameTimeLeft = 20.0;
            }
        }

        let horseFrame = if self.left{
            HORSE_ANI1[self.horseFrame]
        }else{
            HORSE_ANI2[self.horseFrame]
        };

        drawImgRect(outputData, horseFrame, &self.horse);
        drawImgCir(outputData, SHIELD_IMG, &self.shield);
        drawImgCir(outputData, LANCE_IMG, &self.lance);
    }
    fn drawShadow(&self, outputData: &mut OutputData, dist: f32){
        let mut flipShield = self.shield;
        let mut flipLance = self.lance;

        flipShield.x = CANVAS.w - flipShield.x;
        flipLance.x = CANVAS.w - flipLance.x;

        flipShield.r = getScrR(dist, flipShield.r);
        flipLance.r = getScrR(dist, flipLance.r);

        drawCircle(outputData, &flipShield, 3);
        drawCircle(outputData, &flipLance, 3);
    }
}

#[wasm_bindgen]
pub struct Game{
    player1: Player,
    player2: Player,
    stage: Stages,
    introStage: IntroStages,
    timeLeft: f32,
    round: u8,
    score1: i32,
    score2: i32,
    scoreGain1: i32,
    scoreGain2: i32,
    isCp: bool,
    promptMes: String,
    cpClick: f32,
    cpSpeed: f32,
    cpDelay: f32

}
#[wasm_bindgen]
impl Game{ //£Game
    #[wasm_bindgen(constructor)]
    pub fn new() ->Self{
        Game {
            player1: Player::new(true),
            player2: Player::new(false),
            stage: Stages::Intro,
            introStage: IntroStages::First,
            timeLeft: 0.0,
            round: 1,
            score1: 0,
            score2: 0,
            scoreGain1: 0,
            scoreGain2: 0,
            isCp: false,
            promptMes: "".to_owned(),
            cpClick: 5.0,
            cpDelay: 0.2,
            cpSpeed: 200.0
        }
    }
    fn intro(&mut self,outputData: &mut OutputData, inputs: &[bool; INPUTAMO],promptInp: f32){
        drawImgRect(outputData, INTRO_IMG, &CANVAS);

        if self.introStage == IntroStages::First{
            if inputs[6]{
                self.stage = Stages::Charge;
                self.isCp = false;
            }else if inputs[5]{
                self.isCp = true;
                self.introStage = IntroStages::Click;
                self.promptMes = "Ai Click Speed? ".to_owned();
                drawPrompt(outputData, &self.promptMes);
            }else if inputs[1]{
                self.isCp = true;
                self.stage = Stages::Charge;
                self.cpClick = 0.5;
                self.cpDelay = 0.2;
                self.cpSpeed = 200.0;

            }
        }else{
            if promptInp != -2.0{
                if promptInp < 0.0 {
                    drawPrompt(outputData, &self.promptMes);
                }else{
                    match self.introStage{
                        IntroStages::Click => {
                            self.cpClick = 1.0/promptInp;
                            self.introStage = IntroStages::Speed;
                            self.promptMes = "Cp Speed?".to_owned();
                            drawPrompt(outputData, &self.promptMes);
                        }
                        IntroStages::Speed =>{
                            self.cpSpeed = promptInp;
                            self.introStage = IntroStages::Delay;
                            self.promptMes = "Cp delay?".to_owned();
                            drawPrompt(outputData, &self.promptMes);
                        }
                        IntroStages::Delay =>{
                            self.cpDelay = promptInp;
                            self.stage = Stages::Charge;
                        }
                        _ => {}
                        
                    }
                }
            }
        }


    }
    fn charge(&mut self,outputData: &mut OutputData, delta: f32,inputs: &[bool; INPUTAMO]){
        self.player1.update(delta,inputs);
        if self.isCp{
            self.player2.cpUpdate(delta, &self.player1,self.cpClick,self.cpSpeed,self.cpDelay);
        }else{
            self.player2.update(delta,inputs);
        }

        let horseDist: f32 = self.player2.horse.x - self.player1.horse.x - HORSEW;

        drawImgRect(outputData, BG_IMG, &CANVAS);
        
        self.player1.draw(outputData,delta);
        self.player2.draw(outputData,delta);
        self.player1.drawShadow(outputData,horseDist);
        self.player2.drawShadow(outputData,horseDist);

        drawText(outputData, self.round.to_string(), CANVAS.w/2.0, CANVAS.h, 50, 0, 2, 1);
        drawText(outputData, self.score1.to_string(), 0.0, CANVAS.h, 50, 0, 2, 0);
        drawText(outputData, self.score2.to_string(), CANVAS.w, CANVAS.h, 50, 0, 2, 2);
        
        if horseDist <= 0.0{ //Start hit
            

            self.stage = Stages::Hit;
            self.timeLeft = 2.0;
            self.round += 1;

            let mut flipShield1: Circle = self.player1.shield;
            flipShield1.x = CANVAS.w - flipShield1.x;

            let mut flipShield2: Circle = self.player2.shield;
            flipShield2.x = CANVAS.w - flipShield2.x;

            
            self.scoreGain1 = self.player1.hit(flipShield2);
            self.scoreGain2 = self.player2.hit(flipShield1);

            self.score1 += self.scoreGain1;
            self.score2 += self.scoreGain2;

            if self.scoreGain1 != 0 || self.scoreGain2 != 0{
                makeSound(outputData, CRASH_AU);
            };

        }
    }
    fn hit(&mut self,outputData: &mut OutputData, delta: f32){
        drawImgRect(outputData, BG_IMG, &CANVAS);

        self.timeLeft -= delta;
        if self.timeLeft <= 0.0{
            if self.round > 3 && self.score1 != self.score2{
                makeSound(outputData, FANFARE_AU);
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
        drawText(outputData, "Player 1".to_owned(), centerX1, CANVAS.h/2.0-(TEXT_H as f32), TEXT_H, 0, 1, 1);
        drawText(outputData, format!("Score: {}",self.score1), centerX1, CANVAS.h/2.0, TEXT_H, 0, 1, 1);
        drawText(outputData, format!("Gain: {}",self.scoreGain1), centerX1, CANVAS.h/2.0+(TEXT_H as f32), TEXT_H, 0, 1, 1);

        let centerX2 = self.player2.bounds.x + self.player2.bounds.w /2.0;
        drawText(outputData, "Player 2".to_owned(), centerX2, CANVAS.h/2.0-(TEXT_H as f32), TEXT_H, 0, 1, 1);
        drawText(outputData, format!("Score: {}",self.score2), centerX2, CANVAS.h/2.0, TEXT_H, 0, 1, 1);
        drawText(outputData, format!("Gain: {}",self.scoreGain2), centerX2, CANVAS.h/2.0+(TEXT_H as f32), TEXT_H, 0, 1, 1);

        drawText(outputData, ((self.timeLeft*10.0).round()/10.0).to_string(), CANVAS.w/2.0, CANVAS.h/2.0, 50, 0, 1, 1);

    }
    fn end(&mut self,outputData: &mut OutputData,anyInp: bool){
        drawImgRect(outputData, RESULT_IMG, &CANVAS);

        const TEXT_H:i32 = 80;
        drawText(outputData, format!("Player 1: {}",self.score1), CANVAS.w/2.0,CANVAS.h/2.0 -TEXT_H as f32 ,TEXT_H, 0, 1, 1);
        drawText(outputData, format!("Player 2: {}",self.score2), CANVAS.w/2.0,CANVAS.h/2.0,TEXT_H, 0, 1, 1);

        let wonText = if self.score1 > self.score2{"Player 1 Wins"} 
        else if self.score2 > self.score1{"Player 2 wins"}
        else {"Draw"}.to_owned();

        drawText(outputData, wonText, CANVAS.w/2.0,CANVAS.h/2.0 +TEXT_H as f32 ,TEXT_H, 0, 1, 1);

        if anyInp{
            self.stage = Stages::Intro;
            self.score1 = 0;
            self.score2 = 0;
            self.round = 1;
            self.player1 = Player::new(true);
            self.player2 = Player::new(false);
        }
    }
    pub fn update(&mut self,delta: f32,inputNum: i32, promptInp: f32) -> JsValue{
        let mut inputs = [false;INPUTAMO];
        for i in  0..inputs.len(){
            inputs[i] = (inputNum & (1<<i)) != 0
        }
        let mut outputData = OutputData{ arr: vec![], strings: vec![]};
        match self.stage{
            Stages::Intro => self.intro(&mut outputData, &inputs,promptInp),
            Stages::Charge => self.charge(&mut outputData ,delta, &inputs),
            Stages::Hit => self.hit(&mut outputData, delta),
            Stages::End => self.end(&mut outputData,inputs[0])
        };
        to_value(&outputData).unwrap()
    }
}