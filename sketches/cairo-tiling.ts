import {
    ISketch,
} from "./sketch";

interface Pentagon {
    cx: number,
    cy: number,
    size: number,
}

const sqrt3 = Math.sqrt(3);
const smallSideLen = sqrt3 - 1;

// https://en.wikipedia.org/wiki/Cairo_pentagonal_tiling
export class CairoTiling implements ISketch {
    public readonly name = "Cairo tiling";

    public readonly width = 1920;
    public readonly height = 1080;
    public readonly loop = true;

    private stack: Pentagon[] = [];
    private alreadySeen: Pentagon[] = [];

    private palettePoints: [number, number][] = [];
    private palette = [
        "#fdef96",
        "#f7b71d",
        "#afa939",
        "#2b580c",
    ];

    public reset(p: p5) {
        p.background("white");

        this.stack = [{
            cx: this.width / 2,
            cy: this.height / 2,
            size: 20,
        }];

        this.alreadySeen.splice(0, this.alreadySeen.length);
        this.palettePoints.splice(0, this.palettePoints.length);

        const nPalettePoints = 3;
        for (let i = 0; i < nPalettePoints; ++i) {
            this.palettePoints.push([
                i / nPalettePoints * this.width + p.random(this.width / (nPalettePoints - 1)),
                p.random(this.height)
            ]);
        }

        // this.palettePoints = [[this.width / 2, this.height / 2]];
    }

    public draw(p: p5) {
        while (this.stack.length > 0) {
            const c = this.stack.pop()!;
            const { cx, cy, size } = c;

            if (this.isPentagonHidden(c)) {
                continue;
            }

            if (this.hasSeen(cx, cy)) {
                continue;
            }
            this.alreadySeen.push(c);

            this.drawTile(p, c);

            const sl = size * smallSideLen;
            const sh = (sl / 2 * sqrt3);
            this.stack.push(
                {
                    cx: cx + (2 * size) + (sl / 2),
                    cy: cy - sh,
                    size,
                },
                {
                    cx: cx - (2 * size) - (sl / 2),
                    cy: cy + sh,
                    size,
                },
                {
                    cx: cx + sh,
                    cy: cy + 2 * size + sl / 2,
                    size,
                },
                {
                    cx: cx - sh,
                    cy: cy - 2 * size - sl / 2,
                    size,
                },

            );

            break;
        }
    }

    private drawTile(p: p5, pentagon: Pentagon) {
        let closestX = 0;
        let closestY = 0;
        let mind = Infinity;
        for (const [x, y] of this.palettePoints) {
            let d = Math.abs(x - pentagon.cx) + Math.abs(y - pentagon.cy);
            if (d < mind) {
                mind = d;
                closestX = x;
                closestY = y;
            }
        }

        let maxD = Math.abs(closestX + closestY);
        let t = mind / maxD;
        let f = this.palette[Math.min(Math.floor(t * this.palette.length), this.palette.length - 1)];

        p.push();
        p.fill(f);
        p.strokeWeight(3);
        p.translate(pentagon.cx, pentagon.cy);
        for (let i = 0; i < 4; ++i) {
            p.rotate(p.HALF_PI);
            this.drawPentagon(p, pentagon.size);
        }
        p.pop();
    }

    private drawPentagon(p: p5, size: number) {
        const points = [
            [0, 0],
            [0, -1],
            [-smallSideLen / 2 * sqrt3, -1 - smallSideLen / 2],
            [-1.5, -sqrt3 / 2],
            [-1, 0]
        ];

        p.beginShape();
        for (const [x, y] of points) {
            p.vertex(x * size, y * size);
        }
        p.endShape(p.CLOSE);
    }

    private hasSeen(cx: number, cy: number): boolean {
        const ix = this.alreadySeen.findIndex((p) => {
            return Math.abs(p.cx - cx) < 1e-6 && Math.abs(p.cy - cy) < 1e-6;
        });
        return ix >= 0;
    }

    private isPentagonHidden(c: Pentagon): boolean {
        const { cx, cy, size } = c;

        // i'm too lazy to do the correct math to know the proper max delta,
        // 1.5 is enough...
        const delta = size * 1.5;
        return (cx + delta < 0 || cy + delta < 0 || cx - delta > this.width || cy - delta > this.height);
    }
}
