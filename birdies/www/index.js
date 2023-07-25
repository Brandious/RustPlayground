import * as sim from "lib-simulation-wasm";

CanvasRenderingContext2D.prototype.drawCircle =
    function (x, y, radius) {
        this.beginPath();

        // ---
        // | Circle's center.
        // ----- v -v
        this.arc(x, y, radius, 0, 2.0 * Math.PI);
        // ------------------- ^ -^-----------^
        // | Range at which the circle starts and ends, in radians.
        // |
        // | By manipulating these two parameters you can e.g. draw
        // | only half of a circle, Pac-Man style.
        // ---

        this.fillStyle = 'rgb(0, 255, 128)'; // A nice green color
        this.fill();

    };



CanvasRenderingContext2D.prototype.drawTriangle = function (x, y, size, rotation) {
    this.beginPath();

    this.moveTo(
        x - Math.sin(rotation) * size * 1.5,
        y + Math.cos(rotation) * size * 1.5,
    );

    this.lineTo(
        x - Math.sin(rotation + 2.0 / 3.0 * Math.PI) * size,
        y + Math.cos(rotation + 2.0 / 3.0 * Math.PI) * size,
    );

    this.lineTo(
        x - Math.sin(rotation + 4.0 / 3.0 * Math.PI) * size,
        y + Math.cos(rotation + 4.0 / 3.0 * Math.PI) * size,
    );

    this.lineTo(
        x - Math.sin(rotation) * size * 1.5,
        y + Math.cos(rotation) * size * 1.5,
    );

    this.fillStyle = 'rgb(255, 255, 255)'; // A nice white color
    this.fill();

}


const simulation = new sim.Simulation();

document.getElementById('train').onclick = function () {
    console.log(simulation.train());
};

const viewport = document.getElementById('viewport');
const viewportWidth = viewport.width;
const viewportHeight = viewport.height;
const viewportScale = window.devicePixelRatio || 1;

viewport.width = viewportWidth * viewportScale;
viewport.height = viewportHeight * viewportScale;

viewport.style.width = viewportWidth + 'px';
viewport.style.height = viewportHeight + 'px';

const ctx = viewport.getContext('2d');

ctx.scale(viewportScale, viewportScale);
ctx.fillStyle = 'rgba(0,0.0)';

for (const animal of simulation.world().animals) {
    ctx.drawTriangle(
        animal.x * viewportWidth,
        animal.y * viewportHeight,
        0.01 * viewportWidth,
        animal.rotation,
    );

}


function redraw() {
    ctx.clearRect(0, 0, viewportWidth, viewportHeight);




    simulation.step();

    const world = simulation.world();


    for (const food of world.foods) {
        ctx.drawCircle(
            food.x * viewportWidth,
            food.y * viewportHeight,
            (0.01 / 2.0) * viewportWidth,
        );
    }

    for (const animal of simulation.world().animals) {
        ctx.drawTriangle(
            animal.x * viewportWidth,
            animal.y * viewportHeight,
            0.01 * viewportWidth,
            animal.rotation,
        );

    }

    requestAnimationFrame(redraw);
}

redraw();
