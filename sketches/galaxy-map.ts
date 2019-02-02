import { ISketch } from "./sketch";

export class GalaxyMap implements ISketch {
    public readonly name = "Galaxy Map";

    public readonly width = 800;
    public readonly height = 800;
    public readonly loop = false;

    public readonly maxCurves = 10;

    private t = 0;

    public reset(p: p5) {
        p.background(80, 80, 80);
        p.frameRate(2);
    }

    public draw(p: p5) {
        p.background(80, 80, 80);

        this.hyperbola(p);
        // this.drawTimesTable(p, this.t);

        const nCurves = p.random(this.maxCurves);
        for (let i = 0; i < nCurves; ++i) {
            p.push();

            p.translate(p.random(this.width), p.random(this.height));
            p.rotate(p.random(p.TWO_PI));
            const points = [...sinWave(25, this.width / 49, 100)].map((pt) => {
                const a = p.random(p.TWO_PI);
                pt[0] += Math.cos(a) * 20;
                pt[1] += Math.sin(a) * 20;
                return pt;
            });

            this.drawWave(
                p,
                points,
                // Math.floor(p.random(1, points.length / 4)),
                1,
                true,
            );

            p.pop();
        }

        this.t += 0.1;
    }

    public hyperbola(p: p5) {
        const points = [...hyperbolaWave(10, 4, 4)];

        p.push();
        p.translate(this.width * 0.05, this.height * 0.05);

        this.drawWave(p, points, points.length / 2);

        p.rotate(p.HALF_PI);
        this.drawWave(p, points, points.length / 2);

        p.rotate(p.HALF_PI);
        this.drawWave(p, points, points.length / 2);

        p.rotate(p.HALF_PI);
        this.drawWave(p, points, points.length / 2);

        p.pop();
    }

    public drawTimesTable(p: p5, f: number) {
        p.push();
        p.translate(this.width / 2, this.height / 2);

        const points = [];
        for (let i = 0; i < 100; ++i) {
            points.push([
                Math.cos(Math.PI * 2 / 99 * i) * 400,
                Math.sin(Math.PI * 2 / 99 * i) * 400,
            ] as [number, number]);
        }

        p.stroke(200, 200, 200, 50);

        for (let i = 0; i < points.length; ++i) {
            const [x, y] = points[i];
            const [nx, ny] = points[Math.floor(i * f) % points.length];

            p.line(x, y, nx, ny);
        }

        p.pop();
    }

    private drawWave(p: p5, points: Array<[number, number]>, n: number, drawEllipses: boolean = false) {
        p.noFill();

        if (drawEllipses) {
            p.stroke(200, 200, 200, 255);
            for (const [x, y] of points) {
                p.ellipse(x, y, 3);
            }
        }

        p.stroke(200, 200, 200, 50);

        for (let i = 0; i < points.length - n; ++i) {
            const [x, y] = points[i];
            const [nx, ny] = points[i + n];

            p.line(x, y, nx, ny);
        }
    }
}

function* sinWave(steps: number, xstep: number, maxHeight: number): Iterable<[number, number]> {
    for (let i = 0; i < steps; ++i) {
        yield [
            i * xstep,
            Math.sin(Math.PI * 2 / (steps - 1) * i) * maxHeight,
        ];
    }
}

function* hyperbolaWave(steps: number, xstep: number, ystep: number): Iterable<[number, number]> {
    for (let i = steps - 1; i >= 0; --i) {
        yield [0, i * ystep];
    }

    for (let i = 0; i < steps; ++i) {
        yield [i * ystep, 0];
    }
}
