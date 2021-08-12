// eslint-disable-next-line @typescript-eslint/no-var-requires
const HtmlWebpackPlugin = require("html-webpack-plugin");
// eslint-disable-next-line @typescript-eslint/no-var-requires
const path = require("path");

module.exports = {
    devServer: {
        contentBase: path.join(__dirname, "dist"),
        compress: true,
        port: 9000,
    },
    entry: "./index.ts",
    experiments: { topLevelAwait: true },
    module: {
        rules: [{ test: /\.ts?$/, use: "ts-loader", exclude: /node_modules/ }],
    },
    output: { filename: "bundle.js", path: path.resolve(__dirname, "dist") },
    plugins: [new HtmlWebpackPlugin()],
    resolve: {
        alias: { "@lazyjson": path.resolve(__dirname, "../pkg/lazyjson") },
        extensions: [".tsx", ".ts", ".js"],
    },
};
