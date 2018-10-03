import { ISketch } from "./sketch";

export class BloodySpiderWeb implements ISketch {
    public readonly name = "Bloody spider web";
    public readonly description = "Bloody spider web";

    public readonly width = 1920;
    public readonly height = 1080;

    public readonly lines = 1000;

    public reset(p: p5) {
        p.background("black");
    }

    public draw(p: p5) {
        const moonRadius = this.width / 2;

        const moonCenterX = this.width / 2 + p.random(-moonRadius, moonRadius) / 2;
        const moonCenterY = this.height / 2 + p.random(-moonRadius, moonRadius) / 2;

        for (let i = 0; i < this.lines; ++i) {
            const [x1, y1] = this.randomPointInCircle(p, [moonCenterX, moonCenterY], moonRadius);
            const [x2, y2] = this.randomPointInCircle(p, [moonCenterX, moonCenterY], moonRadius);

            if (p.random() >= 0.8) {
                p.strokeWeight(p.random(1, 10));
                p.stroke(187, 10, 30, 10);
            } else {
                p.strokeWeight(p.random(1, 10));
                p.stroke(255, 255, 255, 10);
            }

            p.line(x1, y1, x2, y2);
        }
    }

    private randomPointInCircle(p: p5, [cx, cy]: [number, number], radius: number): [number, number] {
        const angle = p.random(0, p.TWO_PI);
        const r = p.random(0, radius);

        return [cx + r * Math.cos(angle), cy + r * Math.sin(angle)];
    }

}
