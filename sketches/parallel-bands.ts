import { ISketch } from "./sketch";

type Line = [number, number, number, number];

export class ParallelBands implements ISketch {
    public readonly name = "Parallel Bands";

    public readonly width = 1920;
    public readonly height = 1080;
    public readonly loop = false;

    private rects: Rect[] = [];

    public reset(p: p5) {
        p.background("white");
        this.rects = [new Rect(0, 0, this.width, this.height)];
    }

    public draw(p: p5) {
        p.colorMode(p.HSB);
        const palette = [
            p.color(p.random(210, 360), 80, 100),
            p.color(p.random(210, 360), 80, 100),
            p.color(p.random(210, 360), 80, 100),
            p.color(p.random(210, 360), 80, 100),
            p.color(p.random(210, 360), 80, 100),
        ];

        for (let i = 0; i < 5; ++i) {
            const newRects = [];

            for (const r of this.rects) {
                let sub = r.subdivide(p);
                for (const s of sub) {
                    const ratio = s.width / s.height;
                    if (s.area() <= 1000 || ratio < 0.2 || ratio > 10) {
                        sub = [r];
                        break;
                    }
                }
                newRects.push(...sub);
            }

            this.rects = newRects;
        }

        for (const r of this.rects) {
            p.fill(p.random(palette));
            p.rect(r.x, r.y, r.width, r.height);
        }

        p.smooth();
        p.stroke(360, 0, 100);
        p.strokeWeight(6);
        for (const r of this.rects) {
            if (p.random() > 0.8) {
                continue;
            }

            const a = p.random(p.TWO_PI);
            const d = Math.pow(r.width / 2, 2) + Math.pow(r.height / 2, 2);

            const nparallel = 300;
            for (let di = 0; di < nparallel; ++di) {
                const t = di / (nparallel - 1);

                const cx = r.x + r.width / 2 + t * r.width;
                const cy = r.y + r.height / 2 + t * r.height;

                const line: Line = [
                    cx + Math.cos(a) * d,
                    cy + Math.sin(a) * d,
                    cx - Math.cos(a) * d,
                    cy - Math.sin(a) * d,
                ];

                const l = r.clip(...line);
                if (l !== null) {
                    p.line(...l);
                }
            }
        }

        // redraw borders
        p.stroke("black");
        p.strokeWeight(8);
        p.noFill();
        for (const r of this.rects) {
            p.rect(r.x, r.y, r.width, r.height);
        }
    }
}

class Rect {
    constructor(
        public readonly x: number,
        public readonly y: number,
        public readonly width: number,
        public readonly height: number,
    ) {}

    public area(): number {
        return this.width * this.height;
    }

    public subdivide(p: p5): Rect[] {
        const w = p.random(this.width);
        const h = p.random(this.height);

        return [
            new Rect(this.x, this.y, w, h),
            new Rect(this.x + w, this.y, this.width - w, h),
            new Rect(this.x, this.y + h, w, this.height - h),
            new Rect(this.x + w, this.y + h, this.width - w, this.height - h),
        ];
    }

    public clip(
        x0: number,
        y0: number,
        x1: number,
        y1: number,
    ): null | [number, number, number, number] {
        // mostly copied from wikipedia
        const INSIDE = 0;
        const LEFT = 1;
        const RIGHT = 2;
        const BOTTOM = 4;
        const TOP = 8;

        const getOutCode = (x: number, y: number): number => {
            let code = INSIDE;

            if (x < this.x) {
                code |= LEFT;
            }
            if (x > this.x + this.width) {
                code |= RIGHT;
            }

            if (y < this.y) {
                code |= BOTTOM;
            }
            if (y > this.y + this.height) {
                code |= TOP;
            }

            return code;
        };

        let accept = false;

        while (true) {
            let outcode0 = getOutCode(x0, y0);
            let outcode1 = getOutCode(x1, y1);

            if (outcode0 === 0 && outcode1 === 0) {
                // bitwise OR is 0: both points inside window; trivially accept and exit loop
                accept = true;
                break;
            }

            if ((outcode0 & outcode1) !== 0) {
                // bitwise AND is not 0: both points share an outside zone (LEFT, RIGHT, TOP,
                // or BOTTOM), so both must be outside window; exit loop (accept is false)
                break;
            }

            // failed both tests, so calculate the line segment to clip
            // from an outside point to an intersection with clip edge
            const outcodeOut = outcode0 !== 0 ? outcode0 : outcode1;

            let x = 0;
            let y = 0;

            const xmin = this.x;
            const xmax = this.x + this.width;
            const ymin = this.y;
            const ymax = this.y + this.height;

            if (outcodeOut & TOP) {
                // point is above the clip window
                x = x0 + ((x1 - x0) * (ymax - y0)) / (y1 - y0);
                y = ymax;
            } else if (outcodeOut & BOTTOM) {
                // point is below the clip window
                x = x0 + ((x1 - x0) * (ymin - y0)) / (y1 - y0);
                y = ymin;
            } else if (outcodeOut & RIGHT) {
                // point is to the right of clip window
                y = y0 + ((y1 - y0) * (xmax - x0)) / (x1 - x0);
                x = xmax;
            } else if (outcodeOut & LEFT) {
                // point is to the left of clip window
                y = y0 + ((y1 - y0) * (xmin - x0)) / (x1 - x0);
                x = xmin;
            }

            if (outcodeOut === outcode0) {
                x0 = x;
                y0 = y;
            } else {
                x1 = x;
                y1 = y;
            }
        }

        if (accept) {
            return [x0, y0, x1, y1];
        }

        return null;
    }
}
