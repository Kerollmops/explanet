#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var base_texture: texture_2d<f32>;
@group(0) @binding(1) var base_sampler: sampler;
struct SunSettings {
    time: f32,
    aspect: f32,
    #ifdef SIXTEEN_BYTE_ALIGNMENT
    // WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: vec3<f32>
    #endif
}
@group(0) @binding(2) var<uniform> settings: SunSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // Can I make this a single pass? It will output the same thing, always.
    // let freq_0 = textureSample(noise_texture, noise_sampler, vec2(0.01, 0.25)).x;
    // let freq_1 = textureSample(noise_texture, noise_sampler, vec2(0.07, 0.25)).x;
    // let freq_2 = textureSample(noise_texture, noise_sampler, vec2(0.15, 0.25)).x;
    // let freq_3 = textureSample(noise_texture, noise_sampler, vec2(0.30, 0.25)).x;

    let freq_0 = 0.4568;
    let freq_1 = 0.6768;

    let brightness = freq_0 * 0.25 + freq_1 * 0.25;
    let radius = 0.24 + brightness * 0.2;
    let invRadius = 1.0 / radius;

    let orange = vec3(0.8, 0.65, 0.3);
    let orangeRed = vec3(0.8, 0.35, 0.1);
    let time = settings.time * 0.1;
    var p = -0.5 + in.uv;
    p.x *= settings.aspect;

    let fade = pow(length(2.0 * p), 0.5);
    var fVal1 = 1.0 - fade;
    var fVal2 = 1.0 - fade;

    let angle = atan2(p.x, p.y) / 6.2832;
    let dist = length(p);
    let coord = vec3(angle, dist, time * 0.1);

    let newTime1 = abs(snoise(coord + vec3(0.0, -time * (0.35 + brightness * 0.001), time * 0.015), 15.0));
    let newTime2 = abs(snoise(coord + vec3(0.0, -time * (0.15 + brightness * 0.001), time * 0.015), 45.0));
    for (var i = 1; i <= 7; i++) {
        let power = pow(2.0, f32(i + 1));
        fVal1 += (0.5 / power) * snoise(coord + vec3(0.0, -time, time * 0.2), (power * 10.0 * (newTime1 + 1.0)));
        fVal2 += (0.5 / power) * snoise(coord + vec3(0.0, -time, time * 0.2), (power * 25.0 * (newTime2 + 1.0)));
    }

    var corona = pow(fVal1 * max(1.1 - fade, 0.0), 2.0) * 50.0;
    corona += pow(fVal2 * max(1.1 - fade, 0.0), 2.0) * 50.0;
    corona *= 1.2 - newTime1;
    let sphereNormal = vec3(0.0, 0.0, 1.0);
    let dir = vec3(0.0);
    let center = vec3(0.5, 0.5, 1.0);
    var starSphere = vec3(0.0);
    var newUv = vec2(0.0);

    var sp = -1.0 + 2.0 * in.uv;
    sp.x *= settings.aspect;
    sp *= ( 2.0 - brightness );
    let r = dot(sp,sp);
    let f = (1.0 - sqrt(abs(1.0 - r))) / r + brightness * 0.5;
    if (dist < radius) {
        corona *= pow( dist * invRadius, 24.0 );
        newUv = vec2(sp.x * f, sp.y * f) + vec2(time, 0.0);
    }

    let texSample = textureSample(base_texture, base_sampler, newUv).rgb;
    let uOff = texSample.g * brightness * 4.5 + time;
    let starUv = newUv + vec2(uOff, 0.0);
    starSphere = textureSample(base_texture, base_sampler, starUv).rgb;

    if (dist >= radius) {
    	starSphere = vec3(0.0);
    }

    let starGlow  = min(max(1.0 - dist * (1.0 - brightness), 0.0), 1.0);
    let rgb = vec3(f * (0.75 + brightness * 0.3) * orange ) + starSphere + corona * orange + starGlow * orangeRed;

    return vec4(rgb, 1.0);
}

fn modulo(a: vec3<f32>, v: f32) -> vec3<f32> {
    return vec3(
        a.x - v * floor(a.x / v),
        a.y - v * floor(a.y / v),
        a.z - v * floor(a.z / v),
    );
}

fn snoise(in_uv: vec3<f32>, res: f32) -> f32 {
    let s = vec3(1e0, 1e2, 1e4);

    let uv = in_uv * res;

    let uv0 = floor(modulo(uv, res)) * s;
    let uv1 = floor(modulo(uv + vec3(1.0), res)) * s;

    var f = fract(uv);
    f = f * f * (3.0 - 2.0 * f);

    let v = vec4(uv0.x + uv0.y + uv0.z, uv1.x + uv0.y + uv0.z,
       uv0.x + uv1.y + uv0.z, uv1.x + uv1.y + uv0.z);

    var r = fract(sin(v * 1e-3) * 1e5);
    let r0 = mix(mix(r.x, r.y, f.x), mix(r.z, r.w, f.x), f.y);

    r = fract(sin((v + uv1.z - uv0.z) * 1e-3) * 1e5);
    let r1 = mix(mix(r.x, r.y, f.x), mix(r.z, r.w, f.x), f.y);

    return mix(r0, r1, f.z) * 2.0 - 1.0;
}
