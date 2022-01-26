// TODO: move these to uniforms at some point
// They're fractions of the "smear size", which is 20% of min(image_width, image_height)
let smear_edge_distance_start: f32 = 0.5;
let smear_edge_distance_end: f32 = 0.5;

struct Uniform {
	x: f32;
	y: f32;
	width: f32;
	length: f32;
	rotation: f32;
	corner_radius: f32;
	wave_height: f32;
	wave_length: f32;
	smear_strength: f32;
	smear_size: f32;
	rng_seed: u32;
	color_r: f32;
	color_g: f32;
	color_b: f32;
	anti_alias: u32;
};

let PI: f32 = 3.14159265358979323846264338327950288;
let NOISE_MAP_SIZE: u32 = 256u;

[[group(0), binding(0)]]
var<uniform> uniforms: Uniform;

[[group(1), binding(0)]]
var texture_out: texture_storage_2d<rgba8unorm, write>;

fn rngU32ToF32(rng: u32) -> f32 {
	return f32(rng) / f32(!0u);
}

fn rngNext(rng_seed: u32) -> u32 {
	var value = rng_seed;
	value = value ^ (value << 13u);
	value = value ^ (value >> 17u);
	value = value ^ (value << 5u);
	return value;
}

fn rngNextN(rng_seed: u32, n: u32) -> u32 {
	var value = rng_seed;
	var r = n;
	loop {
		if (r > 0u) {
			r = r - 1u;
			value = rngNext(value);
		} else {
			break;
		}
	}
	return value;
}

// Find the positions in the noise map for the start and end segments
// where this position (0..1) sits in
fn getNoiseValue(rng_seed: u32, position: f32) -> f32 {
	let pp1 = select(position, position + 1.0, position < 0.0);
	let pp2 = fract(pp1) * f32(NOISE_MAP_SIZE);
	let p1 = u32(pp2) % NOISE_MAP_SIZE;
	let p2 = (p1 + 1u) % NOISE_MAP_SIZE;

	// Finds the noise value at the first and second index
	var v1 = 0.0;
	var v2 = 0.0;

	// Faster paths to avoid re-calculating the list twice
	if (p1 < p2) {
		let r1 = rngNextN(rng_seed, p1);
		v1 = rngU32ToF32(r1);
		v2 = rngU32ToF32(rngNextN(r1, p2 - p1));
	} else {
		let r2 = rngNextN(rng_seed, p2);
		v2 = rngU32ToF32(r2);
		v1 = rngU32ToF32(rngNextN(r2, p1 - p2));
	}

	// Phase from v1 to v2
	let f = fract(pp2);

	// Remap phase for smoothstep
	let rf = (1.0 - cos(f * PI)) * 0.5;

	// Finally, interpolate between the two
	return v1 + (v2 - v1) * rf;
}

// Similar to getNoiseValue(), but allows a range to be passed.
// This is so we can apply anti-aliasing and reduce artifacts when
// calculating the smear texture.
// We use a separate function so we can do a smarter calculation without too many
// rngNextN() calls
fn getNoiseValueSmooth(rng_seed: u32, position: f32, range: f32) -> f32 {
	// Decide on samples based on smear size, so we don't waste too
	// much power when not needed
	let noise_segment_size = uniforms.smear_size / f32(NOISE_MAP_SIZE);

	let min_samples = 2.0;
	let max_samples = 16.0;
	let samples_wanted = round(clamp(1.0 / noise_segment_size * 4.0, min_samples, max_samples));

	var samples = 0u;
	var accum = 0.0;

	// Recalculate the range to avoid overcounting boundaries
	let range_segment = range / samples_wanted;
	let real_range = range - range_segment;

	// Will cache some values between iterations to make it faster
	var last_rng_pos = !0u;
	var last_rng_value = 0u;

	loop {
		let pp0 = position - range * 0.5 + f32(samples) * (range / (samples_wanted - 1.0));
		let pp1 = select(pp0, pp0 + 1.0, pp0 < 0.0);
		let pp2 = fract(pp1) * f32(NOISE_MAP_SIZE);

		let p1 = u32(pp2) % NOISE_MAP_SIZE;
		let p2 = (p1 + 1u) % NOISE_MAP_SIZE;

		var v1 = 0.0;
		var v2 = 0.0;

		// Faster paths to avoid re-calculating the list twice
		if (p1 < p2) {
			var r1 = last_rng_value;
			if (p1 != last_rng_pos) {
				if (p1 > last_rng_pos) {
					r1 = rngNextN(last_rng_value, p1 - last_rng_pos);
				} else {
					r1 = rngNextN(rng_seed, p1);
				}
			}
			v1 = rngU32ToF32(r1);
			v2 = rngU32ToF32(rngNextN(r1, p2 - p1));

			if (p1 < last_rng_pos) {
				last_rng_pos = p1;
				last_rng_value = r1;
			}
		} else {
			var r2 = last_rng_value;
			if (p2 != last_rng_pos) {
				if (p2 > last_rng_pos) {
					r2 = rngNextN(last_rng_value, p2 - last_rng_pos);
				} else {
					r2 = rngNextN(rng_seed, p2);
				}
			}
			v2 = rngU32ToF32(r2);
			v1 = rngU32ToF32(rngNextN(r2, p1 - p2));

			if (p2 < last_rng_pos) {
				last_rng_pos = p2;
				last_rng_value = r2;
			}
		}

		let f = fract(pp2);
		let rf = (1.0 - cos(f * PI)) * 0.5;
		accum = accum + (v1 + (v2 - v1) * rf);

		samples = samples + 1u;

		if (samples >= u32(samples_wanted)) {
			break;
		}
	}

	return accum / f32(samples_wanted);
}

