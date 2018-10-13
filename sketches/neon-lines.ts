import { ISketch } from "./sketch";

export class NeonLines implements ISketch {
    public readonly name = "Neon Lines";

    public readonly width = 600;
    public readonly height = 600;

    public readonly lines = 500;

    public reset(p: p5) {
        p.background("black");
    }

    public draw(p: p5) {
        p.strokeWeight(2);

        const astep = p.PI / 4;

        const rects = [
            [0, 0, this.width, this.height],
        ];

        let a = 0;
        for (let i = 0; i < this.lines; ++i) {
            const ri = Math.floor(p.random(0, rects.length));
            const [ox, oy, w, h] = rects[ri];

            const cx = ox + w / 2;
            const cy = oy + h / 2;

            const x = Math.cos(a) * w;
            const y = Math.sin(a) * h;

            p.stroke(p.random(255), p.random(255), p.random(255), 200);
            p.line(cx - x, cy - y, cx + x, cy + y);

            a = (a + astep) % p.TWO_PI;

            rects.splice(ri, 1);

            const subRects = [
                [ox, oy, w / 2, h / 2],
                [ox + w / 2, oy, w / 2, h / 2],
                [ox, oy + h / 2, w / 2, h / 2],
                [ox + w / 2, oy + h / 2, w / 2, h / 2],
            ];

            for (const subRect of subRects) {
                if (subRect[2] * subRect[3] > 100) {
                    rects.push(subRect);
                }
            }
        }
    }
}
