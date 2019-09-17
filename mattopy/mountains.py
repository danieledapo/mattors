#!/usr/bin/env python3

import math
import random

import noise


THEMES = {
    "flames": {
        "palette": [
            "rgba(255,   0, 0, 1)",
            "rgba(255,  48, 0, 1)",
            "rgba(255,  71, 0, 1)",
            "rgba(255,  88, 0, 1)",
            "rgba(255, 103, 0, 1)",
            "rgba(255, 117, 0, 1)",
            "rgba(255, 130, 0, 1)",
            "rgba(255, 142, 0, 1)",
            "rgba(255, 154, 0, 1)",
            "rgba(255, 165, 0, 1)",
        ],
        "background": "crimson",
        "connect_bottom": True,
        "padding": 0,
        "stroke": "none",
    },
    "bw": {
        "palette": ["none"],
        "background": "white",
        "connect_bottom": False,
        "padding": 20,
        "stroke": "black",
    },
}


def main():
    width = 1920 / 4
    height = 1080 / 4
    padding = 20
    npoints = 20
    nlines = 20
    seed = random.random()

    theme = THEMES["flames"]
    palette = theme["palette"]
    padding = theme["padding"]

    h = height - padding * 2
    w = width - padding * 2
    bh = h / nlines

    segs = []
    for i in range(nlines):
        y = padding + (i + 0.5) / nlines * h

        def noiseval(x):
            return noise.snoise2(
                x / (npoints - 1),
                i / nlines,
                base=seed,
                octaves=4,
                lacunarity=2,
                persistence=0.9,
            )

        knots = [
            (padding + (x / (npoints - 1) * w), y + bh * 3 * noiseval(x))
            for x in range(npoints)
        ]
        col = palette[math.floor(i / nlines * len(palette))]
        segs.append((knots, col))

    dump_svg(
        "mountains.svg",
        (width, height),
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
