import { ISketch } from "./sketch";

export class NoiseQuads implements ISketch {
    public readonly name = "Noise Quads";

    public readonly width = 1920;
    public readonly height = 1080;
    public readonly loop = true;

    private t = 0;
    private n = 0;
    private padding = 20;

    public reset(p: p5) {
        p.background("white");

        this.t += p.random();
        this.n = Math.floor(p.random(1, 5));

        p.strokeWeight(10);
        p.rect(0, 0, this.width, this.height);
        p.strokeWeight(1);
    }

    public draw(p: p5) {
        p.push();
        p.translate(this.padding, this.padding);

        p.rectMode(p.CENTER);

        const s = Math.min(this.width - this.padding * 2, this.height - this.padding * 2) / this.n;

        const w = this.width - this.padding * 2;
        const h = this.height - this.padding * 2;
        const wpad = (w - Math.floor(w / s) * s) / 2;
        const hpad = (h - Math.floor(h / s) * s) / 2;

        p.stroke("black");
        p.fill("white");

        for (let x = wpad; x <= this.width - s - this.padding * 2 - wpad; x += s) {
            for (let y = hpad; y <= this.height - s - this.padding * 2 - hpad; y += s) {
                const nt = p.noise(x, y, this.t);
                const l = s * nt;

                if (p.random() > 0.3) {
                    p.noFill();
                } else {
                    p.fill("white");
                }

                p.push();
                p.translate(x + s / 2, y + s / 2);
                // p.rotate(p.PI * nt);
                p.rect(0, 0, l, l);
                p.pop();
            }
        }

        this.n += 3;

        p.frameRate(1);
        // p.noLoop();

        p.pop();
    }
}
