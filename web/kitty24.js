'use strict';

const WIDTH = 320
const HEIGHT = 180
const FRAME_RATE = 60
const SAMPLE_RATE = 48000
const FRAME_INTERVAL = 1000.0 / FRAME_RATE
const PAGES = 1024 * 1024 * 24 / (64 * 1024)

// Global emulator variables.
const log = document.getElementById('log')
const graph = document.getElementById('graph')
const graphContext = graph.getContext('2d')
let startTime, frameCount = 0

// Global virtual machine variables.
let kitty24

// Initialize global video context as full-screen quad.
const canvas = document.getElementById('video')
const videoContext = canvas.getContext('webgl2')
const vertexSource = `#version 300 es
	in vec2 position;
	out vec2 uv;
	void main() {
		gl_Position = vec4(position.x, position.y, 0.0, 1.0);
		uv = position * 0.5 + 0.5;
	}
`
const fragmentSource = `#version 300 es
	precision highp float;
	uniform sampler2D sampler;
	in vec2 uv;
	out vec4 outColor;
	void main() {
		outColor = texture(sampler, uv);
	}
`
const vertexShader = videoContext.createShader(videoContext.VERTEX_SHADER)
videoContext.shaderSource(vertexShader, vertexSource)
videoContext.compileShader(vertexShader)
const fragmentShader = videoContext.createShader(videoContext.FRAGMENT_SHADER)
videoContext.shaderSource(fragmentShader, fragmentSource)
videoContext.compileShader(fragmentShader)
const program = videoContext.createProgram()
videoContext.attachShader(program, vertexShader)
videoContext.attachShader(program, fragmentShader)
videoContext.linkProgram(program)
videoContext.useProgram(program)
const buffer = videoContext.createBuffer()
const vertices = new Float32Array([-1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0])
videoContext.bindBuffer(videoContext.ARRAY_BUFFER, buffer)
videoContext.bufferData(videoContext.ARRAY_BUFFER, vertices, videoContext.STATIC_DRAW)
const positionAttributeLocation = videoContext.getAttribLocation(program, "position")
const vertexArrayObject = videoContext.createVertexArray()
videoContext.bindVertexArray(vertexArrayObject)
videoContext.vertexAttribPointer(positionAttributeLocation, 2, videoContext.FLOAT, false, 0, 0)
videoContext.enableVertexAttribArray(positionAttributeLocation)
const texture = videoContext.createTexture()
const target = videoContext.TEXTURE_2D
videoContext.bindTexture(target, texture)
videoContext.texImage2D(target, 0, videoContext.RGBA, WIDTH, HEIGHT, 0, videoContext.RGBA, videoContext.UNSIGNED_BYTE, null)
videoContext.texParameteri(target, videoContext.TEXTURE_WRAP_S, videoContext.CLAMP_TO_EDGE)
videoContext.texParameteri(target, videoContext.TEXTURE_WRAP_T, videoContext.CLAMP_TO_EDGE)
videoContext.texParameteri(target, videoContext.TEXTURE_MIN_FILTER, videoContext.NEAREST)
videoContext.texParameteri(target, videoContext.TEXTURE_MAG_FILTER, videoContext.NEAREST)
videoContext.activeTexture(videoContext.TEXTURE0)
videoContext.bindTexture(videoContext.TEXTURE_2D, texture)
const samplerUniformLocation = videoContext.getUniformLocation(program, "sampler")
videoContext.uniform1i(samplerUniformLocation, 0)

let videoBuffer

// Initialize global audio context.
let audioContext = new AudioContext({
	latencyHint: "interactive",
	sampleRate: SAMPLE_RATE,
})
document.addEventListener("click", () => audioContext.resume())
let audioBuffer = new Float32Array(SAMPLE_RATE / FRAME_RATE)
let skew = 0
let node

const onmessage = ({ data: { type, value }}) => {
	switch (type) {
		case "skew":
			skew = value
			break
		case "underrun":
			console.log("UNDERRUN", value)
			break
	}
}

