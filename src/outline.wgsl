#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_types

//  MIT License. © Ian McEwan, Stefan Gustavson, Munrocket
//
fn permute4(x: vec4<f32>) -> vec4<f32> { return ((x * 34. + 1.) * x) % vec4<f32>(289.); }
fn taylorInvSqrt4(r: vec4<f32>) -> vec4<f32> { return 1.79284291400159 - 0.85373472095314 * r; }

fn simplexNoise3(v: vec3<f32>) -> f32 {
  let C = vec2<f32>(1. / 6., 1. / 3.);
  let D = vec4<f32>(0., 0.5, 1., 2.);

  // First corner
  var i: vec3<f32>  = floor(v + dot(v, C.yyy));
  let x0 = v - i + dot(i, C.xxx);

  // Other corners
  let g = step(x0.yzx, x0.xyz);
  let l = 1.0 - g;
  let i1 = min(g.xyz, l.zxy);
  let i2 = max(g.xyz, l.zxy);

  // x0 = x0 - 0. + 0. * C
  let x1 = x0 - i1 + 1. * C.xxx;
  let x2 = x0 - i2 + 2. * C.xxx;
  let x3 = x0 - 1. + 3. * C.xxx;

  // Permutations
  i = i % vec3<f32>(289.);
  let p = permute4(permute4(permute4(
      i.z + vec4<f32>(0., i1.z, i2.z, 1. )) +
      i.y + vec4<f32>(0., i1.y, i2.y, 1. )) +
      i.x + vec4<f32>(0., i1.x, i2.x, 1. ));

  // Gradients (NxN points uniformly over a square, mapped onto an octahedron.)
  var n_: f32 = 1. / 7.; // N=7
  let ns = n_ * D.wyz - D.xzx;

  let j = p - 49. * floor(p * ns.z * ns.z); // mod(p, N*N)

  let x_ = floor(j * ns.z);
  let y_ = floor(j - 7.0 * x_); // mod(j, N)

  let x = x_ *ns.x + ns.yyyy;
  let y = y_ *ns.x + ns.yyyy;
  let h = 1.0 - abs(x) - abs(y);

  let b0 = vec4<f32>( x.xy, y.xy );
  let b1 = vec4<f32>( x.zw, y.zw );

  let s0 = floor(b0)*2.0 + 1.0;
  let s1 = floor(b1)*2.0 + 1.0;
  let sh = -step(h, vec4<f32>(0.));

  let a0 = b0.xzyw + s0.xzyw*sh.xxyy ;
  let a1 = b1.xzyw + s1.xzyw*sh.zzww ;

  var p0: vec3<f32> = vec3<f32>(a0.xy, h.x);
  var p1: vec3<f32> = vec3<f32>(a0.zw, h.y);
  var p2: vec3<f32> = vec3<f32>(a1.xy, h.z);
  var p3: vec3<f32> = vec3<f32>(a1.zw, h.w);

  // Normalise gradients
  let norm = taylorInvSqrt4(vec4<f32>(dot(p0,p0), dot(p1,p1), dot(p2,p2), dot(p3,p3)));
  p0 = p0 * norm.x;
  p1 = p1 * norm.y;
  p2 = p2 * norm.z;
  p3 = p3 * norm.w;

  // Mix final noise value
  var m: vec4<f32> = 0.6 - vec4<f32>(dot(x0,x0), dot(x1,x1), dot(x2,x2), dot(x3,x3));
  m = max(m, vec4<f32>(0.));
  m = m * m;
  return 42. * dot(m * m, vec4<f32>(dot(p0,x0), dot(p1,x1), dot(p2,x2), dot(p3,x3)));
}

struct VertexInput {
    @location(0) position: vec3<f32>,
#ifndef OFFSET_ZERO
    @location(1) normal: vec3<f32>,
#endif
#ifdef SKINNED
    @location(2) joint_indexes: vec4<u32>,
    @location(3) joint_weights: vec4<f32>,
#endif
};

struct OutlineViewUniform {
    @align(16)
    scale: vec2<f32>,
};

struct OutlineVertexUniform {
    @align(16)
    origin: vec3<f32>,
    offset: f32,
};

struct OutlineAnimationUniform {
    @align(8)
    time: f32,
    rate: f32,
};

@group(1) @binding(0)
var<uniform> mesh: Mesh;

#ifdef SKINNED
@group(1) @binding(1)
var<uniform> joint_matrices: SkinnedMesh;
#import bevy_pbr::skinning
#endif

@group(2) @binding(0)
var<uniform> view_uniform: OutlineViewUniform;

@group(3) @binding(0)
var<uniform> vstage: OutlineVertexUniform;

@group(4) @binding(0)
var<uniform> animation: OutlineAnimationUniform;

fn mat4to3(m: mat4x4<f32>) -> mat3x3<f32> {
    return mat3x3<f32>(
        m[0].xyz, m[1].xyz, m[2].xyz
    );
}

fn model_origin_z(plane: vec3<f32>, view_proj: mat4x4<f32>) -> f32 {
    var proj_zw = mat4x2<f32>(
        view_proj[0].zw, view_proj[1].zw,
        view_proj[2].zw, view_proj[3].zw);
    var zw = proj_zw * vec4<f32>(plane, 1.0);
    return zw.x / zw.y;
}

@vertex
fn vertex(vertex: VertexInput) -> @builtin(position) vec4<f32> {
    let MIN_SCALE = 0.7;
    var p = vertex.position;

    if i32(floor(animation.time / animation.rate)) % 2 == 0 {
        p = p + vec3(1000f);
    }

    let scale = MIN_SCALE + simplexNoise3(p) * (1.0-MIN_SCALE);

#ifdef SKINNED
    let model = skin_model(vertex.joint_indexes, vertex.joint_weights);
#else
    let model = mesh.model;
#endif
    let clip_pos = view.view_proj * (model * vec4<f32>(vertex.position, 1.0));
#ifdef FLAT_DEPTH
    let out_zw = vec2<f32>(model_origin_z(vstage.origin, view.view_proj) * clip_pos.w, clip_pos.w);
#else
    let out_zw = clip_pos.zw;
#endif
#ifdef OFFSET_ZERO
    let out_xy = clip_pos.xy;
#else
    let clip_norm = mat4to3(view.view_proj) * (mat4to3(model) * vertex.normal);

    let ndc_delta = vstage.offset * normalize(clip_norm.xy) * view_uniform.scale * out_zw.y * scale;
    let out_xy = clip_pos.xy + ndc_delta;
#endif
    return vec4<f32>(out_xy, out_zw);
}
