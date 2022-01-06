struct Uniform {
	x: f32;
	y: f32;
	radius: f32;
	color_r: f32;
	color_g: f32;
	color_b: f32;
	anti_alias: u32;
};

[[group(0), binding(0)]]
var<uniform> uniforms: Uniform;

[[group(1), binding(0)]]
var texture_out: texture_storage_2d<rgba8unorm, write>;

fn circle(sample_position: vec2<f32>, radius: f32) -> f32 {
	return length(sample_position) - radius;
}

fn translate(sample_position: vec2<f32>, offset: vec2<f32>) -> vec2<f32> {
	return sample_position - offset;
}

fn signedDistanceToMask(signed_distance: f32) -> f32 {
	let use_anti_alias = uniforms.anti_alias != 0u;
	if (use_anti_alias) {
		return 1.0 - smoothStep(-0.5, 0.5, signed_distance);
	} else {
		return 1.0 - step(0.0, signed_distance);
	}
}

fn renderCircle(color: vec3<f32>, position: vec2<f32>, circle_position: vec2<f32>, radius: f32) -> vec4<f32> {
	var object_position: vec2<f32> = position;
	object_position = translate(object_position, circle_position);
	let signed_distance = circle(object_position, radius);
	let shape_mask = signedDistanceToMask(signed_distance);
	return vec4<f32>(color, shape_mask);
}

[[stage(compute), workgroup_size(16, 16, 1)]]
fn cs_main([[builtin(global_invocation_id)]] global_id : vec3<u32>) {
	let position = vec2<f32>(uniforms.x, uniforms.y);
	let color = vec3<f32>(uniforms.color_r, uniforms.color_g, uniforms.color_b);
	let frag_color: vec4<f32> = renderCircle(color, vec2<f32>(global_id.xy) + vec2<f32>(0.5, 0.5), position, uniforms.radius);
	textureStore(texture_out, vec2<i32>(global_id.xy), frag_color);
}
