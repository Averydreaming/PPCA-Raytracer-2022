use super::vec3::Point3;
use rand::Rng;
const POINT_COUNT: usize = 256;

pub struct Perlin {
    pub ranfloat: [f64; POINT_COUNT],
    pub perm_x: [usize; POINT_COUNT],
    pub perm_y: [usize; POINT_COUNT],
    pub perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut ranfloat = [0.; POINT_COUNT];
        let mut permx = [0; POINT_COUNT];
        let mut permy = [0; POINT_COUNT];
        let mut permz = [0; POINT_COUNT];
        let mut rng = rand::thread_rng();
        for i in 0..POINT_COUNT {
            ranfloat[i] = rng.gen();
        }
        Perlin::perline_generate_perm(&mut permx);
        Perlin::perline_generate_perm(&mut permy);
        Perlin::perline_generate_perm(&mut permz);

        Self {
            ranfloat: ranfloat,
            perm_x: permx,
            perm_y: permy,
            perm_z: permz,
        }
    }
    pub fn perline_generate_perm(p: &mut [usize; POINT_COUNT]) {
        for i in 0..POINT_COUNT {
            p[i] = i;
        }
        for i in (0..POINT_COUNT).rev() {
            let target = rand::thread_rng().gen_range(0..i + 1);
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }
    }
    pub fn permute(p: &mut [usize; POINT_COUNT], n: usize) {
        for i in n - 1..0 {
            let target = rand::thread_rng().gen_range(0..i + 1);
            let tmp = p[i] as usize;
            p[i] = p[target];
            p[target] = tmp;
        }
    }
    pub fn TtoO(x: usize, y: usize, z: usize) -> usize {
        return x * 4 + y * 2 + z;
    }
    pub fn trilinear_interp(c: &[f64; POINT_COUNT], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum = accum
                        + (i as f64 * u + (1.0 - i as f64) * (1.0 - u))
                            * (j as f64 * v + (1.0 - j as f64) * (1.0 - v))
                            * (k as f64 * w + (1.0 - k as f64) * (1.0 - w))
                            * (c[Perlin::TtoO(i, j, k)]);
                }
            }
        }
        return accum;
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let mut u = p.x - p.x.floor();
        let mut v = p.y - p.y.floor();
        let mut w = p.z - p.z.floor();
        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);
        let i = (p.x.floor()) as i32;
        let j = (p.y.floor()) as i32;
        let k = (p.z.floor()) as i32;
        let mut c = [0.; POINT_COUNT];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[Perlin::TtoO(di, dj, dk)] = self.ranfloat[self.perm_x
                        [((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]];
                }
            }
        }
        return Perlin::trilinear_interp(&c, u, v, w);
    }
}
