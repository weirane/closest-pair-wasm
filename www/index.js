import {WasmApp} from 'closest-pair-wasm';

const CIRCLE_RADIUS = 5;
let app = new WasmApp;
let calc = document.querySelector('#calc');
let canvas = document.querySelector('canvas');
let clear = document.querySelector('#clear');
let distance = document.querySelector('#dist');

function getClickPos(e) {
    let canSty = getComputedStyle(canvas);
    let topBorder = parseInt(canSty.getPropertyValue('border-left-width'));
    let leftBorder = parseInt(canSty.getPropertyValue('border-top-width'));
    let pos = canvas.getBoundingClientRect();
    let x = e.clientX - pos.x - leftBorder;
    let y = e.clientY - pos.y - topBorder;
    return {x: x, y: y};
}

function drawCircle(ctx, x, y) {
    ctx.beginPath();
    ctx.arc(x, y, CIRCLE_RADIUS, 0, Math.PI * 2);
    ctx.fill();
}

function presentResult(dist, p0_x, p0_y, p1_x, p1_y) {
    distance.innerHTML = 'Distance: ' + app.dist.toFixed(3);

    // Highlight the points red and draw a line between them
    let ctx = canvas.getContext('2d');
    ctx.save();
    ctx.fillStyle = '#ff0000';
    ctx.strokeStyle = '#ff0000';
    drawCircle(ctx, p0_x, p0_y);
    drawCircle(ctx, p1_x, p1_y);
    ctx.beginPath();
    ctx.moveTo(p0_x, p0_y);
    ctx.lineTo(p1_x, p1_y);
    ctx.lineWidth = 2;
    ctx.stroke();
    ctx.restore();

    console.log(`(${p0_x}, ${p0_y})`);
    console.log(`(${p1_x}, ${p1_y})`);
}

function calculationDone() {
    calc.removeEventListener('click', calcListener);
    canvas.removeEventListener('click', canvasListener);
    canvas.style.cursor = 'default';
}

function calcListener() {
    try {
        app.calculate();
        presentResult(app.dist, app.p0_x, app.p0_y, app.p1_x, app.p1_y);
    } catch (e) {
        distance.innerHTML = `<span style="color:red">${e}</span>`;
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
