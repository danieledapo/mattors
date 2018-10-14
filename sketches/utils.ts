
/**
 * Pickup a random point in circle with the given radius. The center is assumed
 * to be centered in the origin.
 */
export function randomPointInCircle(p: p5, maxr: number): [number, number] {
    const a = p.random(0, p.TWO_PI);
    const r = p.random(1, maxr);

    return [Math.cos(a) * r, Math.sin(a) * r];
}

/**
 * Sample the circle with the given radius at angles with a given offset between
 * each other.
 * @param tstep step angle in radians to sample the circle at
 * @param radius radius of the circle
 */
export function* sampleCircle(tstep: number, radius: number): Iterable<[number, number]> {
    for (let t = 0; t <= Math.PI * 2; t += tstep) {
        const x = radius * Math.cos(t);
        const y = radius * Math.sin(t);

        yield [x, y];
    }
}
