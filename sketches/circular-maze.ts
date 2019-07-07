import { ISketch } from "./sketch";

export class CircularMaze implements ISketch {
    public readonly name = "Circular Maze (kind of)";

    public readonly width = 1920;
    public readonly height = 1080;
    public readonly loop = false;

    public reset(p: p5) {
        const h1 = p.random(360);
        const h2 = (h1 + 180) % 360;

        p.colorMode(p.HSB);
        p.background(h1, 80, 80);
        p.stroke(h2, 80, 80);
        p.noFill();
        p.rect(0, 0, this.width, this.height);
    }

    public draw(p: p5) {
        p.push();
        p.translate(this.width / 2, this.height / 2);

        const r = Math.min(this.width, this.height) / 2 - 20;
        const nested = Math.floor(p.random(3, 20));
        const divs = Math.floor(p.random(10, 20));

        const pl = p.random(0.7);
        const pa = p.random(0.7);

        p.noFill();
        p.strokeWeight(5);

        for (let i = 0; i < nested - 1; ++i) {
            const t = i / (nested - 1);

            for (let j = 0; j < divs; ++j) {
                if (p.random() > pa) {
                    p.arc(
                        0,
                        0,
                        r * 2 * t,
                        r * 2 * t,
                        (p.TWO_PI * j) / divs,
                        (p.TWO_PI * (j + 1)) / divs,
                    );
                }
            }
        }
        p.ellipse(0, 0, r * 2, r * 2);

        for (let i = 0; i < divs; ++i) {
            const a = p.TWO_PI * (i / divs);

            for (let j = 0; j < nested - 1; ++j) {
                const sr = r * (j / (nested - 1));
                const br = r * ((j + 1) / (nested - 1));

                if (p.random() > pl) {
                    p.line(sr * Math.cos(a), sr * Math.sin(a), br * Math.cos(a), br * Math.sin(a));
                }
            }
        }

        p.pop();
    }
}
