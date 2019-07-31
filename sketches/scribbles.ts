import { ISketch } from "./sketch";

interface Particle {
    position: p5.Vector;
    velocity: p5.Vector;
    acceleration: number;
    angle: number;
    path: p5.Vector[];
    alive: boolean;
}

interface Container {
    contains: (x: number, y: number) => boolean;
    random: (p: p5) => [number, number];
}

class Rect implements Container {
    constructor(
        public readonly x: number,
        public readonly y: number,
        public readonly width: number,
        public readonly height: number) {}

    public contains(x: number, y: number): boolean {
        return this.x <= x && this.y <= y && this.x + this.width >= x && this.y + this.height >= y;
    }

    public random(p: p5): [number, number] {
        return [p.random(this.x, this.x + this.width), p.random(this.y, this.y + this.height)];
    }

}

class Circle implements Container {
    constructor(
        public readonly x: number,
        public readonly y: number,
        public readonly radius: number) {}

    public contains(x: number, y: number): boolean {
        return Math.pow(this.x - x, 2) + Math.pow(this.y - y, 2) <= Math.pow(this.radius, 2);
    }

    public random(p: p5): [number, number] {
        const a = p.random(p.TWO_PI);
        const r = p.random(this.radius);

        return [this.x + Math.cos(a) * r, this.y + Math.sin(a) * r];
    }

}

export class Scribbles implements ISketch {
    public readonly name = "Scribbles";

    public readonly width = 1920;
    public readonly height = 1080;
    public readonly loop = true;

    private particles: Particle[] = [];
    private container: Container = new Rect(0,0,0,0);

    public reset(p: p5) {
        p.colorMode(p.HSB);

        const cp = p.random();

        if (cp > 0.5) {
            this.container = new Rect(0, 0, this.width, this.height);
        } else {
            this.container = new Circle(this.width/2, this.height/2, 300);
        }

        this.particles = [];

        const n = p.random(1, 9);
        for (let i = 0; i < n; ++i) {
            this.particles.push(this.genParticle(p, ...this.container.random(p)));
        }
    }

    public draw(p: p5) {
        p.background("white");
        p.stroke(0, 0, 0);
        p.noFill();

        p.strokeWeight(5);
        p.rect(0, 0, this.width, this.height);
        p.strokeWeight(1);

        for (const particle of this.particles) {
            if (particle.alive && !this.container.contains(particle.position.x, particle.position.y)) {
                particle.alive = false;
                this.particles.push(this.genParticle(p, ...this.container.random(p)));
            }

            if (particle.alive) {
                particle.path.push(particle.position.copy());

                particle.acceleration += p.random(-0.01, 0.011);
                particle.angle = p.randomGaussian(0, 1);

                particle.velocity.add(particle.acceleration).rotate(particle.angle);
                particle.velocity.limit(10);

                particle.position.add(particle.velocity);
            }

            p.beginShape();
            for (const pos of particle.path) {
                p.curveVertex(pos.x, pos.y);
            }
            p.endShape();

        }
    }

    private genParticle(p: p5, x: number, y: number): Particle {
        return {
            acceleration: p.random(),
            alive: true,
            angle: p.random(p.TWO_PI),
            path: [],
            position: p.createVector(x, y),
            velocity: p5.Vector.random2D(),
        };
    }
}
