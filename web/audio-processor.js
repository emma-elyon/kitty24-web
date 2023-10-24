class AudioProcessor extends AudioWorkletProcessor {
	constructor(...args) {
		super(...args)
		this.buffer = new Float32Array(48000)
		this.length = 0
		this.lastSample = 0.0
		this.port.onmessage = ({ data }) => {
			if (data.type == "samples") {
				this.buffer.set(data.value, this.length)
				this.length += data.value.length
			}
		}
	}
	
	process(_inputs, outputs, _parameters) {
		const output = outputs[0]
		const requestedLength = output[0].length
		if (requestedLength <= this.length) {
			output.forEach(channel => {
				channel.set(this.buffer.slice(0, requestedLength))
			})
			this.lastSample = this.buffer[requestedLength - 1]
			this.buffer = this.buffer.copyWithin(0, requestedLength, this.length)
			this.length -= requestedLength
			this.port.postMessage({
				"type": "skew",
				"value": this.length - 2400
			})
		} else {
			output.forEach(channel => {
				for (let i = 0; i < channel.length; ++i) {
					channel[i] = this.lastSample
				}
			})
			this.port.postMessage({
				"type": "underrun",
				"value": this.length - requestedLength
			})
		}
		return true
	}
}

registerProcessor("audio-processor", AudioProcessor)