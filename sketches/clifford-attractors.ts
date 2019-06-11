import { ISketch } from "./sketch";

export class CliffordAttractors implements ISketch {
    public readonly name = "Clifford Attractors";

    public readonly width = 1920;
    public readonly height = 1080;
    public readonly loop = true;

    private a = -1.4;
    private b = 1.6;
    private c = 1;
    private d = 0.7;

    private x = 0;
    private y = 0;
    private alreadySeen = new Set<string>();

    public reset(p: p5) {
        p.background("black");

        // this.a = -2;
        // this.b = 1.6;
        // this.c = -2;
        // this.d = 0.7;

        this.alreadySeen.clear();

        this.a = p.random([1, -1]) * p.random(1, 2);
        this.b = p.random([1, -1]) * p.random(1, 2);
        this.c = p.random([1, -1]) * p.random(1, 2);
        this.d = p.random([1, -1]) * p.random(1, 2);

        console.log(">>> clifford attractors: parameters", this.a, this.b, this.c, this.d);
    }

    public draw(p: p5) {
        p.translate(this.width / 2, this.height / 2);

        for (let i = 0; i < 500; ++i) {
            const nx = (Math.sin(this.a * this.y) + this.c * Math.cos(this.a * this.x));
            const ny = (Math.sin(this.b * this.x) + this.d * Math.cos(this.b * this.y));

            p.stroke(255, 255, 255, 50);
            p.fill(255, 255, 255, 50);

            p.point(nx * 200, ny * 200);

            // some attractors form very small loops which are not particularly
            // interesting to render, just skip them
            if (this.alreadySeen.has([nx, ny].toString()) && this.alreadySeen.size < 500) {
                console.log(">>> clifford attractors: skipping because it didn't have enough variance");
                this.reset(p);
                return;
            }

            this.alreadySeen.add([nx, ny].toString());

            this.x = nx;
            this.y = ny;
        }
    }
}
