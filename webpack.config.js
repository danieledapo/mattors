const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

const dist = path.resolve(__dirname, "dist");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
    entry: "./index.tsx",
    output: {
        path: dist,
        filename: "bundle.js"
    },

    devtool: "source-map",

    devServer: {
        contentBase: dist,
    },

    plugins: [
        new HtmlWebpackPlugin({
            template: 'index.html'
        }),

        // TODO: uncomment when wasm-pack supports workspaces
        // new WasmPackPlugin({
        //     crateDirectory: path.resolve(__dirname, "geo")
        // }),
    ],

    resolve: {
        extensions: [".ts", ".tsx", ".js", ".scss"]
    },

    module: {
        rules: [

            {
                test: /\.tsx?$/,
                loader: "awesome-typescript-loader"
            },

            {
                enforce: "pre",
                test: /\.js$/,
                loader: "source-map-loader"
            },

            // TODO: in production load scss separately from JS using
            // https://github.com/webpack-contrib/mini-css-extract-plugin
            {
                test: /\.scss$/,
                use: [
                    "style-loader", // creates style nodes from JS strings
                    "css-loader", // translates CSS into CommonJS
                    "postcss-loader", // spectre.css needs autoprefixer
                    "sass-loader" // compiles Sass to CSS, using Node Sass by default
                ]
            }
        ]
    },
};
