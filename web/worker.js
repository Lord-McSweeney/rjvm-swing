import __wbg_init, { fileLoaded, setPanicHook, onMouseMove } from "./pkg/rjvm_swing.js";

await __wbg_init();

setPanicHook();

let paintQueue = [];

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

self.startPaint = function() {
    paintQueue = [];
}

self.flushPaint = function() {
    self.postMessage({
        "type": "flushPaint",
        "data": paintQueue,
    });
}

self.drawLine = function(x1, y1, x2, y2) {
    paintQueue.push({
        "type": "canvasDrawLine",
        "x1": x1,
        "y1": y1,
        "x2": x2,
        "y2": y2,
    });
}

self.fillRect = function(x, y, width, height) {
    paintQueue.push({
        "type": "canvasFillRect",
        "x": x,
        "y": y,
        "width": width,
        "height": height,
    });
}

self.setColor = function(r, g, b, _) {
    paintQueue.push({
        "type": "canvasSetColor",
        "r": r,
        "g": g,
        "b": b,
    });
}

self.translate = function(x, y, _) {
    paintQueue.push({
        "type": "canvasTranslate",
        "x": x,
        "y": y,
    });
}

self.drawString = function(text, x, y) {
    paintQueue.push({
        "type": "drawString",
        "text": text,
        "x": x,
        "y": y,
    });
}

self.setFont = function(name, size, modifiers) {
    paintQueue.push({
        "type": "setFont",
        "name": name,
        "size": size,
        "modifiers": modifiers,
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

        case "mouseMove":
            onMouseMove(e.data.x, e.data.y);
            break;
    }
});
