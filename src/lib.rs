use rand::{seq::SliceRandom, Rng};

const PERMUTATION_TABLE: [u8; 256] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180,
];

fn smoother_step(w: f64) -> f64 {
    (w * (w * 6. - 15.) + 10.) * w * w * w
}

fn gradient_dot_weighted(
    (offset_x, offset_y): (f64, f64),
    (gradient_x, gradient_y): (f64, f64),
) -> f64 {
    smoother_step(1. - offset_x.abs())
        * smoother_step(1. - offset_y.abs())
        * ((gradient_x * offset_x) + (gradient_y * offset_y))
}

pub struct Perlin2 {
    grads: Vec<(f64, f64)>,
}

impl Perlin2 {
    pub fn new<R: Rng>(rng: &mut R) -> Self {
        let mut grads = Vec::with_capacity(256);
        for i in 0..256 {
            let angle = (std::f64::consts::PI * 2. * (i as f64)) / 256.;
            grads.push((angle.cos(), angle.sin()));
        }
        grads.shuffle(rng);
        Self { grads }
    }

    pub fn noise(&self, (x, y): (f64, f64)) -> f64 {
        let left_x = x.floor() as i32;
        let right_x = left_x + 1;
        let top_y = y.floor() as i32;
        let bottom_y = top_y + 1;
        // top
        let mut gradient_index_y = PERMUTATION_TABLE[(top_y & 0xFF) as usize];
        // top left
        let mut gradient_index =
            PERMUTATION_TABLE[((gradient_index_y as i32 + left_x) & 0xFF) as usize];
        let mut ret = gradient_dot_weighted(
            (x - left_x as f64, y - top_y as f64),
            self.grads[gradient_index as usize],
        );
        // top right
        gradient_index = PERMUTATION_TABLE[((gradient_index_y as i32 + right_x) & 0xFF) as usize];
        ret += gradient_dot_weighted(
            (x - right_x as f64, y - top_y as f64),
            self.grads[gradient_index as usize],
        );
        // bottom
        gradient_index_y = PERMUTATION_TABLE[(bottom_y & 0xFF) as usize];
        // bottom left
        gradient_index = PERMUTATION_TABLE[((gradient_index_y as i32 + left_x) & 0xFF) as usize];
        ret += gradient_dot_weighted(
            (x - left_x as f64, y - bottom_y as f64),
            self.grads[gradient_index as usize],
        );
        // bottom right
        gradient_index = PERMUTATION_TABLE[((gradient_index_y as i32 + right_x) & 0xFF) as usize];
        ret += gradient_dot_weighted(
            (x - right_x as f64, y - bottom_y as f64),
            self.grads[gradient_index as usize],
        );
        ret
    }

    pub fn noise01(&self, coord: (f64, f64)) -> f64 {
        (1. + self.noise(coord)) / 2.
    }
}
