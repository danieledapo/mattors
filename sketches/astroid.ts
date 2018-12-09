import { ISketch } from "./sketch";
import { randomPointInCircle, sampleCircle } from "./utils";

// astroid, a.k.a. Planet of the apes Caesar window
export class Astroid implements ISketch {
    public readonly name = "Astroid";

    public readonly width = 600;
    public readonly height = 600;
    public readonly loop = false;

    public readonly radius = 200;
    public readonly step = Math.PI / 24;
    public readonly perturbations = 30;
    public readonly maxRoughness = 20;

    // tslint:disable-next-line:no-empty
    public reset() { }

    public draw(p: p5) {
        this.blackboardBackground(p);
        this.astroid(p);
        this.snow(p);
    }

    private blackboardBackground(p: p5, linesCount: number = 5000) {
        p.background("#4e5a65");

        p.stroke(255, 255, 255, 5);
        p.strokeWeight(3);

        for (let i = 0; i < linesCount; ++i) {
            p.line(
                p.random(0, this.width),
                p.random(0, this.height),
                p.random(0, this.width),
                p.random(0, this.height),
            );
        }
    }

    private astroid(p: p5) {
        p.push();

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

        p.pop();
    }

    private snow(p: p5, snowflakesCount: number = 1000) {
        p.stroke(255, 255, 255, 5);
        p.strokeWeight(3);

        for (let i = 0; i < snowflakesCount; ++i) {
            p.push();

            const cx = p.random(0, this.width);
            const cy = p.random(0, this.height);
            const cr = p.random(1, 10);

            p.translate(cx, cy);

            for (let j = 0; j < 5; ++j) {
                const p1 = randomPointInCircle(p, cr);
                const p2 = randomPointInCircle(p, cr);

                drawApproximatedLine(p, p1, p2, 5, 3);
            }

            p.pop();
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
