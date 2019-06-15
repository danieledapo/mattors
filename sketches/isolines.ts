import { ISketch } from "./sketch";

export class Isolines implements ISketch {
    public readonly name = "Isolines";

    public readonly width = 1920;
    public readonly height = 1080;
    public readonly loop = false;

    public readonly npoints = 20;

    private t = 0;
    private points: [number, number][] = [];
    private cx = 0;
    private cy = 0;

    private palette = [
        "#0575e6",
        "#0394c4",
        "#02b3a3",
        "#01d281",
        "#00f260",
    ];

    public reset(p: p5) {
        p.background(0x6, 0x55, 0x107);

        this.cx = p.random(40, this.width - 40);
        this.cy = p.random(40, this.height - 40);

        this.points = [];
        for (let i = 0; i < this.npoints; ++i) {
            let a = i / (this.npoints - 1) * p.TWO_PI;
            this.points.push([
                this.cx + Math.cos(a) * 30,
                this.cy + Math.sin(a) * 30
            ]);
        }

        this.points = this.deform(p);
    }

    public draw(p: p5) {
        let isoLines = [];
        for (let i = 0; i < 40; ++i) {
            isoLines.push(this.points);
            this.points = this.deform(p);
        }
        isoLines.reverse();

        p.stroke(0, 0, 0);

        let s = isoLines.length / this.palette.length;

        isoLines.forEach((pts, i) => {
            let c = this.palette[Math.floor(i / s)];
            p.fill(c);

            if (i % s == 0) {
                p.strokeWeight(3);
            } else {
                p.strokeWeight(1);
            }

            p.beginShape();
            for (const [x, y] of pts) {
                p.curveVertex(x, y);
            }
            for (let i = 1; i < 3; ++i) {
                p.curveVertex(pts[i][0], pts[i][1]);
            }
            p.endShape();
        });
    }

    private deform(p: p5): [number, number][] {
        let d: [number, number][] = this.points.map(pt => {
            let [x, y] = pt;

            let da = p.map(
                p.noise(x, y, this.t),
                0, 1,
                -p.QUARTER_PI / 4, p.QUARTER_PI / 4
            );
            let a = Math.atan2(y - this.cy, x - this.cx) + da;
            let r = p.random(20, 40);

            return [x + Math.cos(a) * r, y + Math.sin(a) * r];
        });

        this.t += 0.001;

        return d;
    }
}
