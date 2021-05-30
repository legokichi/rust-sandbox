const path = require('path');
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const pkg = require("./package.json");
module.exports = {
    entry: './src/index.ts',
    target: 'node',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js',
    },
    plugins: [
        new WasmPackPlugin({
            // crateDirectory: path.resolve(__dirname, pkg.dependencies["wasm-rust-crate"].split("file:")[1])
            crateDirectory: path.resolve(__dirname, "../../crates/wasm-rust-crate/pkg")
        }),
    ],
    module: {
        rules: [
            {
                test: /\.ts$/,
                use: 'ts-loader',
            }
        ],
    },
    // mode: 'development',
    mode: "production",
    experiments: {
        syncWebAssembly: true
    },
    optimization: {
        minimize: false
    },
};