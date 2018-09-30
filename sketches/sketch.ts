export interface ISketch {
    name: string;
    description: string;

    width: number;
    height: number;

    reset(p: p5): void;
    draw(p: p5): void;
}
