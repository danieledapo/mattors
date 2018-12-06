import { ISketch } from "./sketch";
import { randomPointInCircle, sampleCircle } from "./utils";

export class Nucleus implements ISketch {
    public readonly name = "Nucleus";

    public readonly width = 600;
    public readonly height = 600;

    public readonly circleDeformations = 15;
    public readonly perturbations = 10;

    public reset(p: p5) {
        p.background("white");
    }

    public draw(p: p5) {
        p.translate(this.width / 2, this.height / 2);
        this.drawDrop(p);
    }

    private drawDrop(p: p5) {
        if (p.random() > 0.5) {
            p.blendMode(p.EXCLUSION);
        }

        p.fill(p.random(255), p.random(255), p.random(255), 10);
        p.stroke(0, 0, 0, 30);

        for (let i = 0; i < this.circleDeformations; ++i) {
            let polylinePoints: p5.Vector[] = [];

            for (const [x, y] of sampleCircle(p.PI / 8, Math.min(this.width, this.height) / 5)) {
                polylinePoints.push(p.createVector(x, y));
            }

            polylinePoints = this.irregularize(p, polylinePoints, 30);

            for (let j = 0; j < this.perturbations; ++j) {
                polylinePoints = this.irregularize(p, polylinePoints);
                this.drawPoly(p, polylinePoints);
            }
        }
    }

    private drawPoly(p: p5, polylinePoints: p5.Vector[]) {
        p.beginShape();
        for (const p0 of polylinePoints) {
            p.vertex(p0.x, p0.y);
        }
        p.endShape();
    }

    private irregularize(p: p5, polylinePoints: p5.Vector[], minEdgeLength?: number): p5.Vector[] {
        const out: p5.Vector[] = [];

        for (let i = 0; i < polylinePoints.length; ++i) {
            const p0 = polylinePoints[i];
            const p1 = polylinePoints[(i + 1) % polylinePoints.length];

            out.push(p0);
            // p1 is pushed in next iter

            const d = p0.dist(p1);

            if (minEdgeLength !== undefined && d < minEdgeLength) {
                continue;
            }

            const pt = randomPointInCircle(p, d);
            const mid = p5.Vector.add(p0, p1).div(2);

            out.push(mid.add(pt[0], pt[1]));
        }

        return out;
    }
}
