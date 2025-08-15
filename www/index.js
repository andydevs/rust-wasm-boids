import "./styles.scss"
import * as wasm from "rust-wasm"
import { memory } from "rust-wasm/rust_wasm_bg.wasm"

function createBoid({ length, eccentricity, divet }) {
    let x = length / 2
    let y = (length * eccentricity) / 2
    let path = new Path2D(`
        M ${-x} ${-y} 
        L ${x} 0
        L ${-x} ${y}
        L ${-x + length * divet} 0 
        Z
    `)
    return {
        length,
        eccentricity,
        divet,
        path,
    }
}

// Get canvas
let canvas = document.querySelector("#boids-canvas")
let width = canvas.width
let height = canvas.height
let cx = width / 2
let cy = height / 2
let ctx = canvas.getContext("2d")

// Get boid svg
const boid = createBoid({
    length: 20,
    eccentricity: 0.8,
    divet: 0.1,
})

/**
 * Boid drawing handler
 *
 * @param {number} x x position of boid
 * @param {number} y y position of boid
 * @param {number} a angle relative to horizontal line for boid
 * @param {string} color color of boid to draw
 */
function drawBoid(x, y, a, color) {
    ctx.save()
    ctx.translate(x, y)
    ctx.rotate(a)
    ctx.fillStyle = color
    ctx.fill(boid.path)
    ctx.stroke(boid.path)
    ctx.restore()
}

// Initialize simulation
let sim = wasm.BoidsSim.init(width, height, boid.length, 4)

// Animation loop
function animation() {
    ctx.clearRect(0, 0, width, height)
    for (let { x, y, a } of sim.get_boids()) {
        drawBoid(x, y, a, "cyan")
    }
    sim.update_boids()
}

// Start animation
let loopFunc = () => {
    animation()
    requestAnimationFrame(loopFunc)
}
requestAnimationFrame(loopFunc)
