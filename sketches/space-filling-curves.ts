import { ISketch } from "./sketch";

export class SpaceFillingCurves implements ISketch {
    public readonly name = "Space Filling Curves";

    public readonly width = 800;
    public readonly height = 800;
    public readonly loop = false;

    private readonly padding = 10;

    public reset(p: p5) {
        p.background("white");
        // p.background("gray");
    }

    public draw(p: p5) {
        const size = this.width / 2 - 2 * this.padding;

        const pgc = new PeanoGosperCurve();

        p.push();
        p.translate(this.padding, this.padding);
        this.drawQuad(p, size, 1);
        p.pop();

        p.push();
        p.translate(this.width / 2 + this.padding, this.padding);
        this.drawQuad(p, size, 1);
        p.pop();

        p.push();
        p.translate(this.width * 0.09, this.height * 0.58);
        this.drawLSystem(p, pgc.advance(4), pgc.strokeLen(4, size));
        p.pop();

        p.push();
        p.translate(this.width / 2 + this.padding, this.height / 2 + this.padding);
        this.drawQuad(p, size, 1);
        p.pop();
    }

    private drawQuad(p: p5, size: number, depth: number) {
        if (p.random() <= depth / 4) {
            const curves = [new HilbertCurve(), new HilbertCurve2()];
            const c = p.random(curves);
            const gens = Math.floor(p.random(2, 6 - depth));

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
        return size / gens * 0.07;
    }
}
