import { WasmApp } from 'closest-pair-wasm';
import { drawCircle } from './present';

let app = new WasmApp();
let calc = document.getElementById('calc');
let canvas = document.querySelector('canvas');
let clear = document.getElementById('clear');
let distance = document.getElementById('dist');

function getClickPos(e) {
    let canSty = getComputedStyle(canvas);
    let topBorder = parseInt(canSty.getPropertyValue('border-left-width'));
    let leftBorder = parseInt(canSty.getPropertyValue('border-top-width'));
    let pos = canvas.getBoundingClientRect();
    let x = e.clientX - pos.x - leftBorder;
    let y = e.clientY - pos.y - topBorder;
    return { x: x, y: y };
}

function calculationDone() {
    calc.removeEventListener('click', calcListener);
    canvas.removeEventListener('click', canvasListener);
    canvas.style.cursor = 'default';
}

function calcListener() {
    try {
        app.calculate();
    } catch (e) {
        let span = document.createElement('span');
        span.style.color = 'red';
        span.innerText = e;
        distance.appendChild(span);
    } finally {
        calculationDone();
    }
}

function canvasListener(e) {
    let pos = getClickPos(e);
    let ctx = canvas.getContext('2d');
    drawCircle(ctx, pos.x, pos.y);
    app.addPoint(pos.x, pos.y);
}

function drawGrid() {
    let ctx = canvas.getContext('2d');
    let w = canvas.width;
    let h = canvas.height;
    ctx.clearRect(0, 0, w, h);
    ctx.beginPath();
    for (let i = 0; i < w; i += 20) {
        ctx.moveTo(i, 0);
        ctx.lineTo(i, h);
    }
    for (let i = 0; i < h; i += 20) {
        ctx.moveTo(0, i);
        ctx.lineTo(w, i);
    }
    ctx.save();
    ctx.strokeStyle = '#ccc';
    ctx.stroke();
    ctx.restore();
}

clear.addEventListener('click', () => {
    calc.addEventListener('click', calcListener);
    canvas.addEventListener('click', canvasListener);
    canvas.style.cursor = 'pointer';
    drawGrid();
    distance.innerHTML = '';
    app.clear();
});

clear.click();
