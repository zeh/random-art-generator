struct Uniform {
	blend_type: u32,
	opacity: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniform;

@group(1) @binding(0)
var sampler_in: sampler;

@group(1) @binding(1)
var texture_bottom_in: texture_2d<f32>;

@group(1) @binding(2)
var texture_top_in: texture_2d<f32>;

@group(1) @binding(3)
var texture_out: texture_storage_2d<rgba8unorm, write>;

fn blend_normal(bottom: f32, top: f32) -> f32 {
	return top;
}

fn blend_multiply(bottom: f32, top: f32) -> f32 {
	return bottom * top;
}

fn blend_screen(bottom: f32, top: f32) -> f32 {
	return 1.0 - (1.0 - bottom) * (1.0 - top);
}

fn blend_overlay(bottom: f32, top: f32) -> f32 {
	if (bottom < 0.5) {
		return 2.0 * bottom * top;
	} else {
		return 1.0 - 2.0 * (1.0 - bottom) * (1.0 - top);
	}
}

fn blend_darken(bottom: f32, top: f32) -> f32 {
	return min(bottom, top);
}

fn blend_lighten(bottom: f32, top: f32) -> f32 {
	return max(bottom, top);
}

fn blend_color_dodge(bottom: f32, top: f32) -> f32 {
	if (bottom == 0.0) {
		return 0.0;
	} else if (top == 1.0) {
		return 1.0;
	} else {
		return min((bottom / (1.0 - top)), 1.0);
	}
}

fn blend_color_burn(bottom: f32, top: f32) -> f32 {
	if (bottom == 1.0) {
		return 1.0;
	} else if (top == 0.0) {
		return 0.0;
	} else {
		return 1.0 - min(((1.0 - bottom) / top), 1.0);
	}
}

fn blend_hard_light(bottom: f32, top: f32) -> f32 {
	if (top <= 0.5) {
		return 2.0 * bottom * top;
	} else {
		return 1.0 - (1.0 - bottom) * (1.0 - (2.0 * top - 1.0));
	}
}

fn blend_soft_light(bottom: f32, top: f32) -> f32 {
	if (top <= 0.5) {
		return bottom - (1.0 - 2.0 * top) * bottom * (1.0 - bottom);
	} else {
		var d = 0.0;
		if (bottom <= 0.25) {
			d = ((16.0 * bottom - 12.0) * bottom + 4.0) * bottom;
		} else {
			d = sqrt(bottom);
		};
		return (bottom + (2.0 * top - 1.0) * (d - bottom));
	}
}

fn blend_difference(bottom: f32, top: f32) -> f32 {
	return min(max(abs(bottom - top), 0.0), 1.0);
}

fn blend_exclusion(bottom: f32, top: f32) -> f32 {
	return bottom + top - 2.0 * bottom * top;
}

fn blend_channel(bottom: f32, top: f32) -> f32 {
	// Enum ids come from utils/colors.rs/BlendingMode
	switch (uniforms.blend_type) {
		case 0u:  { return blend_normal(bottom, top); }
		case 1u:  { return blend_multiply(bottom, top); }
		case 2u:  { return blend_screen(bottom, top); }
		case 3u:  { return blend_overlay(bottom, top); }
		case 4u:  { return blend_darken(bottom, top); }
		case 5u:  { return blend_lighten(bottom, top); }
		case 6u:  { return blend_color_dodge(bottom, top); }
		case 7u:  { return blend_color_burn(bottom, top); }
		case 8u:  { return blend_hard_light(bottom, top); }
		case 9u:  { return blend_soft_light(bottom, top); }
		case 10u: { return blend_difference(bottom, top); }
		case 11u: { return blend_exclusion(bottom, top); }
		default: { return blend_normal(bottom, top); }
	}
}

fn blend_pixel(bottom: vec3<f32>, top: vec3<f32>) -> vec3<f32> {
	return vec3<f32>(
		blend_channel(bottom.r, top.r),
		blend_channel(bottom.g, top.g),
		blend_channel(bottom.b, top.b)
	);
}

fn blend_pixel_with_opacity(bottom: vec3<f32>, top: vec3<f32>, opacity: f32) -> vec3<f32> {
	if (opacity == 0.0) {
		return bottom;
	} else {
		let opaque_result = blend_pixel(bottom, top);
		if (opacity == 1.0) {
			return opaque_result;
		} else {
			return opaque_result * opacity + bottom * (1.0 - opacity);
		}
	}
}

@compute @workgroup_size(16, 16, 1)
fn cs_main(@builtin(global_invocation_id) global_id: vec3<u32>) {
	let texel_center = vec2<f32>(0.5);
	let position_f = vec2<f32>(global_id.xy) + texel_center;
	let position_i = vec2<i32>(global_id.xy);

	let bottom_size = vec2<f32>(textureDimensions(texture_bottom_in));
	let bottom = textureSampleLevel(texture_bottom_in, sampler_in, position_f / bottom_size, 0.0);
	let top_size = vec2<f32>(textureDimensions(texture_bottom_in));
	let top = textureSampleLevel(texture_top_in, sampler_in, position_f / top_size, 0.0);

	let frag_color = vec4<f32>(blend_pixel_with_opacity(bottom.rgb, top.rgb, uniforms.opacity * top.a), 1.0);
	textureStore(texture_out, position_i, frag_color);

	// Test - paint gray
	// textureStore(texture_out, position_i, vec4<f32>(0.75));

	// Test - paint with the bottom
	// textureStore(texture_out, position_i, vec4<f32>(bottom.rgb, 1.0));

	// Test - paint with the top
	// textureStore(texture_out, position_i, vec4<f32>(top.rgb, 1.0));

	// Test - paint with gradient
	// let size = vec2<f32>(textureDimensions(texture_out));
	// textureStore(texture_out, position_i, vec4<f32>(f32(global_id.x) / size.x, f32(global_id.y) / size.y, 1.0, 1.0));
}
