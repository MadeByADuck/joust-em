import init, { Game } from "./pkg/rust_test.js";
await init();
/** @type {HTMLCanvasElement} */
const canvas = document.getElementById("canvas1") //£: Top
/** @type {CanvasRenderingContext2D} */
const ctx = canvas.getContext("2d")

const canvasWidth = 1280;
const canvasHeight = 720;
let screenScale;
function resizeCanvas(){
    screenScale = Math.min(window.innerWidth / canvasWidth,window.innerHeight/canvasHeight);
    canvas.width = canvasWidth * screenScale;
    canvas.height = canvasHeight * screenScale;
    canvas.style.width = canvas.width + "px";
    canvas.style.height = canvas.height + "px";
    ctx.setTransform(screenScale,0,0,screenScale,0,0)
    ctx.imageSmoothingEnabled = false //if Pixel Art
}
window.addEventListener("resize",resizeCanvas);
resizeCanvas()

let previousTime = 0;
const isKeyDown = {};

let mouseX = canvasWidth / 2;
let mouseY = canvasHeight / 2;
let mouseDown = false;

document.addEventListener("keydown", (event) => {
    inputs[0] = true;
    const index = INPUTLIST.indexOf(event.key.toLowerCase())
    if (index !== -1){
        inputs[index] = true;
    }
});
document.addEventListener("keyup", (event) => {
    inputs[0] = false;
    const index = INPUTLIST.indexOf(event.key.toLowerCase())
    if (index !== -1){
        inputs[index] = false;
    }
})

const COLOURS = [ //Consts
    "black",
    "blue",
    "green",
    "rgba(0,0,0,.5)"
]
const INPUTLIST = [ 
    "all",
    "w","a","s","d","e","q",
    "i","j","k","l","o","u"
]
const TEXTBASES = [
    "top",
    "middle",
    "bottom"
]
const TEXTALIGNS = [
    "left",
    "center",
    "right"
]
let inputs = Array(INPUTLIST.length).fill(false);
function getInput(){
    let inputNum = 0
    for (let i=0; i<INPUTLIST.length; i++){
        if (inputs[i]){
            inputNum |= 1 << i
        }
    }
    return inputNum
}
function drawHud(data){
    if (data.stage !== 1 && data.stage !== 2 ) return
    ctx.textBaseline = "bottom";
    ctx.font = "50px Arial";
    ctx.fillStyle = "black";

    ctx.textAlign = "center";
    ctx.fillText(data.round,canvasWidth/2,canvasHeight);

    ctx.textAlign = "left";
    ctx.fillText(`Score: ${data.p1Score}`,0,canvasHeight);

    ctx.textAlign = "right";
    ctx.fillText(`Score: ${data.p2Score}`,canvasWidth,canvasHeight);

}
function drawFRect(rect){
    ctx.fillStyle = COLOURS[rect.colour];
    ctx.fillRect(
        rect.x,
        rect.y,
        rect.w,
        rect.h
    )
}
function drawSRect(sRect){
    ctx.strokeStyle = COLOURS[sRect.colour];
    ctx.lineWidth = sRect.thickness;
    ctx.strokeRect(
        sRect.x,
        sRect.y,
        sRect.w,
        sRect.h
    )
}
function drawFCircle(cir){
    ctx.beginPath()
    ctx.arc(
        cir.x,
        cir.y,
        cir.r,
        0,
        2*Math.PI
    )
    ctx.fillStyle = COLOURS[cir.colour]
    ctx.fill()
}

function drawText(t){
    ctx.font = `${t.h}px Arial`;
    ctx.fillStyle = COLOURS[t.colour];

    ctx.textBaseline = TEXTBASES[t.base]
    ctx.textAlign = TEXTALIGNS[t.align]

    ctx.fillText(t.text,t.x,t.y);
}
function draw(drawData){
    drawData.rects.forEach(i => drawFRect(i));
    drawData.sRects.forEach(i => drawSRect(i));
    drawData.circles.forEach(i => drawFCircle(i));
    drawData.texts.forEach(i => drawText(i))

}
let game = new Game()
function animate(timestamp) { //£ animate
    const delta = (timestamp - previousTime) / 1000;
    previousTime = timestamp
    ctx.clearRect(0, 0, canvasWidth, canvasHeight);

    const drawData = game.update(delta,getInput());
    draw(drawData);

    //if (data.stage === 1)drawHud(data)
    requestAnimationFrame(animate);
}
requestAnimationFrame(animate);
