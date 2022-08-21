fn rand2(n: vec2<f32>) -> f32 {
  return fract(sin(dot(n, vec2<f32>(12.9898, 4.1414))) * 43758.5453);
}

fn noise2(n: vec2<f32>) -> f32 {
  let d = vec2<f32>(0., 1.);
  let b = floor(n);
  let f = smoothStep(vec2<f32>(0.), vec2<f32>(1.), fract(n));
  return mix(mix(rand2(b), rand2(b + d.yx), f.x), mix(rand2(b + d.xy), rand2(b + d.yy), f.x), f.y);
}

var m2: mat2x2<f32> = mat2x2<f32>(vec2<f32>(0.8, 0.6), vec2<f32>(-0.6, 0.8));
fn fbm(p: vec2<f32>) -> f32 {
  var f: f32 = 0.;
  f = f + 0.5000 * noise2(p); p = m2 * p * 2.02;
  f = f + 0.2500 * noise2(p); p = m2 * p * 2.03;
  f = f + 0.1250 * noise2(p); p = m2 * p * 2.01;
  f = f + 0.0625 * noise2(p);
  return f / 0.9375;
}