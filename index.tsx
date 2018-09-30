import * as React from "react";
import { render } from "react-dom";

import "./index.scss";

// TODO: lazy load this
import P5 from "p5";

import { CubicDisarray } from "./sketches/cubic-disarray";
import { Print10 } from "./sketches/print10";
import { ISketch } from "./sketches/sketch";

export const SKETCHES = [
    new Print10(),
    new CubicDisarray(),
];

const CANVAS_ID = "piece-canvas-container";

interface IState {
    p5Object: any;
    selectedSketchName: string;
}

export class App extends React.Component<{}, IState> {
    private readonly sketchesMap = new Map(
        SKETCHES.map((s) => [s.name, s] as [string, ISketch]),
    );

    constructor(props: Readonly<{}>) {
        super(props);

        this.changeSketch = this.changeSketch.bind(this);
        this.startSketch = this.startSketch.bind(this);

        this.state = {
            p5Object: undefined,
            selectedSketchName: SKETCHES[0].name,
        };
    }

    public render() {
        let intro = <div></div>;
        let centeringClasses: string;

        if (this.state.p5Object === undefined) {
            const sketches = [];

            for (const sketchName of this.sketchesMap.keys()) {
                sketches.push(
                    <option value={sketchName} key={sketchName}>{sketchName}</option>,
                );
            }

            intro = (
                <div>
                    <p>
                        Matto is a generative art playground built on top of Typescript and Rust.
                        It also uses the p5js library.
                    </p>

                    <div className="input-group">
                        <select
                            className="form-select"
                            onChange={this.changeSketch}
                            value={this.state.selectedSketchName}
                        >
                            ${sketches}
                        </select>
                        <button className="btn input-group-btn" onClick={this.startSketch}>
                            Start Sketch
                        </button>
                    </div>

                </div>
            );

            centeringClasses = "col-4 col-sm-8 col-mx-auto";
        } else {
            centeringClasses = "col-10 col-sm-12 col-mx-auto";
        }

        return (
            <div className="container">
                <h1 className="text-center">Matto</h1>
                <div className="columns">
                    <div className={`column ${centeringClasses}`}>
                        {intro}
                        <div style={{ overflow: "scroll" }}>
                            <div id={CANVAS_ID}></div>
                        </div>
                    </div>
                </div>
            </div>
        );
    }

    public changeSketch(evt: React.ChangeEvent<HTMLSelectElement>) {
        this.setState({
            selectedSketchName: evt.target.value,
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
                if (p.mouseButton !== p.LEFT) {
                    return;
                }

                sketch.reset(p);
                p.draw();
            };

        }, CANVAS_ID);

        this.setState({ p5Object });
    }
}

const mountNode = document.getElementById("app");
render(<App />, mountNode);
