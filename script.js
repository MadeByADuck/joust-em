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
const ATLAS = new Image()
ATLAS.src = "assets/atlas.png"
const S_IMGS = [
    [0,0,283,189], //horse
    [283,0,189,189], //lance
    [472,0,151,151], //shield
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
function drawFRect(arr,i){
    ctx.fillStyle = COLOURS[arr[i+5]];
    ctx.fillRect(
        arr[i+1],
        arr[i+2],
        arr[i+3],
        arr[i+4]
    )
    return i+6
}
function drawSRect(arr,i){
    ctx.strokeStyle = COLOURS[arr[i+5]];
    ctx.lineWidth = arr[i+6];
    ctx.strokeRect(
        arr[i+1],
        arr[i+2],
        arr[i+3],
        arr[i+4]
    )
    return i+7
}
function drawFCircle(arr,i){
    ctx.beginPath()
    ctx.arc(
        arr[i+1],
        arr[i+2],
        arr[i+3],
        0,
        2*Math.PI
    )
    ctx.fillStyle = COLOURS[arr[i+4]]
    ctx.fill()
    return i+5
}

function drawText(arr,i,strings){
    ctx.font = `${arr[i+4]}px Arial`;
    ctx.fillStyle = COLOURS[arr[i+5]];

    ctx.textBaseline = TEXTBASES[arr[i+6]]
    ctx.textAlign = TEXTALIGNS[arr[i+7]]

    ctx.fillText(strings[arr[i+1]],arr[i+2],arr[i+3]);
    return i+8
}
function drawImg(arr,i){
    const sImg = S_IMGS[arr[i+1]]
    ctx.drawImage(ATLAS,
        sImg[0], sImg[1], sImg[2], sImg[3],
        arr[i+2],arr[i+3],arr[i+4],arr[i+5]
    )
    return i+6
}
function draw(drawData){
    let i = 0
    while (i<drawData.arr.length){
        switch (drawData.arr[i]){
            case 0:
                i=drawFRect(drawData.arr,i)
                break
            case 1:
                i=drawSRect(drawData.arr,i)
                break
            case 2: 
                i=drawFCircle(drawData.arr,i)
                break
            case 3:
                i=drawText(drawData.arr,i,drawData.strings)
                break
            case 4:
                i=drawImg(drawData.arr,i)
                break
            default:
                console.log("Wrong i")

        }

    }
    
   
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
