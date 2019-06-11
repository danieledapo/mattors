import { ISketch } from "./sketch";

export class Blankets implements ISketch {
    public readonly name = "Blankets";

    public readonly width = 1920;
    public readonly height = 1080;
    public readonly loop = true;

    public readonly gridSize = 10;

    private t = 0;

    public reset(p: p5) {
        this.t = p.random();
        p.frameRate(2);

        p.background("black");
    }

    public draw(p: p5) {
        p.stroke(200, 200, 200, 10);

        this.drawSheet(p, 0.05);
    }

    public drawSheet(p: p5, div: number) {
        const startx = p.random(this.width);
        const starty = p.random(this.height);

        let t = this.t;
        for (let ti = 0; ti < 5; ti += 0.01) {
            let ptx = startx;
            let pty = starty;

            for (let i = 0; i < 500; i++) {
                const a = p.noise(ptx / this.gridSize * div, pty / this.gridSize * div, t) * p.TWO_PI;

                const nptx = ptx + Math.cos(a) * this.gridSize;
                const npty = pty + Math.sin(a) * this.gridSize;

                p.line(ptx, pty, nptx, npty);

                ptx = nptx;
                pty = npty;

                if (ptx < 0 || ptx > this.width || pty < 0 || pty > this.height) {
                    break;
                }
            }

            t += 0.001;
        }

    }
}
