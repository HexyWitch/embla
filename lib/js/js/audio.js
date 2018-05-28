window.create_audio_output = function (channels, callback) {
    var audioCtx = new AudioContext();
    var scriptNode = audioCtx.createScriptProcessor(4096, 0, channels);

    scriptNode.connect(audioCtx.destination);
    scriptNode.onaudioprocess = function (e) {
        for (var channel = 0; channel < e.outputBuffer.numberOfChannels; channel++) {
            callback.call(channel, audioCtx.sampleRate, e.outputBuffer.getChannelData(channel));
        }
    }

    return {
        callback: callback
    }
}

window.destroy_audio_output = function (device) {
    device.callback.free();
}