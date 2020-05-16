import { ISketch } from "./sketch";

export class SpiralNoise implements ISketch {
    public readonly name = "Spiral Noise";

    public readonly width = 1920;
    public readonly height = 1080;
    public readonly loop = false;
    public readonly padding = 40;
    private t = 0;

    public reset(p: p5) {
        p.background(255);
    }

    public draw(p: p5) {
        p.translate(this.width / 2, this.height / 2);

        const nVertices = p.random(8, 20);
        const deltaRadius = p.random(8, 10);

        const baseShape = [];
        for (let i = 0; i < nVertices - 1; ++i) {
            const a = i / nVertices * p.TWO_PI;
            const l = p.noise(this.t, i);

            baseShape.push(p5.Vector.fromAngle(a, l));
        }

        p.noFill();
        p.beginShape();

        let insideCanvas = true;
        let r = 0;
        while (insideCanvas) {
            for (const pos of baseShape) {
                const x = pos.x * r;
                const y = pos.y * r;

                if (Math.abs(x) >= this.width / 2 - this.padding
                    || Math.abs(y) >= this.height / 2 - this.padding) {
                    insideCanvas = false;
                    break;
                }

                p.vertex(x, y);
                r += deltaRadius / baseShape.length;
            }
        }

        p.endShape();

        this.t += 0.1;
    }
}
