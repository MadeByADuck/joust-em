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
    "blue",
    "green"
]
const INPUTLIST = [
    "w",
    "a",
    "s",
    "d"
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
function draw(drawData){
    for (let i=0;i<drawData.length; i += 5){
        ctx.fillStyle = colours[drawData[i+4]]
        ctx.fillRect(
            drawData[i],
            drawData[i+1],
            drawData[i+2],
            drawData[i+3]
        )
    }
}
let game = new Game(0,canvasWidth/2,canvasHeight/2)

function animate(timestamp) { //£ animate
    const delta = (timestamp - previousTime) / 1000;
    previousTime = timestamp
    ctx.clearRect(0, 0, canvasWidth, canvasHeight);


    const drawData = game.update(delta,getInput())
    draw(drawData)

    requestAnimationFrame(animate);
}
requestAnimationFrame(animate);
