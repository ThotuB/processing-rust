static SOURCE: [u8; 256] = [
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

static mut PERM: [u8; 512] = {
    let mut perm = [0; 512];
    let mut i = 0;
    while i < 256 {
        perm[i] = SOURCE[i];
        perm[i + 256] = SOURCE[i];
        i += 1;
    }
    perm
};

const SQRT3: f64 = 1.7320508075688772935;
const SQRT5: f64 = 2.2360679774997896964;

const F2: f64 = 0.5 * (SQRT3 - 1.0);
const G2: f64 = (3.0 - SQRT3) / 6.0;
const G22: f64 = G2 * 2.0 - 1.0;

const F3: f64 = 1.0 / 3.0;
const G3: f64 = 1.0 / 6.0;

const F4: f64 = (SQRT5 - 1.0) / 4.0;
const G4: f64 = (5.0 - SQRT5) / 20.0;
const G42: f64 = G4 * 2.0;
const G43: f64 = G4 * 3.0;
const G44: f64 = G4 * 4.0 - 1.0;

const GRAD1: [f64; 2] = [1.0, -1.0];

const GRAD2: [[f64; 2]; 8] = [
    [1.0, 0.0],
    [-1.0, 0.0],
    [0.0, 1.0],
    [0.0, -1.0],
    [1.0, 1.0],
    [-1.0, 1.0],
    [1.0, -1.0],
    [-1.0, -1.0],
];

const GRAD3: [[f64; 3]; 12] = [
    [1.0, 1.0, 0.0],
    [-1.0, 1.0, 0.0],
    [1.0, -1.0, 0.0],
    [-1.0, -1.0, 0.0],
    [1.0, 0.0, 1.0],
    [-1.0, 0.0, 1.0],
    [1.0, 0.0, -1.0],
    [-1.0, 0.0, -1.0],
    [0.0, 1.0, 1.0],
    [0.0, -1.0, 1.0],
    [0.0, 1.0, -1.0],
    [0.0, -1.0, -1.0],
];

pub fn seed(seed: u32) {
    let f1 = (seed & 0xff) as u8;
    let f2 = ((seed >> 8) & 0xff) as u8;
    let f3 = ((seed >> 16) & 0xff) as u8;
    let f4 = ((seed >> 24) & 0xff) as u8;

    let mut i = 0;
    while i < 256 {
        let perm = SOURCE[i] ^ f1 ^ f2 ^ f3 ^ f4;
        unsafe {
            PERM[i] = perm;
            PERM[i + 256] = perm;
        }
        i += 1;
    }
}

#[inline(always)]
fn fast_floor(x: f64) -> i32 {
    if x > 0.0 {
        x as i32
    } else {
        (x as i32) - 1
    }
}

#[inline(always)]
fn dot2d(g: [f64; 2], x: f64, y: f64) -> f64 {
    g[0] * x + g[1] * y
}

#[inline(always)]
fn dot3d(g: [f64; 3], x: f64, y: f64, z: f64) -> f64 {
    g[0] * x + g[1] * y + g[2] * z
}

#[inline(always)]
fn dot4d(g: [f64; 4], x: f64, y: f64, z: f64, w: f64) -> f64 {
    g[0] * x + g[1] * y + g[2] * z + g[3] * w
}

pub fn noise3d(x: f64, y: f64, z: f64) -> f64 {
    // noise contributions from the four corners
    // skew the input space to determine which simplex cell we're in
    let s = (x + y + z) * F3;

    // for 3D
    let i = fast_floor(x + s);
    let j = fast_floor(y + s);
    let k = fast_floor(z + s);

    let t = (i + j + k) as f64 * G3;

    // the x, y, z distances from the cell origin
    let x0 = x - (i as f64 - t);
    let y0 = y - (j as f64 - t);
    let z0 = z - (k as f64 - t);

    // for the 3D case, the simplex shape is a slightly irregular tetrahedron.
    // determine which simplex we are in.
    // offsets for second corner of simplex in (i,j,k)
    let i1;
    let j1;
    let k1;

    // coords
    // offsets for third corner of simplex in (i,j,k)
    let i2;
    let j2;
    let k2;

    if x0 >= y0 {
        if y0 >= z0 {
            // X Y Z order
            i1 = 1;
            j1 = 0;
            k1 = 0;
            i2 = 1;
            j2 = 1;
            k2 = 0;
        } else if x0 >= z0 {
            // X Z Y order
            i1 = 1;
            j1 = 0;
            k1 = 0;
            i2 = 1;
            j2 = 0;
            k2 = 1;
        } else {
            // Z X Y order
            i1 = 0;
            j1 = 0;
            k1 = 1;
            i2 = 1;
            j2 = 0;
            k2 = 1;
        }
    } else if y0 < z0 {
        // Z Y X order
        i1 = 0;
        j1 = 0;
        k1 = 1;
        i2 = 0;
        j2 = 1;
        k2 = 1;
    } else if x0 < z0 {
        // Y Z X order
        i1 = 0;
        j1 = 1;
        k1 = 0;
        i2 = 0;
        j2 = 1;
        k2 = 1;
    } else {
        // Y X Z order
        i1 = 0;
        j1 = 1;
        k1 = 0;
        i2 = 1;
        j2 = 1;
        k2 = 0;
    }

    // offsets for the second corner in (x,y,z) coords
    let x1 = x0 - i1 as f64 + G3;
    let y1 = y0 - j1 as f64 + G3;
    let z1 = z0 - k1 as f64 + G3;

    // offsets for the third corner in (x,y,z) coords
    let x2 = x0 - i2 as f64 + F3;
    let y2 = y0 - j2 as f64 + F3;
    let z2 = z0 - k2 as f64 + F3;

    // offsets for the last corner in (x,y,z) coords
    let x3 = x0 - 0.5;
    let y3 = y0 - 0.5;
    let z3 = z0 - 0.5;

    // work out the hashed gradient indices of the four simplex corners
    let ii = (i & 0xff) as usize;
    let jj = (j & 0xff) as usize;
    let kk = (k & 0xff) as usize;

    // calculate the contribution from the four corners
    let t0 = 0.6 - x0 * x0 - y0 * y0 - z0 * z0;

    let n0 = if t0 < 0.0 {
        0.0
    } else {
        let gi0 = unsafe { PERM[ii + PERM[jj + PERM[kk] as usize] as usize] % 12 };
        let grad0 = GRAD3[gi0 as usize];
        t0 * t0 * t0 * t0 * dot3d(grad0, x0, y0, z0)
    };

    let t1 = 0.6 - x1 * x1 - y1 * y1 - z1 * z1;
    let n1 = if t1 < 0.0 {
        0.0
    } else {
        let gi1 = unsafe { PERM[ii + i1 + PERM[jj + j1 + PERM[kk + k1] as usize] as usize] % 12 };
        let grad1 = GRAD3[gi1 as usize];
        t1 * t1 * t1 * t1 * dot3d(grad1, x1, y1, z1)
    };

    let t2 = 0.6 - x2 * x2 - y2 * y2 - z2 * z2;
    let n2 = if t2 < 0.0 {
        0.0
    } else {
        let gi2 = unsafe { PERM[ii + i2 + PERM[jj + j2 + PERM[kk + k2] as usize] as usize] % 12 };
        let grad2 = GRAD3[gi2 as usize];
        t2 * t2 * t2 * t2 * dot3d(grad2, x2, y2, z2)
    };

    let t3 = 0.6 - x3 * x3 - y3 * y3 - z3 * z3;
    let n3 = if t3 < 0.0 {
        0.0
    } else {
        let gi3 = unsafe { PERM[ii + 1 + PERM[jj + 1 + PERM[kk + 1] as usize] as usize] % 12 };
        let grad3 = GRAD3[gi3 as usize];
        t3 * t3 * t3 * t3 * dot3d(grad3, x3, y3, z3)
    };

    // add contributions from each corner to get the final noise value.
    // the result is scaled to return values in the interval [-1, 1]
    32.0 * (n0 + n1 + n2 + n3)
}

#[inline(always)]
pub fn noise2d(x: f64, y: f64) -> f64 {
    noise3d(x, y, 0.0)
}

#[inline(always)]
pub fn noise1d(x: f64) -> f64 {
    noise3d(x, 0.0, 0.0)
}

// pub fn noise2d(x: f64, y: f64) -> f64 {
//     // noise contributions from the three corners
//     // skew the input space to determine which simplex cell we're in
//     let s = (x + y) * F2;
//
//     // for 2D
//     let i = fast_floor(x + s);
//     let j = fast_floor(y + s);
//
//     let t = (i + j) as f64 * G2;
//
//     // the x, y distances from the cell origin
//     let x0 = x - (i as f64 - t);
//     let y0 = y - (j as f64 - t);
//
//     // for the 2D case, the simplex shape is an equilateral triangle.
//     // determine which simplex we are in.
//     // offsets for second (middle) corner of simplex in (i,j) coords
//     let i1;
//     let j1;
//
//     // coords
//     // offsets for third corner of simplex in (i,j) coords
//     let i2;
//     let j2;
//
//     if x0 > y0 {
//         i1 = 1;
//         j1 = 0;
//     } else {
//         i1 = 0;
//         j1 = 1;
//     }
//
//     // offsets for the second corner in (x,y) coords
//     let x1 = x0 - i1 as f64 + G2;
//     let y1 = y0 - j1 as f64 + G2;
//
//     // offsets for the third corner in (x,y) coords
//     let x2 = x0 - 1.0 + F2;
//     let y2 = y0 - 1.0 + F2;
//
//     // work out the hashed gradient indices of the three simplex corners
//     let ii = (i & 0xff) as usize;
//     let jj = (j & 0xff) as usize;
//
//     // calculate the contribution from the three corners
//     let t0 = 0.5 - x0 * x0 - y0 * y0;
//     let n0 = if t0 < 0.0 {
//         0.0
//     } else {
//         let gi0 = PERM[ii + PERM[jj] as usize] % 12;
//         let grad0 = GRAD2[gi0 as usize];
//         t0 * t0 * t0 * t0 * dot2d(grad0, x0, y0)
//     };
//
//     let t1 = 0.5 - x1 * x1 - y1 * y1;
//     let n1 = if t1 < 0.0 {
//         0.0
//     } else {
//         let gi1 = PERM[ii + i1 + PERM[jj + j1] as usize] % 12;
//         let grad1 = GRAD2[gi1 as usize];
//         t1 * t1 * t1 * t1 * dot2d(grad1, x1, y1)
//     };
//
//     let t2 = 0.5 - x2 * x2 - y2 * y2;
//     let n2 = if t2 < 0.0 {
//         0.0
//     } else {
//         let gi2 = PERM[ii + 1 + PERM[jj + 1] as usize] % 12;
//         let grad2 = GRAD2[gi2 as usize];
//         t2 * t2 * t2 * t2 * dot2d(grad2, x2, y2)
//     };
//
//     // add contributions from each corner to get the final noise value.
//     // the result is scaled to return values in the interval [-1, 1]
//     70.0 * (n0 + n1 + n2)
// }
