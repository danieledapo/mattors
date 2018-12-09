export interface ISketch {
    name: string;

    width: number;
    height: number;

    loop: boolean;

    reset(p: p5): void;
    draw(p: p5): void;
}
