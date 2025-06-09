[[group(0), binding(0)]] var u_texture: texture_2d<f32>;
[[group(0), binding(1)]] var u_sampler: sampler;

[[group(0), binding(2)]] var<uniform> u_shadow_params: ShadowParams;

struct ShadowParams {
    shadow_offset: vec2<f32>;
    shadow_color: vec4<f32>;
    shadow_opacity: f32;
    shadow_spread: f32;
    shadow_blur: f32;
};

[[stage(vertex)]]
fn vertex_main(
    [[location(0)]] position: vec2<f32>,
    [[location(1)]] uv: vec2<f32>
) -> [[builtin(position)]] vec4<f32> {
    return vec4<f32>(position, 0.0, 1.0);
}

[[stage(fragment)]]
fn fragment_main(
    [[location(0)]] uv: vec2<f32>
) -> [[location(0)]] vec4<f32> {
    let original = textureSample(u_texture, u_sampler, uv);

    var shadow_color: vec4<f32> = vec4<f32>(0.0);

    // Define the sampling pattern for the shadow blur
    let offsets: array<vec2<f32>, 9> = array<vec2<f32>, 9>(
        vec2<f32>(-1.0, -1.0), vec2<f32>(0.0, -1.0), vec2<f32>(1.0, -1.0),
        vec2<f32>(-1.0,  0.0), vec2<f32>(0.0,  0.0), vec2<f32>(1.0,  0.0),
        vec2<f32>(-1.0,  1.0), vec2<f32>(0.0,  1.0), vec2<f32>(1.0,  1.0)
    );

    for (i = 0u; i < 9u; i = i + 1u) {
        let offset_uv = uv + u_shadow_params.shadow_offset
                           + offsets[i] * u_shadow_params.shadow_spread * u_shadow_params.shadow_blur;
        let shadow_sample = textureSample(u_texture, u_sampler, offset_uv);
        shadow_color += shadow_sample.a * u_shadow_params.shadow_opacity * u_shadow_params.shadow_color;
    }

    // Average the shadow contributions
    shadow_color /= 9.0;

    return shadow_color + original;
}