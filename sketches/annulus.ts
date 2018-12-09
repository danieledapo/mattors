import { ISketch } from "./sketch";

export class Annulus implements ISketch {
    public readonly name = "Annulus";

    public readonly width = 1920;
    public readonly height = 1080;
    public readonly loop = false;

    // expose to the ui
    public readonly nArcs = 1000;

    public reset(p: p5) {
        p.background("black");
    }

    public draw(p: p5) {

        const center: [number, number] = [this.width / 2, this.height / 2];

        const palette = [
            // "#a9e5bb",
            "#fcf6b1",
            "#f7b32b",
            "#f72c25",
            "#2d1e2f",
        ];

        p.noFill();

        let radius = 0;

        for (let i = 0; i < this.nArcs; ++i) {
            const color = palette[Math.floor(p.random(0, palette.length))];
            p.stroke(color);

            const newRadius = radius + p.random(1, 8);

            p.strokeWeight(newRadius - radius);

            const startAngle = p.random(0, p.TWO_PI);
            const endAngle = p.random(0, p.TWO_PI);

            p.arc(center[0], center[1], newRadius, newRadius, startAngle, endAngle);

            radius = newRadius;
        }
    }

}