fn applySmear(sample_position: vec2<f32>, half_size: vec2<f32>, corner_radius: f32) -> f32 {
	if (uniforms.smear_strength == 0.0) {
		return 0.0;
	}

	let rng_seed = uniforms.rng_seed;
	let smear_texture = getNoiseValueSmooth(rng_seed, sample_position.y / uniforms.smear_size, 1.0 / uniforms.smear_size);

	var smear_edge_distance =
		select(smear_edge_distance_end, smear_edge_distance_start, sample_position.x < 0.0)
		* smear_texture
		* uniforms.smear_size
		* uniforms.smear_strength;

	if (smear_edge_distance > 0.0) {
		var edge_distance = half_size.x - abs(sample_position.x);
		let top_distance = half_size.y - abs(sample_position.y);

		if (corner_radius > 0.0 && top_distance < corner_radius && edge_distance < corner_radius + smear_edge_distance) {
			// Take rounded corner into account if necessary
			let corner_y = corner_radius - top_distance;
			let corner_x = sqrt(corner_radius * corner_radius - corner_y * corner_y);
			let horizontal_distance_for_curve = corner_radius - corner_x;
			edge_distance = edge_distance - horizontal_distance_for_curve;
		}

		let smear_edges = 1.0 - smoothStep(0.0, smear_edge_distance, edge_distance);
		return clamp(0.0, 1.0, smear_texture * uniforms.smear_strength + smear_edges);
	}

	return clamp(0.0, 1.0, smear_texture * uniforms.smear_strength);
}

fn applyWave(sample_position: vec2<f32>, wave_height: f32, wave_length: f32) -> vec2<f32> {
	let rng_seed = uniforms.rng_seed + 1u;
	let noise_x = (getNoiseValue(rng_seed, sample_position.y / wave_length) - 0.5) * wave_height;
	let noise_y = (getNoiseValue(rng_seed, sample_position.x / wave_length) - 0.5) * wave_height;

	return vec2<f32>(sample_position.x + noise_x, sample_position.y + noise_y);
}

fn stroke(sample_position: vec2<f32>, half_size: vec2<f32>, corner_radius: f32) -> f32 {
	let component_wise_edge_distance = vec2<f32>(abs(sample_position.x), abs(sample_position.y)) - half_size + vec2<f32>(corner_radius);
	let outside_distance = length(max(component_wise_edge_distance, vec2<f32>(0.0)));
	let inside_distance = min(max(component_wise_edge_distance.x, component_wise_edge_distance.y), 0.0);
	return outside_distance + inside_distance - corner_radius;
}

fn translate(sample_position: vec2<f32>, offset: vec2<f32>) -> vec2<f32> {
	return sample_position - offset;
}

fn rotate(sample_position: vec2<f32>, rotation: f32) -> vec2<f32> {
	let PI = 3.141592653;
	let angle = rotation / 180.0 * PI;
	let sine = sin(angle);
	let cosine = cos(angle);
	return vec2<f32>(
		cosine * sample_position.x + sine * sample_position.y,
		cosine * sample_position.y - sine * sample_position.x
	);
}

fn signedDistanceToMask(signed_distance: f32) -> f32 {
	let use_anti_alias = uniforms.anti_alias != 0u;
	if (use_anti_alias) {
		return 1.0 - smoothStep(-0.5, 0.5, signed_distance);
	} else {
		return 1.0 - step(0.0, signed_distance);
	}
}

fn renderStroke(color: vec3<f32>, position: vec2<f32>, stroke_position: vec2<f32>, stroke_size: vec2<f32>, stroke_rotation: f32, stroke_corner_radius: f32) -> vec4<f32> {
	var object_position: vec2<f32> = position;

	// Position
	object_position = translate(object_position, stroke_position);
	object_position = rotate(object_position, stroke_rotation);

	// Wave displacement-like distortion to the shape
	object_position = applyWave(object_position, uniforms.wave_height, uniforms.wave_length);

	// Paint
	let signed_distance = stroke(object_position, stroke_size * 0.5, stroke_corner_radius);
	let shape_mask = signedDistanceToMask(signed_distance);

	// If we know it's transparent, return early
	if (shape_mask == 0.0) {
		return vec4<f32>(color, 0.0);
	}

	// Surface details
	let smear = applySmear(object_position, stroke_size * 0.5, stroke_corner_radius);

	return vec4<f32>(color, shape_mask * (1.0 - smear));
}

[[stage(compute), workgroup_size(16, 16, 1)]]
fn cs_main([[builtin(global_invocation_id)]] global_id : vec3<u32>) {
	let position = vec2<f32>(uniforms.x, uniforms.y);
	let size = vec2<f32>(uniforms.length, uniforms.width);
	let color = vec3<f32>(uniforms.color_r, uniforms.color_g, uniforms.color_b);
	let frag_color: vec4<f32> = renderStroke(color, vec2<f32>(global_id.xy) + vec2<f32>(0.5, 0.5), position, size, uniforms.rotation, uniforms.corner_radius);
	textureStore(texture_out, vec2<i32>(global_id.xy), frag_color);
}
