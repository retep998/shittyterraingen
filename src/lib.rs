#![crate_type = "cdylib"]
#![cfg_attr(test, feature(test))]

extern crate noise;
#[macro_use] extern crate lazy_static;

use noise::{Seed, open_simplex2};

const SIZE: usize = 16;

fn coord(c: u32, n: usize) -> f64 {
    (c as f64) * (SIZE as f64) + (n as f64)
}

fn get_seed() -> &'static Seed {
    lazy_static! {
        static ref SEED: Seed = {
            Seed::new(273)
        };
    }
    &SEED
}
fn gen(seed: &Seed, x: f64, y: f64, m: f64) -> f64 {
    open_simplex2(seed, &[x * m, y * m])
}


#[no_mangle]
pub extern "stdcall" fn generate(buf: &mut [[[u16; SIZE]; SIZE]; SIZE], cx: u32, cy: u32, cz: u32) -> i32 {
    let seed = get_seed();
    for x in 0..SIZE {
        let xx = coord(cx, x);
        for z in 0..SIZE {
            let zz = coord(cz, z);
            let h = 512.
                + gen(seed, xx, zz, 0.004) * 256.
                + gen(seed, xx, zz, 0.008) * 128.
                + gen(seed, xx, zz, 0.016) * 64.
                + gen(seed, xx, zz, 0.032) * 32.
                + gen(seed, xx, zz, 0.064) * 16.
                + gen(seed, xx, zz, 0.128) * 8.;
            for y in 0..SIZE {
                let yy = coord(cy, y);
                buf[z][y][x] = if yy > h { 0 } else { 1 }
            }
        }
    }
    return 273;
}

#[cfg(test)] mod tests {
    extern crate test;
    use self::test::Bencher;
    #[bench]
    fn bench_gen(b: &mut Bencher) {
        let mut buf = [[[0; ::SIZE]; ::SIZE]; ::SIZE];
        b.iter(|| ::generate(&mut buf, 0, 0, 0));
    }
}