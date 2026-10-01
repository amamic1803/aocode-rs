import init, { AoC } from "../wasm.js";

await init();
let aoc = new AoC();

onmessage = (e) => {
    switch (e.data.workType) {
        case "years":
            postMessage(aoc.years());
            break;
        case "days":
            postMessage(aoc.days(e.data.year));
            break;
        case "solve":
            postMessage(aoc.solve(e.data.year, e.data.day, e.data.part, e.data.input));
            break;
        case "benchmark":
            postMessage(aoc.benchmark(e.data.year, e.data.day, e.data.part, e.data.input));
            break;
        default:
            throw new Error("Unknown work type: " + e.data.workType);
    }
};

postMessage("ready");
