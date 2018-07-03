class CanvasWindow {
    constructor(canvas_id, input_handler) {
        this.canvas = document.getElementById(canvas_id);
        this.input_handler = input_handler;

        this.canvas.addEventListener("mousemove", function (event) {
            input_handler.mouse_move(event.offsetX, event.offsetY);
        });
        this.canvas.addEventListener("mousedown", function (event) {
            input_handler.mouse_down(event.button, event.offsetX, event.offsetY);
        });
        this.canvas.addEventListener("mouseup", function (event) {
            input_handler.mouse_up(event.button, event.offsetX, event.offsetY);
        });
        window.addEventListener("keydown", function (event) {
            input_handler.key_down(event.keyCode);
        });
        window.addEventListener("keyup", function (event) {
            input_handler.key_up(event.keyCode);
        });
    }
}

window.create_canvas_window = function (canvas_id, input_handler) {
    return new CanvasWindow(canvas_id, input_handler);
}

window.delete_canvas_window = function (window) {
    window.input_handler.free();
}

window.get_window_context = function (window) {
    return window.canvas.getContext('webgl');
}