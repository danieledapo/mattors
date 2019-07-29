import { ISketch } from "./sketch";

export class Chaikin implements ISketch {
    public readonly name = "Chaikin";

    public readonly width = 800;
    public readonly height = 800;
    public readonly loop = false;

    public reset(p: p5) {
        p.background("white");
        // p.frameRate(10);
    }

    public draw(p: p5) {
        p.push();

        const steps = 4;
        const points = 100;

        let path = [];
        for (let i = 0; i < points; ++i) {
            path.push(
                p.createVector((i / (points - 1)) * this.width, (p.random() * this.height) / steps),
            );
        }

        p.noFill();
        for (let i = 1; i < steps; ++i) {
            p.beginShape();
            for (const pt of path) {
                p.vertex(pt.x, pt.y);
            }
            p.endShape();

            const newPath: p5.Vector[] = [];

            for (let j = 0; j < path.length - 1; ++j) {
                newPath.push(
                    path[j]
                        .copy()
                        .mult(0.75)
                        .add(path[j + 1].copy().mult(0.25)),
                    path[j]
                        .copy()
                        .mult(0.25)
                        .add(path[j + 1].copy().mult(0.75)),
                );
            }
            path = newPath;

            p.translate(0, this.height / steps);
        }

        p.pop();
    }
}
