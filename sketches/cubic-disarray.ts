import {
    ISketch,
} from "./sketch";

// Port of George Ness Cubic Disarray piece
export class CubicDisarray implements ISketch {
    public readonly name = "Cubic disarray";
    public readonly description = "Port of Cubic Disarray by George Ness";

    public readonly borderMargin = 30;
    public readonly width = 600;
    public readonly height = 720;

    public readonly cols = 20;
    public readonly rows = 24;

    public readonly crazinessPerRow = 3;

    public reset(p: p5) {
        p.background("white");
    }

    public draw(p: p5) {
        p.noFill();
        p.rectMode(p.CENTER);

        p.push();

        p.translate(this.borderMargin, this.borderMargin);

        const rectWidth = (this.width - this.borderMargin * 2) / this.cols;
        const rectHeight = (this.height - this.borderMargin * 2) / this.rows;

        let cols = this.cols;

        for (let y = 0; y < this.rows; ++y) {
            const rowWeight = y / (this.rows * this.crazinessPerRow);

            for (let x = 0; x < cols; ++x) {
                p.push();

                p.translate(
                    x * rectWidth + rectWidth / 2,
                    y * rectHeight + rectHeight / 2,
                );

                this.drawRect(p, rectWidth, rectHeight, rowWeight);

                p.pop();
            }

            cols -= y / this.rows;
        }

        p.pop();
    }

    private drawRect(p: p5, rectWidth: number, rectHeight: number, rowWeight: number) {
        p.translate(
            p.random(-rectWidth * rowWeight, rectWidth * rowWeight),
            p.random(-rectHeight * rowWeight, rectHeight * rowWeight),
        );

        const rotation = p.random(-p.HALF_PI * rowWeight, p.HALF_PI * rowWeight);
        p.rotate(rotation);

        p.rect(0, 0, rectWidth - 1, rectHeight - 1);
    }
}
