struct VertexOutput {
    @builtin(position) position: vec4<f32>,
#ifdef FLAT_DEPTH
    @location(0) @interpolate(flat) flat_depth: f32,
#endif
#ifdef Y_CUTOFF
    @location(1) y_cutoff: f32,
    @location(2) world_position: vec4<f32>,
#endif
};

struct FragmentOutput {
    @location(0) colour: vec4<f32>,
#ifdef FLAT_DEPTH
    @builtin(frag_depth) frag_depth: f32,
#endif
};

struct OutlineFragmentUniform {
    @align(16)
    colour: vec4<f32>,
};

#ifdef VOLUME
@group(3) @binding(1)
var<uniform> fstage: OutlineFragmentUniform;
#endif

// BRUH
@fragment
fn fragment(in: VertexOutput) -> FragmentOutput {
#ifdef Y_CUTOFF
    if (u32(in.world_position.y > in.y_cutoff) == 1u) {
        discard;
    }
#endif
    var out: FragmentOutput;
#ifdef VOLUME
    out.colour = fstage.colour;
#endif
#ifdef FLAT_DEPTH
    out.frag_depth = in.flat_depth; 
#endif
    return out;
}