const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

const dist = path.resolve(__dirname, "dist");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

module.exports = {
    entry: "./index.tsx",
    output: {
        path: dist,
        filename: "bundle.js",
        chunkFilename: '[name].bundle.js',
    },

    devtool: "source-map",

    devServer: {
        contentBase: dist,
    },

    plugins: [
        new HtmlWebpackPlugin({
            template: 'index.html'
        }),

        new MiniCssExtractPlugin({
            filename: "[name].css",
            chunkFilename: "[id].css"
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

            {
                test: /\.scss$/,
                use: [
                    process.env.NODE_ENV !== 'production' ? 'style-loader' : MiniCssExtractPlugin.loader,
                    "css-loader",
                    "postcss-loader", // spectre.css needs autoprefixer
                    "sass-loader"
                ]
            }
        ]
    },
};
