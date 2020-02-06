const CIRCLE_RADIUS = 5;

export function drawCircle(ctx, x, y) {
    ctx.beginPath();
    ctx.arc(x, y, CIRCLE_RADIUS, 0, Math.PI * 2);
    ctx.fill();
}

export function presentResult(dist, p0_x, p0_y, p1_x, p1_y) {
    let distance = document.getElementById('dist');
    let canvas = document.querySelector('canvas');
    distance.innerText = 'Distance: ' + dist.toFixed(3);

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
