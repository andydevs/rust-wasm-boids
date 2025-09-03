import "./styles.scss"
import * as wasm from "rust-wasm"
import { createBoid, drawBoid } from "./boid.js";

// Get canvas
let canvas = document.querySelector("#boids-canvas")
let ctx = canvas.getContext("2d")
let width = canvas.width
let height = canvas.height

// Boid parameters
const boid_length = 20
const boid_eccentricity = 0.9
const boid_divet = 0.2
const boid_count = 100
const boid_velocity = 300
const max_query_distance = 100
const max_angle_change = 2 * Math.PI
const separation = 40

// Initialize simulation
let sim = wasm.BoidsSim.init(
    width,
    height,
    boid_length,
    boid_velocity,
    max_query_distance,
    max_angle_change,
    separation,
    boid_count
)

// Get boid svg
const graphic = createBoid({
    length: boid_length,
    eccentricity: boid_eccentricity,
    divet: boid_divet,
})

// Animation loop
function animation({ dt }) {
    ctx.clearRect(0, 0, width, height)
    for (let boid of sim.get_boids()) {
        drawBoid(ctx, graphic, boid)
    }
    sim.update_boids(dt)
}

// Start animation
const msPerSec = 1000.0
let start = performance.now() / msPerSec
let loopFunc = () => {
    let end = performance.now() / msPerSec
    let dt = end - start
    start = end
    animation({ dt })
    requestAnimationFrame(loopFunc)
}
requestAnimationFrame(loopFunc)
