#!/usr/bin/env python3

import argparse
import math
import random

import noise


THEMES = {
    "flame": {
        "palette": [
            "rgb(255,  48, 0)",
            "rgb(255,  71, 0)",
            "rgb(255,  88, 0)",
            "rgb(255, 103, 0)",
            "rgb(255, 117, 0)",
            "rgb(255, 130, 0)",
            "rgb(255, 142, 0)",
            "rgb(255, 154, 0)",
            "rgb(255, 165, 0)",
        ],
        "background": "rgb(255, 0, 0)",
        "connect_bottom": True,
        "padding": 0,
        "stroke": "none",
    },
    "ocean": {
        "palette": [
            "rgb(68, 44, 253)",
            "rgb(94, 70, 251)",
            "rgb(113, 92, 249)",
            "rgb(129, 113, 246)",
            "rgb(141, 134, 243)",
            "rgb(151, 154, 241)",
            "rgb(160, 175, 237)",
            "rgb(167, 195, 234)",
            "rgb(173, 216, 230)",
        ],
        "background": "rgb(0, 0, 255)",
        "connect_bottom": True,
        "padding": 0,
        "stroke": "none",
    },
    "bw": {
        "palette": ["none"],
        "background": "white",
        "connect_bottom": False,
        "padding": 0.08,
        "stroke": "black",
    },
}


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("--width", default=1920, type=int)
    parser.add_argument("--height", default=1080, type=int)
    parser.add_argument("--points", default=20, type=int)
    parser.add_argument("--lines", default=20, type=int)
    parser.add_argument("-t", "--theme", choices=list(THEMES.keys()), default="flame")

    return parser.parse_args()


def main():
    args = parse_args()
    seed = random.random()

    theme = THEMES[args.theme]
    palette = theme["palette"]
    padding = min(args.width, args.height) * theme["padding"]

    h = args.height - padding * 2
    w = args.width - padding * 2
    bh = h / args.lines

    segs = []
    for i in range(args.lines):
        y = padding + (i + 0.5) / args.lines * h

        def noiseval(x):
            return noise.snoise2(
                x / (args.points - 1),
                i / args.lines,
                base=seed,
                octaves=4,
                lacunarity=2,
                persistence=0.9,
            )

        knots = [
            (padding + (x / (args.points - 1) * w), y + bh * 3 * noiseval(x))
            for x in range(args.points)
        ]
        col = palette[math.floor(i / args.lines * len(palette))]
        segs.append((knots, col))

    dump_svg(
        "mountains.svg",
        (args.width, args.height),
        segs,
        background=theme["background"],
        stroke=theme["stroke"],
        padding=padding,
        connect_bottom=theme["connect_bottom"],
    )


def dump_svg(
    filename,
    dimensions,
    lines,
    background,
    stroke="none",
    padding=20,
    connect_bottom=True,
):
    width, height = dimensions

    def bezier(i, points):
        def ctrl_pt(cur, prv, nex, reverse=False, smoothing=0.2):
            cur = points[cur]
            prv = cur if prv < 0 else points[prv]
            nex = cur if nex >= len(points) else points[nex]

            length, angle = line_props(prv, nex)
            length *= smoothing
            if reverse:
                angle += math.pi

            return (
                cur[0] + math.cos(angle) * length,
                cur[1] + math.sin(angle) * length,
            )

        cpsx, cpsy = ctrl_pt(i - 1, i - 2, i)
        cpex, cpey = ctrl_pt(i, i - 1, i + 1, reverse=True)
        return (cpsx, cpsy, cpex, cpey, *points[i])

    with open(filename, "wt") as fp:
        print(
            """<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg xmlns="http://www.w3.org/2000/svg" version="1.1" viewBox="0 0 {width} {height}">
<rect width="{width}" height="{height}" stroke="none" fill="{background}" />
{lines}
</svg>""".format(
                width=width,
                height=height,
                background=background,
                lines="\n".join(
                    '<path d="{points}" stroke="{stroke}" fill="{fill}" />'.format(
                        points=" ".join(
                            "M {} {}".format(x, y)
                            if i == 0
                            else "C {},{} {},{} {},{}".format(*bezier(i, pts))
                            for i, (x, y) in enumerate(pts)
                        )
                        + (
                            "L {} {} L {} {} ".format(
                                width - padding,
                                height - padding,
                                padding,
                                height - padding,
                            )
                            if connect_bottom
                            else ""
                        ),
                        fill=fill,
                        stroke=stroke,
                    )
                    for (pts, fill) in lines
                ),
            ),
            file=fp,
        )


def line_props(prv, nex):
    lenx = nex[0] - prv[0]
    leny = nex[1] - prv[1]

    return (math.sqrt(lenx ** 2 + leny ** 2), math.atan2(leny, lenx))


if __name__ == "__main__":
    main()
