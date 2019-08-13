import { ISketch } from "./sketch";

type SortingAlgo = (arr: number[]) => Iterable<number[]>;

export class Sorting implements ISketch {
    public readonly name = "Sorting Algorithms";

    public readonly width = 1920;
    public readonly height = 1080;
    public readonly loop = false;

    private sortAlgo: SortingAlgo = selectionSort;

    public reset(p: p5) {
        p.background("white");

        this.sortAlgo = p.random([selectionSort, bubbleSort, quickSort]);
    }

    public draw(p: p5) {
        const n = Math.floor(p.random(2, 150));

        const numbers: number[] = [];
        for (let i = 0; i < n; ++i) {
            numbers.push(p.random());
        }

        const passes = [numbers.slice()];
        for (const step of this.sortAlgo(numbers)) {
            passes.push(step.slice());
        }

        for (let i = 0; i < passes[passes.length - 1].length - 1; ++i) {
            if (passes[passes.length - 1][i] > passes[passes.length - 1][i + 1]) {
                console.warn("not sorted", this.sortAlgo.toString());
            }
        }

        p.noStroke();
        p.colorMode(p.HSB);
        const h1 = p.random(360);
        const h2 = (h1 + 180) % 360;

        const h = this.height / passes.length;
        const w = this.width / n;
        for (let pa = 0; pa < passes.length; ++pa) {
            const pass = passes[pa];

            for (let i = 0; i < pass.length; ++i) {
                p.fill(p.lerp(h1, h2, pass[i]), 80, 80);
                p.rect(w * i, pa * h, w + 1, h + 1);
            }
        }
    }
}

function* selectionSort(numbers: number[]): Iterable<number[]> {
    for (let i = 0; i < numbers.length - 1; i++) {
        let jMin = i;

        for (let j = i + 1; j < numbers.length; j++) {
            if (numbers[j] < numbers[jMin]) {
                jMin = j;
            }
        }

        if (jMin != i) {
            swap(numbers, i, jMin);
        }

        yield numbers;
    }
}

function* bubbleSort(numbers: number[]): Iterable<number[]> {
    let n = numbers.length;

    do {
        let newn = 0;
        for (let i = 1; i < n; ++i) {
            if (numbers[i - 1] > numbers[i]) {
                swap(numbers, i - 1, i);
                newn = i;
            }
        }
        n = newn;

        yield numbers;
    } while (n > 1);
}

function* quickSort(numbers: number[], left: number = 0, right?: number): Iterable<number[]> {
    right = right === undefined ? numbers.length - 1 : right;

    if (left >= right) {
        yield numbers;
        return;
    }

    const partition = (pivot: number, left: number, right: number) => {
        const pivotValue = numbers[pivot];

        let partitionIndex = left;

        for (let i = left; i < right; i++) {
            if (numbers[i] < pivotValue) {
                swap(numbers, i, partitionIndex);
                partitionIndex++;
            }
        }

        swap(numbers, right, partitionIndex);
        return partitionIndex;
    };

    const pivot = right;
    const partitionIndex = partition(pivot, left, right);

    yield numbers;

    yield* quickSort(numbers, left, partitionIndex - 1);
    yield* quickSort(numbers, partitionIndex + 1, right);
}

function swap(numbers: number[], i: number, j: number) {
    const tmp = numbers[i];
    numbers[i] = numbers[j];
    numbers[j] = tmp;
}
