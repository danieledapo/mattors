import { ISketch } from "./sketch";

export class FocusEye implements ISketch {
    public readonly name = "Focus Eye";

    public readonly width = 1920;
    public readonly height = 1080;
    public readonly loop = true;

    private t = 0;
    private focusPoint = new p5.Vector(0, 0);

    public reset(p: p5) {
        p.background("black");
        p.frameRate(10);

        this.focusPoint = p.createVector(
            p.random(20, this.width - 20),
            p.random(20, this.height - 20),
        );
    }

    public draw(p: p5) {
        const focusPoint = this.focusPoint;

        p.strokeWeight(0.5);
        p.stroke("white");
        p.fill("white");

        const gridSize = p.random(1, 5);
        for (let x = 0; x <= this.width; x += this.width / gridSize) {
            this.slowLine(p, p.createVector(x, 0), focusPoint);
            this.slowLine(p, p.createVector(x, this.height), focusPoint);
        }
        for (let y = 0; y <= this.height; y += this.height / gridSize) {
            this.slowLine(p, p.createVector(0, y), focusPoint);
            this.slowLine(p, p.createVector(this.width, y), focusPoint);
        }

        p.noFill();
        p.stroke("white");
        p.strokeWeight(10);
        p.rect(0, 0, this.width, this.height);

        const a = p.noise(this.t) * p.TWO_PI;
        focusPoint.add(p5.Vector.fromAngle(a).mult(5));
        this.t += 0.01;
    }

    private slowLine(p: p5, a: p5.Vector, b: p5.Vector) {
        if (p.random() > 0.7) {
            return;
        }

        const d = b.copy().sub(a);
        const l = d.mag();
        d.normalize();

        const ellipseRad = 1;

        for (let cl = 0; cl < l; cl += ellipseRad) {
            const c = a.copy().add(d.copy().mult(cl));

            const alpha = p.lerp(10, 50, cl / (l - ellipseRad));
            p.fill(255, 255, 255, alpha);
            p.stroke(255, 255, 255, alpha);

            p.ellipse(c.x, c.y, ellipseRad * 2, ellipseRad * 2);
        }
    }
}
