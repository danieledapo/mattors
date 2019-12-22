import { ISketch } from "./sketch";

export class NoiseSymmetry implements ISketch {
    public readonly name = "Noise Symmetry";

    public readonly width = 1920;
    public readonly height = 1080;
    public readonly loop = true;
    private t = 0;

    private curve: [number, number][] = [];

    public reset(p: p5) {
        p.frameRate(15);

        p.background("white");
        p.stroke(0, 0, 0, 100);

        let x = this.width / 2;
        let ystep = this.height / 2 / p.random(3, 10);

        this.curve = [];
        for (let y = 0; y < this.height / 2; y += ystep) {
            this.curve.push([x, y]);
            const t = p.noise(x, this.t) - 0.5;

            x += t * (this.width / 4);

            ystep = ystep * 0.9;
        }

        for (let j = this.curve.length - 2; j >= 0; --j) {
            this.curve.push([this.curve[j][0], this.height - this.curve[j][1]]);
        }
    }

    public draw(p: p5) {
        p.noFill();

        const delta = 30;

        const curve: [number, number][] = [];
        for (let i = 0; i < this.curve.length; ++i) {
            const a = p.noise(this.t, ...this.curve[i]) * p.TWO_PI;

            if (p.random() < 0.1) {
                continue;
            }

            curve.push([
                this.curve[i][0] + Math.cos(a) * delta,
                this.curve[i][1] + Math.sin(a) * delta,
            ]);
        }

        p.beginShape();
        for (const [x, y] of curve) {
            p.curveVertex(x, y);
        }
        p.endShape();

        p.beginShape();
        for (const [x, y] of curve) {
            p.curveVertex(this.width - x, y);
        }
        p.endShape();

        this.t += 0.1;
    }
}
