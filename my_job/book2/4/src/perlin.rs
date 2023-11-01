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
    pub fn noise(&self, p: Point3) -> f64 {
        let i = (((4. * p.x) as isize) & 255) as usize;
        let j = (((4. * p.y) as isize) & 255) as usize;
        let k = (((4. * p.z) as isize) & 255) as usize;
        self.ranfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }
}
