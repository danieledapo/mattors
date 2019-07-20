import { ISketch } from "./sketch";

interface Particle {
    position: p5.Vector;
    radius: number;
    parent: null | Particle;
}

class Bbox {
    constructor(public readonly lower: p5.Vector, public readonly upper: p5.Vector) {}

    public expand(p: p5.Vector) {
        this.lower.x = Math.min(this.lower.x, p.x);
        this.lower.y = Math.min(this.lower.y, p.y);

        this.upper.x = Math.max(this.upper.x, p.x);
        this.upper.y = Math.max(this.upper.y, p.y);
    }

    public contains(p: p5.Vector): boolean {
        return (
            this.lower.x <= p.x && this.upper.x >= p.x && this.lower.y <= p.y && this.upper.y >= p.y
        );
    }

    public copy(): Bbox {
        return new Bbox(this.lower.copy(), this.upper.copy());
    }

    public enlarge(r: number) {
        this.lower.x -= r;
        this.lower.y -= r;

        this.upper.x += r;
        this.upper.y += r;
    }

    public center(): p5.Vector {
        return this.lower
            .copy()
            .add(this.upper)
            .div(2);
    }
}

export class Dla implements ISketch {
    public readonly name = "Diffusion limited aggregation";

    public readonly width = 1920;
    public readonly height = 1080;
    public readonly loop = true;

    private particles: Particle[] = [];
    private particlesBbox: Bbox = new Bbox(new p5.Vector(), new p5.Vector());
    private containerRadius = 0;
    private stickiness = 1;

    public reset(p: p5) {
        p.background("black");

        p.noFill();
        p.colorMode(p.HSB);
        p.stroke(p.random(360), 50, 50, 0.4);

        this.particles = [
            {
                position: p.createVector(),
                radius: 16,
                parent: null,
            },
        ];

        this.particlesBbox = new Bbox(
            p.createVector(Infinity, Infinity),
            p.createVector(-Infinity, -Infinity),
        );
        for (const pa of this.particles) {
            this.particlesBbox.expand(pa.position);
        }

        this.containerRadius = Math.min(this.width / 2, this.height / 2) - 50 * 2;
        this.stickiness = p.random(1);
    }

    public draw(p: p5) {
        p.push();
        p.translate(this.width / 2, this.height / 2);

        const spawnBbox = this.particlesBbox.copy();
        spawnBbox.enlarge(50);

        let particle: Particle | undefined = undefined;

        while (true) {
            if (particle === undefined) {
                particle = {
                    position: p.createVector(
                        p.random(spawnBbox.lower.x, spawnBbox.upper.x),
                        p.random(spawnBbox.lower.y, spawnBbox.upper.y),
                    ),
                    radius: 8,
                    parent: null,
                };

                particle.position.limit(this.containerRadius);
            }

            if (!spawnBbox.contains(particle.position)) {
                particle = undefined;
                continue;
            }

            const neighbor = this.particles.find(e => {
                return e.position.dist(particle!.position) < e.radius + particle!.radius;
            });

            if (neighbor === undefined || p.random() > this.stickiness) {
                particle.position.add(p5.Vector.random2D().mult(particle.radius));
                continue;
            }

            particle.parent = neighbor;

            this.particles.push(particle);
            this.particlesBbox.expand(particle.position);

            p.ellipse(particle.position.x, particle.position.y, particle.radius, particle.radius);

            p.beginShape();
            let pa: Particle | null = particle;
            while (pa != null) {
                p.curveVertex(pa.position.x, pa.position.y);
                pa = pa.parent;
            }
            p.endShape();

            break;
        }

        p.pop();
    }
}
