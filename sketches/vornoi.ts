import { ISketch } from "./sketch";

export class Voronoi implements ISketch {
    public readonly name = "Voronoi";

    public readonly width = 1920;
    public readonly height = 1080;
    public readonly loop = false;

    private pivots: { pos: p5.Vector; color: p5.Color }[] = [];

    public reset(p: p5) {
        this.pivots = [];

        const n = p.random(10, 50);
        for (let i = 0; i < n; ++i) {
            const pv = p.createVector(p.random(this.width), p.random(this.height));

            this.pivots.push({
                pos: pv,
                color: p.color(
                    p.random([
                        "#5f8dd3",
                        "#438e90",
                        "#4f643a",
                        "#7b8db9",
                        "#64a943",
                        "#3ca64b",
                        "#99b8c9",
                        "#83c066",
                        "#ebbe75",
                        "#c44da2",
                        "#937218",
                        "#9c531e",
                        "#435596",
                        "#9a27cc",
                        "#d04143",
                        "#387389",
                        "#5d48d4",
                        "#6faea2",
                        "#d89d4b",
                        "#337fb9",
                    ]),
                ),
            });
        }
    }

    public draw(p: p5) {
        p.background("white");

        p.loadPixels();

        for (let y = 0; y < this.height; ++y) {
            for (let x = 0; x < this.width; ++x) {
                let minD = Infinity;
                let col: p5.Color;
                for (const pivot of this.pivots) {
                    const d = Math.abs(pivot.pos.x - x) + Math.abs(pivot.pos.y - y);
                    if (d < minD) {
                        minD = d;
                        col = pivot.color;
                    }
                }

                p.set(x, y, col!);
            }
        }

        p.updatePixels();
    }
}
