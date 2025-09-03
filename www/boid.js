// Boid colors!
const colors = ["cyan", "coral", "lime", "yellow", "violet"]

/**
 * Create packaged information for drawing a boid
 * 
 * @param {Object} param0 boid shape parameters
 * 
 * @returns {Object} boid drawing information 
 */
export function createBoid({ length, eccentricity, divet }) {
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

/**
 * Boid drawing handler
 *
 * @param {CanvasRenderingContext2D} ctx draw ctx  
 * @param {Object} graphic information related to drawing boid
 * @param {Boid} boid boid to draw
 */
export function drawBoid(ctx, graphic, boid) {
    let { id, x, y, a } = boid
    ctx.save()
    ctx.translate(x, y)
    ctx.rotate(a)
    ctx.fillStyle = colors[id % colors.length]
    ctx.fill(graphic.path)
    ctx.restore()
}