// Initialize Kitty24 emulator.
(async () => {
	// Initialize audio.
	await audioContext.audioWorklet.addModule('audio-processor.js')
	node = new AudioWorkletNode(audioContext, 'audio-processor')
	node.connect(audioContext.destination)
	node.port.onmessage = onmessage

	// Fetch and instantiate module.
	const response = fetch('kitty24.wasm')
	const mem = new WebAssembly.Memory({ initial: PAGES })
	const imports = { js: { mem }}
	const { instance: { exports }} =
		await WebAssembly.instantiateStreaming(response, imports)
	kitty24 = exports

	// Assign global variables for interacting with the virtual machine.
	const virtualMachine = kitty24.virtual_machine()
	const audioOffset = kitty24.audio(virtualMachine)
	const videoOffset = kitty24.video(virtualMachine)

	const errorOffset = kitty24.error(virtualMachine)
	const errorLength = kitty24.error_message(virtualMachine)
	if (errorLength == 0) {
		// Initialize global audio and video memory buffer views once.
		audioBuffer = new Float32Array(kitty24.memory.buffer, audioOffset, SAMPLE_RATE / FRAME_RATE)
		videoBuffer = new Uint8ClampedArray(kitty24.memory.buffer, videoOffset, WIDTH * HEIGHT * 4)

		// Kick off the update loop.
		const then = performance.now()
		startTime = then
		requestAnimationFrame(update(virtualMachine)(then))
	} else {
		// Log error if it is non-zero.
		const bytes = new Uint8Array(kitty24.memory.buffer, errorOffset, errorLength);
		const string = new TextDecoder().decode(bytes)
		console.error("ERROR", string)
	}

})()

// Update every hardware frame (only step and render every FRAME_INTERVAL).
const update = vm => then => now => {
	const elapsed = now - then

	if (elapsed > FRAME_INTERVAL) {
		// New timestamp, but mind overrun to catch up later.
		then = now - (elapsed % FRAME_INTERVAL)

		log.innerHTML = `${Math.round(100.0 * 1000.0 / (now - startTime) * ++frameCount) / 100.0} frames per second<br>${Math.round(1000.0 * skew / SAMPLE_RATE)} milliseconds buffered`

		// Run virtual machine for one frame.
		kitty24.run(vm)

		// Run for more frames if we're behind on audio.
		while (skew < 0 && audioContext.state == "running") {
			node.port.postMessage({
				type: "samples",
				value: audioBuffer.slice(),
			})
			kitty24.run(vm)
			skew += audioBuffer.length
			++frameCount
		}

		// Update video context.
		// Supports High DPI/user zoom.
		// TODO: Avoid doing this outside of zoom- or resize events.
		const dpr = window.devicePixelRatio
		const width = window.innerWidth * dpr
		const height = window.innerHeight * dpr
		const [intWidth, intHeight] = [Math.floor(width / 320.0), Math.floor(height / 180.0)]
		const scale = Math.min(intWidth, intHeight)
		canvas.style.scale = scale / dpr
		const left = width / 2.0 / dpr - canvas.width / 2.0
		const top = height / 2.0 / dpr - canvas.height / 2.0
		canvas.style.left = `${Math.floor(left)}px`
		canvas.style.top = `${Math.floor(top)}px`

		// Update texture and render quad.
		videoContext.texSubImage2D(
			target,
			0, // level
			0, // xoffset
			0, // yoffset
			WIDTH,
			HEIGHT,
			videoContext.RGBA,
			videoContext.UNSIGNED_BYTE,
			videoBuffer
		)
		videoContext.drawArrays(videoContext.TRIANGLE_STRIP, 0, 4)

		// Update audio context with samples if we're not too far ahead.
		if (skew < 2400 && audioContext.state == "running") {
			const value = audioBuffer.slice()
			graph.width = value.length
			graph.height = 256
			graphContext.strokeStyle = "red"
			graphContext.beginPath()
			value.forEach((value, index) => {
				if (index === 0) {
					graphContext.moveTo(index, (value * 0.5 + 0.5) * 255)
				} else {
					graphContext.lineTo(index, (value * 0.5 + 0.5) * 255)
				}
			})
			graphContext.stroke()
			node.port.postMessage({
				type: "samples",
				value,
			})
		}
	}

	requestAnimationFrame(update(vm)(then))
}