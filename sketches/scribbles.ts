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

class Triangle implements Container {
    constructor(
        public readonly a: p5.Vector,
        public readonly b: p5.Vector,
        public readonly c: p5.Vector) {}

    public contains(x: number, y: number): boolean {
        const sign = (p1: p5.Vector, p2: p5.Vector, p3: p5.Vector) => {
            return (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y);
        };

        const xy = this.a.copy();
        xy.set(x, y);

        const d1 = sign(xy, this.a, this.b);
        const d2 = sign(xy, this.b, this.c);
        const d3 = sign(xy, this.c, this.a);

        const has_neg = (d1 < 0) || (d2 < 0) || (d3 < 0);
        const has_pos = (d1 > 0) || (d2 > 0) || (d3 > 0);

        return !(has_neg && has_pos);
    }

    public random(p: p5): [number, number] {
        const r1 = p.random();
        const r2 = p.random();

        const pt = this.a.copy().mult(1 - Math.sqrt(r1))
            .add(this.b.copy().mult((1 - r2) * Math.sqrt(r1)))
            .add(this.c.copy().mult(r2 * Math.sqrt(r1)));

        return [pt.x, pt.y];
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

        if (cp < 0.33) {
            this.container = new Rect(0, 0, this.width, this.height);
        } else if (cp < 0.66) {
            this.container = new Circle(this.width/2, this.height/2, 300);
        } else {
            const t = p.createVector(p.random(this.width/2), p.random(this.height/2));
            const l = p.createVector(p.random(this.width/2, this.width),
                                     p.random(this.height/2));
            // c   = (t + l + r) / 3 =>
            // 3*c = t + l + r       =>
            // r   = 3*c - t - l
            const r = p.createVector(this.width/2, this.height/2).mult(3).sub(t).sub(l);
            this.container = new Triangle(t, l, r);
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
