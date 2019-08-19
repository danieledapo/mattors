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

        for (let x = 0; x < this.width; x += 5) {
            let s = 5 + ((x / 50) * this.width) / this.height;

            for (let y = 0; y < this.height; y += s) {
                const t = Math.max((y + s) / this.height, (x + 5) / this.width);

                if (p.random() > t) {
                    p.line(x, y, x, y + s);
                }

                s *= 1.00005;
            }
        }
        p.frameRate(1);
    }
}
