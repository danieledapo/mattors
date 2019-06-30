import { ISketch } from "./sketch";

const goldenRatio = (1 + Math.sqrt(5)) / 2;

interface Triangle {
    a: p5.Vector;
    b: p5.Vector;
    c: p5.Vector;
    big: boolean;
}

export class PenroseTiling implements ISketch {
    public readonly name = "Penrose Tiling";

    public readonly width = 1920;
    public readonly height = 1080;
    public readonly loop = true;

    private triangles: Triangle[] = [];

    public reset(p: p5) {
        p.frameRate(1);

        p.background("white");

        this.triangles = [];

        for (let i = 0; i < 10; ++i) {
            const b = p5.Vector.fromAngle((2 * i - 1) * Math.PI / 10);
            const c = p5.Vector.fromAngle((2 * i + 1) * Math.PI / 10);

            if (i % 2 == 0) {
                this.triangles.push({
                    big: false,
                    a: p.createVector(),
                    b: c,
                    c: b,
                });
            } else {
                this.triangles.push({
                    big: false,
                    a: p.createVector(),
                    b,
                    c,
                });
            }
        }
    }

    public draw(p: p5) {
        p.push();
        p.background("white");
        p.translate(this.width / 2, this.height / 2);

        const scale = 1.2 * Math.sqrt(Math.pow(this.width / 2, 2) + Math.pow(this.height / 2, 2));

        for (const tri of this.triangles) {
            p.noStroke();
            if (tri.big) {
                p.fill(p.color(102, 102, 255));
            } else {
                p.fill(p.color(255, 90, 90));
            }

            p.beginShape();
            p.vertex(tri.a.x * scale, tri.a.y * scale);
            p.vertex(tri.b.x * scale, tri.b.y * scale);
            p.vertex(tri.c.x * scale, tri.c.y * scale);
            p.endShape(p.CLOSE);

            p.stroke(p.color(51, 51, 51));
            p.strokeWeight(3);
            p.noFill();
            p.line(tri.c.x * scale, tri.c.y * scale, tri.a.x * scale, tri.a.y * scale);
            p.line(tri.a.x * scale, tri.a.y * scale, tri.b.x * scale, tri.b.y * scale);
        }
        p.pop();

        if (this.triangles.length < 100000) {
            this.triangles = this.subdivide(this.triangles);
        }
    }

    private subdivide(triangles: Triangle[]): Triangle[] {
        const newTriangles = [];

        for (const tri of triangles) {
            if (tri.big) {
                const q = tri.b.copy().add(tri.a.copy().sub(tri.b).div(goldenRatio)); // Q = B + (A - B) / goldenRatio
                const r = tri.b.copy().add(tri.c.copy().sub(tri.b).div(goldenRatio)); // R = B + (C - B) / goldenRatio

                newTriangles.push(
                    { big: true, a: r, b: tri.c, c: tri.a },
                    { big: true, a: q, b: r, c: tri.b },
                    { big: false, a: r, b: q, c: tri.a },
                );
            } else {
                const p = tri.a.copy().add(tri.b.copy().sub(tri.a).div(goldenRatio));
                newTriangles.push(
                    { big: false, a: tri.c, b: p, c: tri.b },
                    { big: true, a: p, b: tri.c, c: tri.a },
                );
            }
        }

        return newTriangles;
    }

}
