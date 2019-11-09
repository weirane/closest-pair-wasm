import * as wasm from 'closest-pair-wasm';

const CIRCLE_RADIUS = 5;
let calculated = false;
let calc = document.querySelector('#calc');
let canvas = document.querySelector('canvas');
let clear = document.querySelector('#clear');
let distance = document.querySelector('#dist');
let points = new Array();

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

calc.addEventListener('click', () => {
    if (points.length == 0) {
        // No points are given
        console.log('Empty canvas!');
        return;
    }
    if (calculated) {
        return;
    }
    let xs = new Float64Array(points.map(p => p[0]));
    let ys = new Float64Array(points.map(p => p[1]));
    let ret = wasm.calculate(xs, ys);

    distance.innerHTML = 'Distance: ' + ret.dist.toFixed(3);

    // Highlight the points red and draw a line between them
    let ctx = canvas.getContext('2d');
    let origFillStyle = ctx.fillStyle;
    let origStrokeStyle = ctx.strokeStyle;
    let origLineWidth = ctx.lineWidth;
    ctx.fillStyle = '#ff0000';
    ctx.strokeStyle = '#ff0000';
    drawCircle(ctx, ret.p0_x, ret.p0_y);
    drawCircle(ctx, ret.p1_x, ret.p1_y);
    ctx.beginPath();
    ctx.moveTo(ret.p0_x, ret.p0_y);
    ctx.lineTo(ret.p1_x, ret.p1_y);
    ctx.lineWidth = 2;
    ctx.stroke();
    ctx.fillStyle = origFillStyle;
    ctx.strokeStyle = origStrokeStyle;
    ctx.lineWidth = origLineWidth;

    calculated = true;
    console.log(`(${ret.p0_x}, ${ret.p0_x})`);
    console.log(`(${ret.p1_x}, ${ret.p1_y})`);
});

clear.addEventListener('click', () => {
    calculated = false;
    distance.innerHTML = '';
    points.length = 0;
    let ctx = canvas.getContext('2d');
    ctx.clearRect(0, 0, canvas.width, canvas.height);
});

canvas.addEventListener('click', (e) => {
    if (calculated) {
        return;
    }
    let pos = getClickPos(e);
    let ctx = canvas.getContext('2d');
    drawCircle(ctx, pos.x, pos.y);
    points.push([pos.x, pos.y]);
});
