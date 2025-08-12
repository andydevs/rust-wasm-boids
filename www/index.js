import "./styles.scss"
import * as wasm from "rust-wasm"

function createBoidPath({ length, eccentricity, divet }) {
    let x = length / 2
    let y = (length * eccentricity) / 2
    return new Path2D(`
        M ${-x} ${-y} 
        L ${x} 0
        L ${-x} ${y}
        L ${-x + length * divet} 0 
        Z
    `)
}

// Get canvas
let canvas = document.querySelector("#boids-canvas")
let width = canvas.width
let height = canvas.height
let ctx = canvas.getContext("2d")

// Get boid svg
const boid = createBoidPath({
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
    ctx.rotate((a * Math.PI) / 180)
    ctx.fillStyle = color
    ctx.fill(boid)
    ctx.stroke(boid)
    ctx.restore()
}

let angle = 0

// Animation loop
function animation() {
    ctx.clearRect(0, 0, width, height)
    drawBoid(width / 2, height / 2, angle, "cyan")
}

// Start animation
let loopFunc = () => {
    animation()
    requestAnimationFrame(loopFunc)
}
requestAnimationFrame(loopFunc)
