import { ISketch } from "./sketch";

// https://10print.org/
export class Print10 implements ISketch {
    public readonly name = "10Print";
    public readonly description = "10 Print replica";

    public readonly width = 600;
    public readonly height = 600;

    // this should be configurable from a ui
    public readonly step = 20;

    public reset(p: p5) {
        p.background("white");
    }

    public draw(p: p5) {
        p.strokeWeight(3);

        for (let x = 0; x < this.width; x += this.step) {
            for (let y = 0; y < this.height; y += this.step) {

                if (Math.random() < 0.5) {
                    p.line(x, y, x + this.step, y + this.step);
                } else {
                    p.line(x + this.step, y, x, y + this.step);
                }

            }
        }
    }

}
