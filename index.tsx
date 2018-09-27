import * as React from "react";
import { render } from "react-dom";

import P5 from "p5";

import { Sketch } from "./sketches/sketch";
import { Print10 } from "./sketches/print10";

export const SKETCHES = [
    new Print10()
];

interface State {
    p5Object: any;
    selectedSketchName: string;
}

export class App extends React.Component<{}, State> {
    private readonly sketchesMap = new Map(
        SKETCHES.map(s => [s.name, s] as [string, Sketch])
    );

    constructor(props: Readonly<{}>) {
        super(props);

        this.changeSketch = this.changeSketch.bind(this);
        this.startSketch = this.startSketch.bind(this);

        this.state = {
            selectedSketchName: SKETCHES[0].name,
            p5Object: undefined,
        };
    }

    public render() {
        const sketches = [];

        for (const sketchName of this.sketchesMap.keys()) {
            sketches.push(
                <option value={sketchName} key={sketchName}>{sketchName}</option>
            )
        }

        const disabled = this.state.p5Object !== undefined;

        return (
            <div>
                <p>Sketches</p>

                <select
                    onChange={this.changeSketch}
                    disabled={disabled}
                    value={this.state.selectedSketchName}
                >
                    ${sketches}
                </select>
                <button onClick={this.startSketch} disabled={disabled}>
                    Start Sketch
                </button>
            </div>
        );
    }

    public changeSketch(evt: React.ChangeEvent<HTMLSelectElement>) {
        this.setState({
            selectedSketchName: evt.target.value
        });
    }

    public startSketch() {
        this.runSketch(this.state.selectedSketchName);
    }

    public runSketch(sketchName: string) {
        const sketch = this.sketchesMap.get(sketchName);
        if (sketch === undefined) {
            throw new Error("da fuck bro?");
        }

        const p5Object = new P5((p: p5) => {

            p.setup = () => {
                p.createCanvas(sketch.width, sketch.height);

                sketch.reset(p);
            };

            p.draw = () => {
                sketch.draw(p);
                p.noLoop();
            };

            p.mouseClicked = () => {
                sketch.reset(p);
                p.draw();
            };

        });

        this.setState({ p5Object });
    }
}

const mountNode = document.getElementById("app");
render(<App />, mountNode);