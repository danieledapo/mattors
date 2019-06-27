import { ISketch } from "./sketch";

export class NeonLines implements ISketch {
    public readonly name = "Neon Lines";

    public readonly width = 1920;
    public readonly height = 1080;
    public readonly loop = true;

    public readonly gridSize = 40;

    public backgroundColor: p5.Color = "cannot happen" as unknown as p5.Color;

    public reset(p: p5) {
        this.backgroundColor = p.color("rgb(0,46,99)");
        p.background(this.backgroundColor);

        p.frameRate(3);
    }

    public draw(p: p5) {
        this.drawLine(p, this.backgroundColor);
    }

    private drawLine(p: p5, backgroundColor: p5.Color) {
        p.strokeWeight(5);

        const c = p.color(
            p.red(backgroundColor) + p.random(255 - p.red(backgroundColor)),
            p.blue(backgroundColor) + p.random(255 - p.blue(backgroundColor)),
            p.green(backgroundColor) + p.random(255 - p.green(backgroundColor)),
        );
        p.stroke(c);
        p.fill(c);
        p.strokeCap(p.PROJECT);

        let prev: [number, number] | null = null;

        for (const [x, y] of this.randomWalk(p, p.random(10, 50))) {
            if (prev !== null) {
                p.line(prev[0], prev[1], x, y);
            }

            prev = [x, y];
        }

        if (prev !== null) {
            if (p.random() > 0.5) {
                p.noFill();

                p.strokeWeight(p.random(3, 10));

                const s = p.random(50, 150);
                p.ellipse(prev[0], prev[1], s, s);
            } else {
                p.ellipse(prev[0], prev[1], 30, 30);
            }
        }
    }

    private * randomWalk(p: p5, n: number): Iterable<[number, number]> {
        const cellWidth = this.width / this.gridSize;
        const cellHeight = this.height / this.gridSize;

        // start on the left or top edge and walk towards bottom right
        let startx = 0;
        let starty = 0;
        if (p.random() > 0.5) {
            startx = Math.floor(p.random(0, this.gridSize)) * cellWidth;
        } else {
            starty = Math.floor(p.random(0, this.gridSize)) * cellHeight;
        }

        const pt: [number, number] = [startx, starty];

        for (let i = 0; i < n; ++i) {
            yield pt;

            if (p.random() > 0.2) {
                pt[0] += cellWidth;
                if (pt[0] >= this.width) {
                    break;
                }
            }

            if (p.random() > 0.2) {
                pt[1] += p.random([cellHeight]);
                if (pt[1] >= this.height) {
                    break;
                }
            }
        }
    }
}
