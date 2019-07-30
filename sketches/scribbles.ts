import { ISketch } from "./sketch";

interface Particle {
    position: p5.Vector;
    velocity: p5.Vector;
    acceleration: number;
    angle: number;
    path: p5.Vector[];
    alive: boolean;
}

export class Scribbles implements ISketch {
    public readonly name = "Scribbles";

    public readonly width = 1920;
    public readonly height = 1080;
    public readonly loop = true;

    private particles: Particle[] = [];

    public reset(p: p5) {
        p.colorMode(p.HSB);

        this.particles = [
            this.genParticle(p, this.width / 2, this.height / 2),
        ];

        const n = p.random(8);
        for (let i = 0; i < n; ++i) {
            this.particles.push(this.genParticle(p));

        }
    }

    public draw(p: p5) {
        p.background("white");
        p.stroke(0, 0, 0);
        p.noFill();

        p.strokeWeight(5);
        p.rect(0, 0, this.width, this.height);
        // p.ellipse(this.width/2, this.height/2, 600, 600);
        p.strokeWeight(1);

        for (const particle of this.particles) {
            if (particle.alive &&
                // (particle.position.x < 0 || particle.position.x > this.width
                //     || particle.position.y < 0 || particle.position.y > this.height)
                (particle.position.dist(p.createVector(this.width/2, this.height/2)) > 300)
            ) {
                particle.alive = false;
                this.particles.push(this.genParticle(p));
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

    private genParticle(p: p5, x?: number, y?: number): Particle {
        x = (x !== undefined) ? x : p.random(this.width);
        y = (y !== undefined) ? y : p.random(this.height);

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
