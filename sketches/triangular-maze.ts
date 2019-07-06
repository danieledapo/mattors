import { ISketch } from "./sketch";

type Triangle = [p5.Vector, p5.Vector, p5.Vector];

export class TriangularMaze implements ISketch {
    public readonly name = "Triangular Maze (kind of)";

    public readonly width = 1920;
    public readonly height = 1080;
    public readonly loop = true;

    private seenCenters: p5.Vector[] = [];
    private stack: Triangle[] = [];

    public reset(p: p5) {
        p.frameRate(1);

        p.colorMode(p.HSB);
        const h1 = p.random(0, 360);
        p.background(h1, 80, 80);
        p.stroke((h1 + 180) % 360, 80, 80);

        this.seenCenters = [];

        const l = Math.min(this.width, this.height) / 2;
        this.stack = [
            [
                p.createVector(0, -Math.sqrt(3) / 2).mult(l),
                p.createVector(0.5, 0).mult(l),
                p.createVector(-0.5, 0).mult(l),
            ],
        ];
    }

    public draw(p: p5) {
        p.push();
        p.translate(this.width / 2, this.height / 2);

        p.noFill();
        p.strokeWeight(5);

        const newStack: Triangle[] = [];

        for (const tri of this.stack) {
            const c = triangleCenter(tri);
            if (this.seenCenters.findIndex(sc => sc.dist(c) < 1e-6) >= 0) {
                continue;
            }
            this.seenCenters.push(c);

            this.drawTriangle(p, tri);

            for (const n of this.triangleNeighbors(p, tri)) {
                const nc = triangleCenter(n);
                if (nc.magSq() > Math.pow(this.width / 2, 2) + Math.pow(this.height / 2, 2)) {
                    continue;
                }

                newStack.push(n);
            }
        }

        this.stack = newStack;
        p.pop();
    }

    private drawTriangle(p: p5, tri: Triangle) {
        p.line(tri[0].x, tri[0].y, tri[1].x, tri[1].y);
        p.line(tri[1].x, tri[1].y, tri[2].x, tri[2].y);
        p.line(tri[2].x, tri[2].y, tri[0].x, tri[0].y);

        const steps = Math.floor(p.random(3, 20));
        for (let e = 0; e < tri.length; ++e) {
            const a = tri[e];
            const b = tri[(e + 1) % tri.length];
            const c = tri[(e + 2) % tri.length];

            for (let i = 0; i < steps; ++i) {
                const t = i / (steps - 1);

                const p1 = p5.Vector.lerp(a, b, t);
                const p2 = p5.Vector.lerp(a, c, t);
                for (let j = 0; j < i; ++j) {
                    if (p.random() > 0.3) {
                        continue;
                    }

                    const sp1 = p5.Vector.lerp(p1, p2, j / i);
                    const sp2 = p5.Vector.lerp(p1, p2, (j + 1) / i);
                    p.line(sp1.x, sp1.y, sp2.x, sp2.y);
                }
            }
        }
    }

    private triangleNeighbors(p: p5, tri: Triangle): Triangle[] {
        const third = (a: p5.Vector, b: p5.Vector): p5.Vector => {
            const mid = a
                .copy()
                .add(b)
                .div(2);
            const ba = b
                .copy()
                .sub(a)
                .normalize();
            const off = p.createVector(ba.y, -ba.x).mult((a.dist(b) * Math.sqrt(3)) / 2);
            return mid.add(off);
        };

        return [
            [tri[1], tri[0], third(tri[0], tri[1])],
            [tri[2], tri[1], third(tri[1], tri[2])],
            [tri[0], tri[2], third(tri[2], tri[0])],
        ];
    }
}

function triangleCenter(tri: Triangle): p5.Vector {
    return tri[0]
        .copy()
        .add(tri[1])
        .add(tri[2])
        .div(3);
}
