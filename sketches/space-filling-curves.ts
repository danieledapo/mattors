import { ISketch } from "./sketch";

export class SpaceFillingCurves implements ISketch {
    public readonly name = "Space Filling Curves";

    public readonly width = 800;
    public readonly height = 800;
    public readonly loop = false;

    private readonly padding = 10;

    public reset(p: p5) {
        p.background(80, 80, 80);
    }

    public draw(p: p5) {
        p.stroke(200, 200, 200);

        const size = this.width / 2 - 2 * this.padding;

        p.push();
        p.translate(this.padding, this.padding);
        this.drawQuad(p, size, 1);
        p.pop();

        p.push();
        p.translate(this.width / 2 + this.padding, this.padding);
        this.drawQuad(p, size, 1);
        p.pop();

        p.push();
        p.translate(this.padding, this.height / 2 + this.padding);
        this.drawQuad(p, size, 1);
        p.pop();

        p.push();
        p.translate(this.width / 2 + this.padding, this.height / 2 + this.padding);
        this.drawQuad(p, size, 1);
        p.pop();
    }

    private drawQuad(p: p5, size: number, depth: number) {
        if (p.random() <= depth / 3) {
            const curves = [
                new HilbertCurve(),
                new HilbertCurve2(),
                new PeanoGosperCurve(),
            ];

            const c: Curve = p.random(curves);
            const gens = c.randGen(depth);

            // PeanoGosperCurve doesn't start at the top left, adjust translation
            if (c instanceof PeanoGosperCurve) {
                const sl = c.strokeLen(gens, size);
                const hl = Math.sqrt(3) / 2 * sl;

                // I wasn't able to figure out the math to properly translate
                // the curve, eyeball it. |- T -|
                p.translate(
                    hl * ((depth === 1) ? 14 : 2),
                    hl * ((depth === 1) ? 14 : 7),
                );
            }

            this.drawLSystem(p, c.advance(gens), c.strokeLen(gens, size));
            return;
        }

        const np = this.padding / depth;
        const s = size / 2 - np * 2;

        p.push();
        p.translate(np, np);
        this.drawQuad(p, s, depth + 1);
        p.pop();

        p.push();
        p.translate(size / 2 + np, np);
        this.drawQuad(p, s, depth + 1);
        p.pop();

        p.push();
        p.translate(np, size / 2 + np);
        this.drawQuad(p, s, depth + 1);
        p.pop();

        p.push();
        p.translate(size / 2 + np, size / 2 + np);
        this.drawQuad(p, s, depth + 1);
        p.pop();
    }

    private drawLSystem(p: p5, lSys: LSystem, w: number) {
        for (const c of lSys.state) {
            switch (c) {
                case "+":
                    p.rotate(lSys.theta);
                    break;
                case "-":
                    p.rotate(-lSys.theta);
                    break;
                case "F":
                    p.line(0, 0, w, 0);
                    p.translate(w, 0);
                    break;
            }
        }
    }
}

export class LSystem {
    constructor(
        public readonly state: string,
        public readonly rules: Map<string, string>,
        public readonly theta: number = Math.PI / 2,
    ) { }

    public advance(gens: number = 1): LSystem {
        if (gens <= 0) {
            return this;
        }

        let newState = "";

        for (const c of this.state) {
            let nc = this.rules.get(c);
            if (nc === undefined) {
                nc = c;
            }

            newState += nc;

        }

        const lSys = new LSystem(newState, this.rules, this.theta);
        return lSys.advance(gens - 1);
    }
}

export type Curve = HilbertCurve | HilbertCurve2 | PeanoGosperCurve;

export class HilbertCurve extends LSystem {
    constructor() {
        super("L", new Map([
            ["L", "+RF-LFL-FR+"],
            ["R", "-LF+RFR+FL-"],
        ]));
    }

    public strokeLen(gens: number, size: number): number {
        return size / (Math.pow(2, gens) - 1);
    }

    public randGen(depth: number): number {
        return Math.floor(2 + Math.random() * (7 - depth));
    }
}

export class HilbertCurve2 extends LSystem {
    constructor() {
        super("X", new Map([
            ["X", "XFYFX+F+YFXFY-F-XFYFX"],
            ["Y", "YFXFY-F-XFYFX+F+YFXFY"],
        ]));
    }

    public strokeLen(gens: number, size: number): number {
        return size / (Math.pow(3, gens) - 1);
    }

    public randGen(depth: number): number {
        return Math.floor(2 + Math.random() * (4 - depth));
    }
}

export class PeanoGosperCurve extends LSystem {
    constructor() {
        super(
            "Y",
            new Map([
                ["X", "X+YF++YF-FX--FXFX-YF+"],
                ["Y", "-FX+YFYF++YF+FX--FX-Y"],
            ]),
            Math.PI / 3,
        );
    }

    public strokeLen(gens: number, size: number): number {
        return size / (Math.pow(2.82, gens) - 1);
    }

    public randGen(depth: number): number {
        return 3 + Math.max(0, 2 - depth);
    }
}
