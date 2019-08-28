import { ISketch } from "./sketch";

export class Rots implements ISketch {
    public readonly name = "Rots";

    public readonly width = 1920;
    public readonly height = 1080;
    public readonly loop = true;

    public reset(p: p5) {
        p.background("white");
    }

    public draw(p: p5) {
        p.background("white");

        const rows = 4;
        const columns = 7;
        const cellW = this.width / columns;
        const cellH = this.height / rows;
        const t = p.random(0.05, 0.5);

        for (let r = 0; r < rows; ++r) {
            for (let c = 0; c < 10; ++c) {
                p.push();
                p.translate(c * cellW + cellW / 2, r * cellH + cellH / 2);
                p.rotate(p.random(p.TWO_PI));

                this.drawRots(p, 0.95 * Math.min(cellW / 2, cellH / 2), t);

                p.pop();
            }
        }

        p.frameRate(0.25);
    }

    private drawRots(p: p5, r: number, t: number) {
        const nVertices = Math.floor(p.random(3, 8));

        let vertices: p5.Vector[] = [];
        for (let i = 0; i < nVertices; ++i) {
            vertices.push(p5.Vector.fromAngle((i / nVertices) * p.TWO_PI, r));
        }

        const drawVertices = (t: number) => {
            p.beginShape();
            for (const { x, y } of vertices) {
                p.vertex(x, y);
            }
            p.endShape(p.CLOSE);
        };

        drawVertices(0);

        const n = Math.floor(p.random(2, 10));

        for (let i = 0; i < n; ++i) {
            const newVertices = [];

            for (let j = 0; j < vertices.length; ++j) {
                newVertices.push(
                    p5.Vector.lerp(vertices[j], vertices[(j + 1) % vertices.length], t),
                );
            }

            vertices = newVertices;
            drawVertices((i + 1) / n);
        }
    }
}
