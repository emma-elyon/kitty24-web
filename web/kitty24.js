'use strict';

const WIDTH = 320
const HEIGHT = 180
const FRAME_RATE = 60
const SAMPLE_RATE = 48000
const FRAME_INTERVAL = 1000.0 / FRAME_RATE
const PAGES = 1024 * 1024 * 24 / (64 * 1024)

// Global emulator variables.
let log = document.getElementById('log')
let startTime, frameCount = 0

// Global virtual machine variables.
let kitty24

// Initialize global video context as full-screen quad.
const canvas = document.getElementById('video')
const videoContext = canvas.getContext("webgl2")
const vertexSource = `#version 300 es
	in vec2 position;
	uniform vec2 resolution;
	out vec2 uv;
	void main() {
		vec2 scaled_position = vec2(
			position.x * 320.0 / resolution.x,
			position.y * 180.0 / resolution.y
		);
		float scale = min(resolution.x / 320.0, resolution.y / 180.0);
		scale = floor(scale) == 0.0 ? scale : floor(scale);
		gl_Position = vec4(scaled_position.x * scale, -scaled_position.y * scale, 0, 1);
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
const vertices = new Float32Array([
	-1.0, -1.0,
	-1.0,  1.0,
	 1.0, -1.0,
	 1.0,  1.0,
])
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
const resolutionUniformLocation = videoContext.getUniformLocation(program, "resolution")
videoContext.uniform2f(resolutionUniformLocation, 320, 180)

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
	let virtualMachine = kitty24.virtual_machine()
	let audioOffset = kitty24.audio(virtualMachine)
	let videoOffset = kitty24.video(virtualMachine)

	// Initialize global audio and video memory buffer views.
	audioBuffer = new Float32Array(kitty24.memory.buffer, audioOffset, SAMPLE_RATE / FRAME_RATE)
	videoBuffer = new Uint8ClampedArray(kitty24.memory.buffer, videoOffset, WIDTH * HEIGHT * 4)

	// Kick off the update loop.
	let then = performance.now()
	startTime = then
	requestAnimationFrame(update(virtualMachine)(then))
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
		// TODO: Support High DPI/user zoom:
		// https://webgl2fundamentals.org/webgl/lessons/webgl-resizing-the-canvas.html
		if (canvas.width != window.innerWidth || canvas.height != window.innerHeight) {
			canvas.width = window.innerWidth
			canvas.height = window.innerHeight
			videoContext.viewport(0, 0, canvas.width, canvas.height)
		}
		videoContext.uniform2f(resolutionUniformLocation, canvas.width, canvas.height)
		videoContext.texSubImage2D(target, 0, 0, 0, WIDTH, HEIGHT, videoContext.RGBA, videoContext.UNSIGNED_BYTE, videoBuffer)
		videoContext.drawArrays(videoContext.TRIANGLE_STRIP, 0, 4)

		// Update audio context with samples if we're not too far ahead.
		if (skew < 2400 && audioContext.state == "running") {
			node.port.postMessage({
				type: "samples",
				value: audioBuffer.slice(),
			})
		}
	}

	requestAnimationFrame(update(vm)(then))
}