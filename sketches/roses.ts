import { ISketch } from "./sketch";

type Point = [number, number];
type Shape = Point[];

export class Roses implements ISketch {
    public readonly name = "Roses";

    public readonly width = 1920;
    public readonly height = 1080;
    public readonly loop = true;

    private readonly npoints = 40;
    private shapes: Shape[] = [];
    private t = 0;

    public reset(p: p5) {
        p.background("black");
        p.frameRate(10);

        const genCircle = (r: number) => {
            let shape: Shape = [];
            for (let i = 0; i < this.npoints; ++i) {
                let a = i / (this.npoints - 1) * p.TWO_PI;
                shape.push([
                    Math.cos(a) * r,
                    Math.sin(a) * r
                ]);
            }
            return shape;
        };

        let r = Math.min(this.height, this.width) / 3;
        this.shapes = [
            // genCircle(r),
            // genCircle(r * 0.75),
            // genCircle(r * 0.50),
            genCircle(r * 0.25),
            genCircle(r * 0.2),
        ];
    }

    public draw(p: p5) {
        let mutated: Shape[] = [];

        for (let si = 0; si < this.shapes.length; ++si) {
            const s = this.shapes[si];

            for (let g = 0; g < 10; ++g) {
                mutated[si] = s.map(([x, y]) => {
                    let f = p.noise(x, y, this.t);

                    let a = Math.atan2(y, x);
                    let da = p.map(f, 0, 1, -p.QUARTER_PI/4, p.QUARTER_PI/4);
                    let r = p.map(f, 0, 1, 20, 80);

                    return [x + Math.cos(a + da) * r, y + Math.sin(a + da) * r];
                });

                p.push();
                p.translate(this.width / 2, this.height / 2);
                p.stroke("#c21e560a");
                p.noFill();

                p.beginShape();
                for (const [x, y] of mutated[si]) {
                    p.curveVertex(x, y);
                }
                for (let i = 1; i < 3; ++i) {
                    p.curveVertex(mutated[si][i][0], mutated[si][i][1]);
                }
                p.endShape();
                p.pop();

                this.t += 0.01;
            }

            if (p.random() > 0.9) {
                this.shapes = mutated;
            }
        }
    }
}
