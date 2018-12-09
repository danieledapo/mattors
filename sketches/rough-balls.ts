import { ISketch } from "./sketch";

export class RoughBalls implements ISketch {
    public readonly name = "Rough Balls";

    public readonly width = 1300;
    public readonly height = 800;
    public readonly loop = false;

    public readonly radius = 200;

    public reset(p: p5) {
        p.background("white");
    }

    public draw(p: p5) {
        p.strokeWeight(5);

        const cx = this.width / 2;

        const balls = [
            {
                alpha: 10,
                iterations: 500,
                radius: this.radius / 4,
                xs: [cx - this.width / 12, cx, cx + this.width / 12],
                y: this.height / 8,
            },
            {
                alpha: 15,
                iterations: 1000,
                radius: this.radius / 2,
                xs: [cx - this.width / 6, cx, cx + this.width / 6],
                y: this.height / 8 * 3,
            },
            {
                alpha: 15,
                iterations: 5000,
                radius: this.radius,
                xs: [cx - this.width / 3, cx, cx + this.width / 3],
                y: this.height / 8 * 3 + this.height / 3,
            },
        ];

        for (const { radius, y, xs, iterations } of balls) {
            const pa = p.random();

            let planetsInRow = 1;
            if (pa >= 0.9) {
                planetsInRow = 3;
            } else if (pa >= 0.8) {
                planetsInRow = 2;
            }

            for (let i = 0; i < planetsInRow; ++i) {
                const x = p.random(xs);
                p.push();

                p.translate(x, y);
                this.drawBall(p, radius, iterations, 10);

                p.pop();
            }
        }
    }

    public drawBall(p: p5, radius: number, iterations: number, alpha: number) {
        for (let i = 0; i < iterations; ++i) {
            p.stroke(p.random(255), p.random(255), p.random(255), alpha);
            p.noFill();

            const a = p.random(p.TWO_PI);
            const sx = Math.cos(a) * radius;
            const sy = Math.sin(a) * radius;

            const ea = p.random(p.TWO_PI);
            const ex = Math.cos(ea) * radius;
            const ey = Math.sin(ea) * radius;

            const midx = (sx + ex) / 2;
            const midy = (sy + ey) / 2;

            p.strokeWeight((midx * midx + midy * midy) / (radius * radius) * 3);

            p.bezier(
                sx, sy,
                midx + p.random(-0.5, 0.5) * radius / 2, midy + p.random(-0.5, 0.5) * radius / 2,
                midx + p.random(-0.5, 0.5) * radius / 2, midy + p.random(-0.5, 0.5) * radius / 2,
                ex, ey,
            );
        }
    }
}
