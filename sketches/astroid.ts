import { ISketch } from "./sketch";

// astroid, a.k.a. Planet of the apes Caesar window
export class Astroid implements ISketch {
    public readonly name = "Astroid";

    public readonly width = 600;
    public readonly height = 600;

    public readonly radius = 200;
    public readonly step = Math.PI / 24;
    public readonly perturbations = 30;
    public readonly maxRoughness = 20;

    public reset(p: p5) {
        p.background("#4e5a65");
    }

    public draw(p: p5) {
        p.translate(this.width / 2, this.height / 2);

        p.stroke(255, 255, 255, 30);
        p.strokeWeight(3);

        let prev: [number, number] | null = null;
        for (const c of sampleCircle(this.step, this.radius)) {
            if (prev !== null) {
                drawApproximatedLine(p, prev, c, this.perturbations, this.maxRoughness);
            }

            prev = c;
        }

        prev = null;
        for (const astro of astroid(this.step, this.radius)) {
            if (prev !== null) {
                drawApproximatedLine(p, prev, astro, this.perturbations, this.maxRoughness);
            }

            prev = astro;
        }
    }
}

/**
 * Approximate an astroid https://en.wikipedia.org/wiki/Astroid returning the
 * points.
 * @param tstep step angle in radians to sample the astroid at
 * @param radius radius of the astroid
 */
export function* astroid(tstep: number, radius: number): Iterable<[number, number]> {
    for (let t = 0; t <= Math.PI * 2; t += tstep) {
        const x = radius / 4 * (3 * Math.cos(t) + Math.cos(3 * t));
        const y = radius / 4 * (3 * Math.sin(t) - Math.sin(3 * t));

        yield [x, y];
    }
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

/**
 * Draw a perturbed line by drawing `permutations` lines where each coordinate
 * is slightly randomly offset.
 */
export function drawApproximatedLine(
    p: p5,
    p0: [number, number],
    p1: [number, number],
    perturbations: number,
    maxRoughness: number,
) {
    for (let it = 0; it < perturbations; ++it) {
        const roughness = p.random(0, maxRoughness);
        p.line(
            p0[0] + p.random(-roughness, roughness),
            p0[1] + p.random(-roughness, roughness),
            p1[0] + p.random(-roughness, roughness),
            p1[1] + p.random(-roughness, roughness),
        );
    }
}
