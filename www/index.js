import "./styles.scss"
import * as wasm from "rust-wasm"

// ms per sec
const msPerSec = 1000.0

// Boid colors!
const colors = ["cyan", "coral", "lime", "yellow", "violet"]

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
let ctx = canvas.getContext("2d")

// Get boid svg
const boid = createBoid({
    length: 20,
    eccentricity: 0.9,
    divet: 0.2,
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
    ctx.restore()
}

// Boid parameters
const max_query_distance = 100
const max_angle_change = 0.3
const separation = 50
const boid_count = 50
const boid_velocity = 300

// Initialize simulation
let sim = wasm.BoidsSim.init(
    width,
    height,
    boid.length,
    boid_velocity,
    max_query_distance,
    max_angle_change,
    separation,
    boid_count
)

// Animation loop
function animation({ dt }) {
    ctx.clearRect(0, 0, width, height)
    for (let { id, x, y, a } of sim.get_boids()) {
        drawBoid(x, y, a, colors[id % colors.length])
    }
    sim.update_boids(dt)
}

// Start animation
let start = performance.now() / msPerSec
let loopFunc = () => {
    let end = performance.now() / msPerSec
    let dt = end - start
    start = end
    animation({ dt })
    requestAnimationFrame(loopFunc)
}
requestAnimationFrame(loopFunc)
