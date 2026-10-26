import init, { Game } from "./pkg/joust.js";
await init();
/** @type {HTMLCanvasElement} */
const canvas = document.getElementById("canvas1") //£: Top
/** @type {CanvasRenderingContext2D} */
const ctx = canvas.getContext("2d")

const inputBox = document.getElementById("inp")
inputBox.style.display = "none"
const label = document.getElementById("label")


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

}
window.addEventListener("resize",resizeCanvas);
resizeCanvas()

let previousTime = 0;
const isKeyDown = {};

let promptInp = ""

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
document.addEventListener("submit", (e) =>{
    e.preventDefault()
    promptInp = inputBox.value.trim()

    if (promptInp === "") promptInp = -2
    else {
        promptInp = +promptInp
        if (isNaN(promptInp)) promptInp = -1
        else {
            inputBox.style.display = "none"
            label.innerHTML = ""
            inputBox.value = ""
        }
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
    [0,0,1280,720], //bg 0
    [160,2161,150,100], //horseIn 1
    [410,2161,150,99], //horseIn2 2
    [860,2161,150,97], //horseOut 3
    [1010,2161,150,97], //horseOut2 4
    [560,2161,150,99], //horseStill 5
    [710,2161,150,99], //horseStill2 6
    [0,720,1280,720], //intro 7
    [310,2161,100,100], //lance 8
    [0,2161,160,160], //shield 9
    [0,1440,1280,720], //result 10
]
const AUDIOS = [
    new Audio("assets/audio/lanceBreak.mp3"),
    new Audio("assets/audio/horse.mp3"),
    new Audio("assets/audio/horse.mp3"),
    new Audio("assets/audio/fanfare.mp3")
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
function drawImg(arr,i){
    const sImg = S_IMGS[arr[i+1]]
    ctx.drawImage(ATLAS,
        sImg[0], sImg[1], sImg[2], sImg[3],
        arr[i+2],arr[i+3],arr[i+4],arr[i+5]
    )
    return i+6
}

function drawText(arr,i,strings){
    ctx.font = `${arr[i+4]}px Metamorphous`;
    ctx.fillStyle = COLOURS[arr[i+5]];

    ctx.textBaseline = TEXTBASES[arr[i+6]]
    ctx.textAlign = TEXTALIGNS[arr[i+7]]

    ctx.fillText(strings[arr[i+1]],arr[i+2],arr[i+3]);
    return i+8
}

function drawPrompt(arr,i,strings){
    inputBox.style.display = ""
    label.innerHTML = strings[arr[i+1]]
    return i+2
}
function playAudio(arr,i){
    AUDIOS[arr[i+1]].currentTime = 0
    AUDIOS[arr[i+1]].play()

    return i+2
}
function stopAudio(arr,i){
    AUDIOS[arr[i+1]].pause()

    return i+2
}
const outputFuncs = [drawFCircle,drawImg,drawText,drawPrompt,playAudio,stopAudio]
function draw(outputData){
    let i = 0
    while (i<outputData.arr.length){
        const func = outputFuncs[outputData.arr[i]]

        i = func(outputData.arr,i,outputData.strings)
    }
    
   
}
let game = new Game()
function animate(timestamp) { //£ animate
    const delta = (timestamp - previousTime) / 1000;
    previousTime = timestamp
    ctx.clearRect(0, 0, canvasWidth, canvasHeight);
    
    
    const outputData = game.update(delta,getInput(),promptInp);
    draw(outputData);

    promptInp = -2
    requestAnimationFrame(animate);
}
requestAnimationFrame(animate);
