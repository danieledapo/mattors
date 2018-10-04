import * as React from "react";
import { render } from "react-dom";

import { HashRouter, Route, RouteComponentProps, withRouter } from "react-router-dom";

import "./index.scss";

import { StaticContext } from "react-router";
import { BloodySpiderWeb } from "./sketches/bloody-spider-web";
import { CubicDisarray } from "./sketches/cubic-disarray";
import { Print10 } from "./sketches/print10";
import { ISketch } from "./sketches/sketch";

// p5js is dynamically loaded so reduce bundle size and to better use caching
// since p5js isn't updated too often.
let P5: any;

export const SKETCHES = [
    new Print10(),
    new CubicDisarray(),
    new BloodySpiderWeb(),
];

const sketchesMap = new Map(
    SKETCHES.map((s) => [s.name, s] as [string, ISketch]),
);

interface ISketchSelectorState {
    selectedSketch: string;
}

class SketchSelector extends
    React.Component<
    RouteComponentProps<any, StaticContext, any>,
    ISketchSelectorState
    > {
    constructor(props: Readonly<RouteComponentProps<any, StaticContext, any>>) {
        super(props);

        this.changeSketch = this.changeSketch.bind(this);
        this.startSketch = this.startSketch.bind(this);

        this.state = {
            selectedSketch: SKETCHES[0].name,
        };
    }

    public render() {
        const sketches = [];

        for (const sketchName of sketchesMap.keys()) {
            sketches.push(
                <option value={sketchName} key={sketchName}>{sketchName}</option>,
            );
        }

        return (
            <div className="container">
                <h1 className="text-center">Matto</h1>
                <div className="columns">
                    <div className={"column col-4 col-sm-8 col-mx-auto"}>
                        <p>
                            Matto is a generative art playground built on top of Typescript and Rust.
                            It also uses the p5js library.
                    </p>

                        <div className="input-group">
                            <select
                                className="form-select"
                                onChange={this.changeSketch}
                                value={this.state.selectedSketch}
                            >
                                ${sketches}
                            </select>
                            <button
                                className="btn input-group-btn"
                                onClick={this.startSketch}
                            >
                                Start Sketch
                        </button>
                        </div>
                    </div>
                </div>
            </div>
        );
    }

    private changeSketch(evt: React.ChangeEvent<HTMLSelectElement>) {
        this.setState({
            selectedSketch: evt.target.value,
        });
    }

    private startSketch() {
        this.props.history.push(`/sketch/${this.state.selectedSketch}`);
    }
}

interface ISketchIProps {
    match: {
        params: {
            sketchId: string,
        },
    };
}

class Sketch extends React.Component<ISketchIProps, {}> {
    public static readonly CANVAS_ID = "piece-canvas-container";

    public render() {
        return (
            <div className="container">
                <h1 className="text-center">Matto</h1>
                <div className="columns">
                    <div className={"column col-10 col-sm-12 col-mx-auto"}>
                        <div style={{ overflow: "scroll" }}>
                            <div id={Sketch.CANVAS_ID}></div>
                        </div>
                    </div>
                </div>
            </div>
        );
    }

    public componentDidMount() {
        this.runSketch(this.props.match.params.sketchId);
    }

    private runSketch(sketchName: string) {
        const sketch = sketchesMap.get(sketchName);
        if (sketch === undefined) {
            throw new Error(`da fuck bro? ${sketchName} is not a valid sketch`);
        }

        return new P5((p: p5) => {

            p.setup = () => {
                p.createCanvas(sketch.width, sketch.height);

                sketch.reset(p);
            };

            p.draw = () => {
                p.push();

                sketch.draw(p);
                p.noLoop();

                p.pop();
            };

            p.mouseClicked = () => {
                if (p.mouseButton !== p.LEFT) {
                    return false;
                }

                sketch.reset(p);
                p.draw();

                return false;
            };

            p.keyPressed = () => {
                if (p.key !== " ") {
                    return;
                }

                sketch.reset(p);
                p.draw();

                return false;
            };

        }, Sketch.CANVAS_ID);
    }
}

import(
    /* webpackChunkName: "p5" */
    /* webpackMode: "lazy" */
    "p5",
).then(({ default: pro }) => {
    P5 = pro;

    const mountNode = document.getElementById("app");
    render((
        <HashRouter>
            <div>
                <Route path="/sketch/:sketchId" component={Sketch} />
                <Route exact path="/" component={withRouter(SketchSelector)} />
            </div>
        </HashRouter>
    ), mountNode);
});
