import { ISketch } from "./sketch";

// heavily inspired by https://github.com/anvaka/atree
export class ChristmasSpiralTree implements ISketch {
    public readonly name = "Christmas Spiral Tree";

    public readonly width = 600;
    public readonly height = 600;
    public readonly loop = true;

    private startAngle = 0;

    public reset(p: p5) {
        p.background("black");
    }

    public draw(p: p5) {
        p.background("black");

        p.translate(this.width / 2, this.height);

        const spirals = [
            new Spiral(p.color("#ff0000"), 0, this.startAngle + Math.PI / 2),
            new Spiral(p.color("#00ffcc"), 0, this.startAngle + Math.PI / 2 * 3),
        ];

        while (spirals.filter((s) => s.radius !== 0).length === spirals.length) {
            for (const spiral of spirals) {
                spiral.draw(p);
                spiral.update();
            }
        }

        this.startAngle += p.PI / 64;
        if (this.startAngle >= p.TWO_PI) {
            this.startAngle = this.startAngle - p.TWO_PI;
        }

        p.frameRate(40);
    }

}

class Spiral {
    constructor(
        public color: p5.Color,
        public y: number,
        public angle: number,
        public yOff: number = -5,
        public angleOff: number = Math.PI / 16,
        public radius: number = 220,
        public radiusDec: number = 2,
    ) { }

    public update() {
        this.angle += this.angleOff;
        this.y += this.yOff;
        this.radius -= this.radiusDec;

        const alpha = 128 + 128 * Math.sin(this.angle);
        this.color.setAlpha(Math.max(alpha, 0));
    }

    public draw(p: p5) {
        p.fill(this.color);
        p.noStroke();

        p.ellipse(Math.cos(this.angle) * this.radius, this.y, 3);
    }
}
