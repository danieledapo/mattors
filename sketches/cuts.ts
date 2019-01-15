import { ISketch } from "./sketch";

export class Cuts implements ISketch {
    public readonly name = "Cuts and Tears";

    public readonly width = 600;
    public readonly height = 600;
    public readonly loop = false;

    private readonly padding = 30;
    private readonly nlines = 30;

    public reset(p: p5) {
        p.background(80, 80, 80);
    }

    public draw(p: p5) {
        const ph = p.random(0, 0.3);
        const pv = 1 - ph;

        p.push();
        for (let i = 0; i < this.nlines; ++i) {
            p.translate(0, this.height / (this.nlines + 1));

            if (p.random() > ph) {
                this.drawCurve(p, i);
            }
        }
        p.pop();

        for (let i = 0; i < this.nlines; ++i) {
            p.translate(this.width / (this.nlines + 1), 0);

            if (p.random() > pv) {
                p.push();
                p.rotate(p.HALF_PI);
                this.drawCurve(p, i / this.nlines);
                p.pop();
            }
        }
    }

    public drawCurve(p: p5, i: number) {
        const c = p.color(255, 255, 255, 10);

        const t = i / this.nlines;

        const maxAmpl = this.width / this.nlines / 2;
        const amp = maxAmpl * 0.25 + p.noise(t) * maxAmpl * 0.75;
        const aoff = p.noise(t, amp) * 2 * p.TWO_PI;

        const startOffset = p.random(this.width / 3);
        const endOffset = p.random(this.width / 3);

        for (let x = this.padding + startOffset; x < this.width - this.padding - endOffset; x += 1) {
            c.setAlpha(p.alpha(c) + 130 / this.width);
            p.stroke(c);
            p.strokeCap(p.SQUARE);
            p.noFill();

            p.ellipse(
                x,
                -Math.cos(aoff + p.map(x, 0, this.width, 0, p.TWO_PI)) * amp,
                1,
                1,
            );
        }
    }
}
