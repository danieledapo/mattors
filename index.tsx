import * as React from "react";
import { render } from "react-dom";

import { HashRouter, Link, Route, RouteComponentProps, withRouter } from "react-router-dom";

import "./index.scss";

import { Annulus } from "./sketches/annulus";
import { Astroid } from "./sketches/astroid";
import { Blankets } from "./sketches/blankets";
import { BloodySpiderWeb } from "./sketches/bloody-spider-web";
import { ChristmasSpiralTree } from "./sketches/spiral-christmas-tree";
import { CliffordAttractors } from "./sketches/clifford-attractors";
import { CubicDisarray } from "./sketches/cubic-disarray";
import { Cuts } from "./sketches/cuts";
import { GalaxyMap } from "./sketches/galaxy-map";
import { ISketch } from "./sketches/sketch";
import { Isolines } from "./sketches/isolines";
import { LightInACave } from "./sketches/light-in-a-cave";
import { NeonLines } from "./sketches/neon-lines";
import { Nucleus } from "./sketches/nucleus";
import { Print10 } from "./sketches/print10";
import { RoughBalls } from "./sketches/rough-balls";
import { SpaceFillingCurves } from "./sketches/space-filling-curves";
import { StaticContext } from "react-router";
import { SuperPermutations } from "./sketches/super-permutations";
import { Walls } from "./sketches/walls";
import { Roses } from "./sketches/roses";
import { CairoTiling } from "./sketches/cairo-tiling";
import { PenroseTiling } from "./sketches/penrose-tiling";

export const SKETCHES = [
    new Print10(),
    new CubicDisarray(),
    new BloodySpiderWeb(),
    new Annulus(),
    new Astroid(),
    new NeonLines(),
    new Nucleus(),
    new RoughBalls(),
    new Blankets(),
    new ChristmasSpiralTree(),
    new LightInACave(),
    new Cuts(),
    new Walls(),
    new SuperPermutations(),
    new SpaceFillingCurves(),
    new GalaxyMap(),
    new CliffordAttractors(),
    new Isolines(),
    new Roses(),
    new CairoTiling(),
    new PenroseTiling(),
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
                <h1 className="text-center">
                    <ul className="breadcrumb">
                        <li className="breadcrumb-item">
                            Matto
                        </li>
                    </ul>
                </h1>
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
                <h1 className="text-center">
                    <ul className="breadcrumb">
                        <li className="breadcrumb-item">
                            <Link to="/">Matto</Link>
                        </li>
                        <li className="breadcrumb-item">
                            {this.props.match.params.sketchId}
                        </li>
                    </ul>
                </h1>
                <div className="columns">
                    <div className={"column col-10 col-sm-12 col-mx-auto"}>
                        <div>
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

        return new (p5 as any)((p: p5) => {

            p.setup = () => {
                p.createCanvas(sketch.width, sketch.height);

                sketch.reset(p);
            };

            p.draw = () => {
                p.push();

                sketch.draw(p);
                if (!sketch.loop) {
                    p.noLoop();
                }

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
                if (p.key === " ") {
                    sketch.reset(p);
                    p.draw();
                } else if (p.key === "s") {
                    p.noLoop();
                } else if (p.key === "p" && sketch.loop) {
                    p.loop();
                } else if (p.key === "d") {
                    if (sketch.loop) {
                        p.noLoop();
                    }
                    p.save(`${sketch.name}.png`);
                    if (sketch.loop) {
                        p.loop();
                    }
                } else {
                    return;
                }

                return false;
            };

        }, Sketch.CANVAS_ID);
    }
}

const mountNode = document.getElementById("app");
render((
    <HashRouter>
        <div>
            <Route path="/sketch/:sketchId" component={Sketch} />
            <Route exact path="/" component={withRouter(SketchSelector)} />
        </div>
    </HashRouter>
), mountNode);
