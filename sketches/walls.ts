import { ISketch } from "./sketch";

export class Walls implements ISketch {
    public readonly name = "Walls";

    public readonly width = 1300;
    public readonly height = 800;
    public readonly loop = false;

    public readonly generations = 50;

    private time = 0;

    public reset(p: p5) {
        p.background(80, 80, 80);

        this.time += 1;
    }

    public draw(p: p5) {
        p.noFill();
        p.stroke(255, 255, 255, 100);

        p.translate(0, this.height / 2);
        this.drawPlanesStrip(p, 5);

        p.translate(0, this.height / 2);
        this.drawPlanesStrip(p, 7);
    }

    public drawPlanesStrip(p: p5, id: number) {
        const points: p5.Vector[] = [];

        for (let i = 0; i < 20; ++i) {
            const a = -p.map(i, 0, 19, p.HALF_PI, p.PI);
            points.push(
                p.createVector(
                    Math.cos(a) * 300 + p.random(-10, 10),
                    Math.sin(a) * 300 + p.random(-10, 10),
                ),
            );
        }

        for (let pi = 0; pi < points.length - 1; ++pi) {
            p.ellipse(points[pi].x, points[pi].y, 5);
            p.line(points[pi].x, points[pi].y, points[pi + 1].x, points[pi + 1].y);
        }

        for (let genId = 0; genId < this.generations; ++genId) {
            const a = p.map(p.noise(this.time, id, genId), 0, 1, -p.HALF_PI / 2, p.HALF_PI / 2);

            for (const pt of points) {
                const dx = Math.cos(a) * 50;
                const dy = Math.sin(a) * 30;

                p.line(pt.x, pt.y, pt.x + dx, pt.y + dy);

                pt.add(dx, dy);
            }

            for (let pi = 0; pi < points.length - 1; ++pi) {
                p.ellipse(points[pi].x, points[pi].y, 5);
                p.line(points[pi].x, points[pi].y, points[pi + 1].x, points[pi + 1].y);
            }
        }
    }
}
