use criterion::{criterion_group, criterion_main, Criterion};
use rand::{thread_rng, Rng};
use rect_calc::{calc_rect, Circle};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();

    let num_circles = 512;
    let (min_spawn_radius, max_spawn_radius) = (25.0, 50.0);
    let (min_spawn_x, max_spawn_x) = (100.0, 500.0);
    let (min_spawn_y, max_spawn_y) = (50.0, 400.0);

    let mut circles = vec![];

    for _ in 0..num_circles {
        let rx = rng.gen_range(min_spawn_x..max_spawn_x);
        let ry = rng.gen_range(min_spawn_y..max_spawn_y);
        let rs = rng.gen_range(min_spawn_radius..max_spawn_radius);

       
        circles.push(Circle::new(rx, ry, rs));
    }

    c.bench_function("calc_rect", |b| b.iter(|| calc_rect(&circles)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
