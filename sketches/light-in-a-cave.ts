import { ISketch } from "./sketch";

export class LightInACave implements ISketch {
    public readonly name = "Light in a Cave";

    public readonly width = 1920;
    public readonly height = 1080;
    public readonly loop = true;

    private maxIterations = 0;
    private it = 0;

    public reset(p: p5) {
        p.background("black");
        p.stroke(p.random() * 255, p.random() * 255, p.random() * 255, 30);

        this.maxIterations = p.random(600, 800);
        this.it = 0;
    }

    public draw(p: p5) {
        if (this.it++ >= this.maxIterations) {
            return;
        }

        p.translate(this.width / 2, this.height / 2);

        this.drawSpiral(p);
    }

    private drawSpiral(p: p5) {
        p.noFill();

        let radius = 10;
        let angle = 0;

        const angleFac = p.random(2, 7);

        p.beginShape();

        while (true) {

            const x = Math.cos(angle) * radius;
            const y = Math.sin(angle) * radius;

            if (x <= -this.width / 2 || x >= this.width / 2 || y <= -this.height / 2 || y >= this.height / 2) {
                break;
            }

            p.vertex(x, y);

            radius += p.random(radius / 4);
            angle += p.PI / angleFac / (p.random(4) + 1);
        }

        p.endShape();
    }
}
