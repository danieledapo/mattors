import { ISketch } from "./sketch";

export class GooglyEyes implements ISketch {
    public readonly name = "Googly Eyes";

    public readonly width = 800;
    public readonly height = 800;
    public readonly loop = true;

    private t: number = 0;
    private colors: p5.Color[][][] = [];
    private r = 0;
    private readonly horEyes = 5;
    private readonly verEyes = 5;
    private readonly subEyes = 4;

    public reset(p: p5) {
        p.colorMode(p.HSB);

        for (let x = 0; x < this.horEyes; ++x) {
            this.colors[x] = [];
            for (let y = 0; y < this.verEyes; ++y) {
                this.colors[x][y] = [];

                const h = p.color(p.random(210, 360), 80, 100);
                const oh = p.color((180 + p.hue(h)) % 360, 80, 100);

                for (let e = 0; e < this.subEyes; ++e) {
                    const t = e / (this.subEyes - 1);
                    this.colors[x][y][e] = p.lerpColor(h, oh, t);
                }
            }
        }

        this.r = Math.min((this.width) / this.horEyes, (this.height) / this.verEyes) - 20;
        this.r /= 2;

    }

    public draw(p: p5) {
        p.background("#e4572e");

        const xstep = this.width / this.horEyes;
        const ystep = this.height / this.horEyes;

        p.noStroke();

        for (let x = 0; x < this.horEyes; ++x) {
            for (let y = 0; y < this.verEyes; ++y) {
                for (let e = 0; e < this.subEyes; ++e) {
                    p.push();

                    p.translate(x * xstep + xstep / 2, y * ystep + ystep / 2);

                    const a = p.noise(x, y, this.t) * p.TWO_PI;
                    const px = Math.cos(a) * (this.r);
                    const py = Math.sin(a) * (this.r);

                    for (let c = 0; c < this.subEyes; ++c) {
                        p.fill(this.colors[x][y][c]);

                        const cr = this.r * (this.subEyes - 1 - c) / (this.subEyes - 1);
                        p.ellipse(px - Math.cos(a) * cr, py - Math.sin(a) * cr, cr * 2, cr * 2);
                    }

                    p.pop();
                }
            }
        }

        this.t += 0.01;
    }
}
