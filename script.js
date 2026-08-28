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
    isKeyDown[event.key.toLowerCase()] = true;
});
document.addEventListener("keyup", (event) => {
    isKeyDown[event.key.toLowerCase()] = false;
})
function getMousePos(event) {
    const rect = canvas.getBoundingClientRect();
    mouseX = (event.clientX - rect.left)/screenScale;
    mouseY = (event.clientY - rect.top)/screenScale;
}
canvas.addEventListener("mousemove", function (e) {
    getMousePos(e)
})
canvas.addEventListener("mousedown", function (e) {
    mouseDown = true;
})
canvas.addEventListener("mouseup", function (e) {
    mouseDown = false;
})

const colours = [
    "black",
    "blue",
    "green",
    "rgba(0,0,0,.5)"
]
const INPUTLIST = [
    "w","a","s","d","e","q",
    "i","j","k","l","o","u"
]
function getInput(){
    let input = 0
    for (let i=0; i<INPUTLIST.length; i++){
        if (isKeyDown[INPUTLIST[i]]){
            input |= 1 << i
        }
    }
    return input
}
function drawFRect(drawData,i){//[type,x,y,w,h,colour]
    ctx.fillStyle = colours[drawData[i+5]]
    ctx.fillRect(
        drawData[i+1],
        drawData[i+2],
        drawData[i+3],
        drawData[i+4]
    )
    return i+6
}
function drawSRect(drawData,i){//[type,x,y,w,h,colour,thickness]
    ctx.strokeStyle = colours[drawData[i+5]]
    ctx.lineWidth = drawData[i+6]
    ctx.strokeRect(
        drawData[i+1],
        drawData[i+2],
        drawData[i+3],
        drawData[i+4]
    )
    return i+7
}
function drawFCircle(drawData,i){//[type,x,y,r,colour]
    ctx.beginPath()
    ctx.arc(
        drawData[i+1],
        drawData[i+2],
        drawData[i+3],
        0,
        2*Math.PI
    )
    ctx.fillStyle = colours[drawData[i+4]]
    ctx.fill()
    
    return i+5
}
const drawFuncs = [ //[fillRect;strokeRect;circle]
    drawFRect,
    drawSRect,
    drawFCircle
]
function draw(drawData){
    let i = 0
    while (i<drawData.length){
        i = drawFuncs[drawData[i]](drawData,i)
    }
}
let game = new Game()

function animate(timestamp) { //£ animate
    const delta = (timestamp - previousTime) / 1000;
    previousTime = timestamp
    ctx.clearRect(0, 0, canvasWidth, canvasHeight);


    const drawData = game.update(delta,getInput());
    draw(drawData);

    requestAnimationFrame(animate);
}
requestAnimationFrame(animate);
