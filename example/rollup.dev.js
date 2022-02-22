import rust from "@wasm-tool/rollup-plugin-rust";
import serve from "rollup-plugin-serve";
import livereload from "rollup-plugin-livereload";

export default {
    input: {
        index: "./Cargo.toml",
    },
    output: {
        dir: "public/wasm",
        format: "iife",
        sourcemap: true,
    },
    plugins: [
        rust({
            serverPath: "wasm/",
            debug: false,
            watchPatterns: ["src/**", "../crate", "../../shipyard-hierarchy/src/**"],
            cargoArgs: ["--features", "dev"],
            watch: true,
        }),

        serve({
            contentBase: "public",
            open: true,
            historyApiFallback: true,
        }),

        livereload("public"),
    ],
};
