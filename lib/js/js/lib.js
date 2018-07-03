window.set_main_loop = function (cb) {
    function runner() {
        cb.call();
        window.requestAnimationFrame(runner);
    }
    window.requestAnimationFrame(runner);
}

window.rand = function () {
    return Math.random();
}