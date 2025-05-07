import __wbg_init, { fileLoaded, setPanicHook } from "./pkg/rjvm_swing.js";

await __wbg_init();

setPanicHook();

self.appendText = function(text) {
    self.postMessage({
        "type": "textOutput",
        "data": text,
    });
}

self.setFrameName = function(name) {
    self.postMessage({
        "type": "setFrameName",
        "name": name,
    });
}

self.drawLine = function(x1, y1, x2, y2) {
    self.postMessage({
        "type": "canvasDrawLine",
        "x1": x1,
        "y1": y1,
        "x2": x2,
        "y2": y2,
    });
}

self.fillRect = function(x, y, width, height) {
    self.postMessage({
        "type": "canvasFillRect",
        "x": x,
        "y": y,
        "width": width,
        "height": height,
    });
}

self.setColor = function(r, g, b, _) {
    self.postMessage({
        "type": "canvasSetColor",
        "r": r,
        "g": g,
        "b": b,
    });
}

self.translate = function(x, y, _) {
    self.postMessage({
        "type": "canvasTranslate",
        "x": x,
        "y": y,
    });
}

let currentFileName = null;
let currentFileData = null;

self.addEventListener("message", function(e) {
    switch (e.data.type) {
        case "fileUpload":
            currentFileName = e.data.fileName;
            currentFileData = e.data.fileData;

            break;

        case "runFile":
            let args = e.data.args;
            if (currentFileName == null || currentFileData == null) {
                throw new Error("Called \"runFile\" with null name or data");
            }

            fileLoaded(currentFileName, currentFileData, args);
            break;
    }
});
