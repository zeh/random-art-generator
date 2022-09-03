// We create an array of diff counts considering the max value each group of pixels can hold
// without overflowing the max u32 value.
//
// * Max u32 = 0xffff_ffff (4_294_967_295, or 256 * 256 * 256 * 256 - 1)
//
// The maximum difference accumulated for a channel is a byte (256). Therefore our max pixel count per
// diff field is 16_777_216 (256 * 256 * 256, or 4096 * 4096).
// This has to match the value in the Rust code, since it needs to calculate the number of
// pixel diffs contained within each sum entry in the diffs array.
// TODO: in the future, if we can have float atomics, we can use a single f32 to simplify things
let MAX_PIXEL_STRIDE: u32 = 16777216u;

struct DiffResults {
	// R, G, and B in sequence
	diffs: array<atomic<u32>, 3>, // TODO: this might be 4 for alignment? was stride(4) before
};

@group(0) @binding(0)
var sampler_in: sampler;

@group(0) @binding(1)
var texture_target_in: texture_2d<f32>;

@group(0) @binding(2)
var texture_candidate_in: texture_2d<f32>;

@group(0) @binding(3)
var<storage, read_write> buffer_diff_out: DiffResults;

@compute @workgroup_size(16, 16, 1)
fn cs_main(@builtin(global_invocation_id) global_id: vec3<u32>) {
	let position_base = vec2<f32>(global_id.xy);

	let candidate_size = vec2<f32>(textureDimensions(texture_candidate_in));
	if (position_base.x >= candidate_size.x || position_base.y >= candidate_size.y) {
		return;
	}

	let texel_center = vec2<f32>(0.5);
	let position = position_base + texel_center;

	// Calculate and accumulate differences
	let candidate = textureSampleLevel(texture_candidate_in, sampler_in, position / candidate_size, 0.0);
	let target_size = vec2<f32>(textureDimensions(texture_target_in));
	let target_sample = textureSampleLevel(texture_target_in, sampler_in, position / target_size, 0.0);

	// We accumulate differences per channel, multiplying and assuming it's 0...255.
	// The Rust side will use these values to calculate the proper luma-based differences.
	// We do this to avoid loss of precision by premultiplying the channels and THEN converting
	// to unsigned integers.
	let diffs_index = (global_id.x + global_id.y * u32(candidate_size.x)) / MAX_PIXEL_STRIDE * 3u;
	atomicAdd(&buffer_diff_out.diffs[diffs_index + 0u], u32(round(abs(target_sample.r - candidate.r) * 255.0)));
	atomicAdd(&buffer_diff_out.diffs[diffs_index + 1u], u32(round(abs(target_sample.g - candidate.g) * 255.0)));
	atomicAdd(&buffer_diff_out.diffs[diffs_index + 2u], u32(round(abs(target_sample.b - candidate.b) * 255.0)));
}
