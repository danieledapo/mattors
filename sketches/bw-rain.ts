import { ISketch } from "./sketch";

export class BlackWhiteRain implements ISketch {
    public readonly name = "Black and White Rain";

    public readonly width = 841;
    public readonly height = 1189;
    public readonly loop = true;

    public reset(p: p5) {
        p.background("white");
    }

    public draw(p: p5) {
        p.background("white");

        const nDiags = p.random(20, 200);
        const horStep = p.random(2, 5);

        for (let x = 0; x < this.width; x += horStep) {
            let s = horStep + ((x / nDiags) * this.width) / this.height;

            for (let y = 0; y < this.height; y += s) {
                const t = Math.max((y + s) / this.height, (x + horStep) / this.width);

                if (p.random() > t) {
                    p.line(x, y, x, y + s);
                }

                s *= 1.00005;
            }
        }
        p.frameRate(1);
    }
}
