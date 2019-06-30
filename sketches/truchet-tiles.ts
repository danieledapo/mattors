import { ISketch } from "./sketch";

type Tile = "triangle" | "quad-circles";

export class TruchetTiles implements ISketch {
    public readonly name = "Truchet Tiles";

    public readonly width = 1920;
    public readonly height = 1080;
    public readonly loop = false;

    public readonly cellSize = 40;
    private tileType: Tile = "triangle";

    public reset(p: p5) {
        if (p.random() > 0.5) {
            this.tileType = "triangle";
        } else {
            this.tileType = "quad-circles";
        }
    }

    public draw(p: p5) {
        p.colorMode(p.HSB);

        const h1 = p.random(0, 360);
        p.background(h1, 80, 80);

        let h2 = (360 - h1 + 360) % 360;
        p.fill(h2, 80, 80);
        p.stroke(h2, 80, 80);

        switch (this.tileType) {
            case "triangle":
                this.drawTriangleTiles(p);
                break;
            case "quad-circles":
                this.drawQuadCircles(p);
                break;
        }
    }

    private drawTriangleTiles(p: p5) {
        p.noStroke();

        for (let y = 0; y < this.height; y += this.cellSize) {
            for (let x = 0; x < this.width; x += this.cellSize) {
                const t = p.random();

                if (t < 0.25) {
                    p.triangle(x, y, x + this.cellSize, y, x, y + this.cellSize);
                } else if (t < 0.5) {
                    p.triangle(x, y, x + this.cellSize, y, x + this.cellSize, y + this.cellSize);
                } else if (t < 0.75) {
                    p.triangle(x + this.cellSize, y, x + this.cellSize, y + this.cellSize, x, y + this.cellSize);
                } else {
                    p.triangle(x + this.cellSize, y + this.cellSize, x, y + this.cellSize, x, y);
                }
            }
        }
    }

    private drawQuadCircles(p: p5) {
        p.noFill();
        p.strokeWeight(10);

        for (let y = 0; y < this.height; y += this.cellSize) {
            for (let x = 0; x < this.width; x += this.cellSize) {
                const t = p.random();

                if (t > 0.5) {
                    p.arc(x, y, this.cellSize, this.cellSize, 0, p.HALF_PI);
                    p.arc(x + this.cellSize, y + this.cellSize, this.cellSize, this.cellSize, p.PI, -p.HALF_PI);
                } else {
                    p.arc(x + this.cellSize, y, this.cellSize, this.cellSize, p.HALF_PI, p.PI);
                    p.arc(x, y + this.cellSize, this.cellSize, this.cellSize, -p.HALF_PI, p.TWO_PI);
                }
            }
        }
    }
}
