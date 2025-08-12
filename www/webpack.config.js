const CopyWebpackPlugin = require("copy-webpack-plugin")
const path = require("path")

module.exports = {
    entry: "./bootstrap.js",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "bootstrap.js",
    },
    mode: "development",
    experiments: {
        asyncWebAssembly: true,
    },
    devServer: {
        allowedHosts: "all",
        host: "0.0.0.0",
        port: 8080,
        client: {
            webSocketURL: "ws://0.0.0.0:80/ws",
        },
    },
    plugins: [
        new CopyWebpackPlugin({
            patterns: ["index.html"],
        }),
    ],
}
