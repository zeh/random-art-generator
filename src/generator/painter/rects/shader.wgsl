struct Uniform {
	x: f32;
	y: f32;
	width: f32;
	height: f32;
	rotation: f32;
	color_r: f32;
	color_g: f32;
	color_b: f32;
	anti_alias: u32;
};

let PI: f32 = 3.14159265358979323846264338327950288;

[[group(0), binding(0)]]
var<uniform> uniforms: Uniform;

[[group(1), binding(0)]]
var texture_out: texture_storage_2d<rgba8unorm, write>;

fn rectangle(sample_position: vec2<f32>, half_size: vec2<f32>) -> f32 {
	let component_wise_edge_distance = vec2<f32>(abs(sample_position.x), abs(sample_position.y)) - half_size;
	let outside_distance = length(max(component_wise_edge_distance, vec2<f32>(0.0)));
	let inside_distance = min(max(component_wise_edge_distance.x, component_wise_edge_distance.y), 0.0);
	return outside_distance + inside_distance;
}

fn translate(sample_position: vec2<f32>, offset: vec2<f32>) -> vec2<f32> {
	return sample_position - offset;
}

fn rotate(sample_position: vec2<f32>, rotation: f32) -> vec2<f32> {
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

fn renderRectangle(color: vec3<f32>, position: vec2<f32>, rect_position: vec2<f32>, rect_size: vec2<f32>, rect_rotation: f32) -> vec4<f32> {
	var object_position: vec2<f32> = position;
	object_position = translate(object_position, rect_position);
	object_position = rotate(object_position, rect_rotation);
	let signed_distance = rectangle(object_position, rect_size * 0.5);
	let shape_mask = signedDistanceToMask(signed_distance);
	return vec4<f32>(color, shape_mask);
}

[[stage(compute), workgroup_size(16, 16, 1)]]
fn cs_main([[builtin(global_invocation_id)]] global_id : vec3<u32>) {
	let position = vec2<f32>(uniforms.x, uniforms.y);
	let size = vec2<f32>(uniforms.width, uniforms.height);
	let color = vec3<f32>(uniforms.color_r, uniforms.color_g, uniforms.color_b);
	let frag_color: vec4<f32> = renderRectangle(color, vec2<f32>(global_id.xy) + vec2<f32>(0.5, 0.5), position, size, uniforms.rotation);
	textureStore(texture_out, vec2<i32>(global_id.xy), frag_color);
}
