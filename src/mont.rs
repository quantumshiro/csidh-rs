use crate::{params::{self, Proj}, constants};
use crate::uint;
use crate::fp;

pub fn x_dbladd(r: &mut params::Proj, s: &mut params::Proj, p: &params::Proj, q: &params::Proj, pq: &params::Proj, a: &params::Proj) {
    let mut w: params::Fp =  params::Fp { c: [0; params::LIMBS] };
    let mut x: params::Fp =  params::Fp { c: [0; params::LIMBS] };
    let mut y: params::Fp =  params::Fp { c: [0; params::LIMBS] };
    let mut z: params::Fp =  params::Fp { c: [0; params::LIMBS] };

    fp::fp_add3(&mut w, &q.x, &q.z);
    fp::fp_sub3(&mut x, &q.x, &q.z);
    fp::fp_add3(&mut y, &p.x, &p.z);
    fp::fp_sub3(&mut z, &p.x, &p.z);

    fp::fp_sq2(&mut r.x, &y);
    fp::fp_sq2(&mut s.x, &z);

    fp::fp_mul2(&mut y, &x);
    fp::fp_mul2(&mut z, &w);

    fp::fp_sub3(&mut x, &r.x, &s.x);
    fp::fp_add3(&mut w, &a.x, &a.z);

    // fp_mul3(&R->z, &a, &S->x);
    fp::fp_mul3(&mut r.z, &w, &s.x);

    /*
    fp_add3(&S->x, &A->x, &a);
    fp_add2(&R->z, &R->z); /* multiplication by 2 */
     */

    fp::fp_add3(&mut s.x, &a.x, &w);
    
    let mut tmp: params::Proj = params::Proj { x: params::Fp { c: [0; params::LIMBS] }, z: params::Fp { c: [0; params::LIMBS] } };
    for i in 0..params::LIMBS {
        tmp.x.c[i] = r.z.c[i];
    }
    
    fp::fp_add2(&mut r.z, &tmp.z);

    /*
    fp_mul2(&R->x, &R->z);
    fp_mul2(&S->x, &b);
     */
    fp::fp_mul2(&mut r.x, &r.z);
    fp::fp_mul2(&mut s.x, &x);

    /*
        fp_sub3(&S->z, &c, &d);
        fp_add2(&R->z, &S->x);
        fp_add3(&S->x, &c, &d);
    */
    fp::fp_sub3(&mut s.z, &y, &z);
    fp::fp_add2(&mut r.z, &s.x);
    fp::fp_add3(&mut s.x, &y, &z);

    /*
        fp_mul2(&R->z, &b);
        fp_sq2(&d, &S->z);
        fp_sq2(&b, &S->x);
     */
    fp::fp_mul2(&mut r.z, &x);
    fp::fp_sq2(&mut z, &s.z);
    fp::fp_sq2(&mut x, &s.x);


    /*
        fp_mul3(&S->x, &PQ->z, &b);
        fp_mul3(&S->z, &PQ->x, &d);
     */
    fp::fp_mul3(&mut s.x, &pq.z, &x);
    fp::fp_mul3(&mut s.z, &pq.x, &z);
}

pub fn x_dbl(q: &mut params::Proj, a: &params::Proj, p: &params::Proj) {
    let mut x: params::Fp =  params::Fp { c: [0; params::LIMBS] };
    let mut y: params::Fp =  params::Fp { c: [0; params::LIMBS] };
    let mut z: params::Fp =  params::Fp { c: [0; params::LIMBS] };

    /*
    fp_add3(&a, &P->x, &P->z);
    fp_sq1(&a);
    fp_sub3(&b, &P->x, &P->z);
    fp_sq1(&b);
    fp_sub3(&c, &a, &b);
    fp_add2(&b, &b); fp_add2(&b, &b); /* multiplication by 4 */
     */
    fp::fp_add3(&mut x, &p.x, &p.z);
    fp::fp_sq1(&mut x);
    fp::fp_sub3(&mut y, &p.x, &p.z);
    fp::fp_sq1(&mut y);
    fp::fp_sub3(&mut z, &x, &y);

    let mut tmp: params::Fp = params::Fp { c: [0; params::LIMBS] };
    for i in 0..params::LIMBS {
        tmp.c[i] = y.c[i];
    }
    fp::fp_add2(&mut y, &tmp);
    fp::fp_add2(&mut y, &tmp);

    /*
    fp_mul2(&b, &A->z);
    fp_mul3(&Q->x, &a, &b);
    fp_add3(&a, &A->z, &A->z); /* multiplication by 2 */
    fp_add2(&a, &A->x);
    fp_mul2(&a, &c);
    fp_add2(&a, &b);
    fp_mul3(&Q->z, &a, &c);
     */
    fp::fp_mul2(&mut y, &a.z);
    fp::fp_mul3(&mut q.x, &x, &y);
    fp::fp_add2(&mut x, &a.z);
    fp::fp_add2(&mut x, &a.x);
    fp::fp_mul2(&mut x, &z);
    fp::fp_add2(&mut x, &y);
    fp::fp_mul3(&mut q.z, &x, &z);

}

pub fn x_add(s: &mut params::Proj, p: &params::Proj, q: &params::Proj, pq: &params::Proj) {
    let mut w: params::Fp =  params::Fp { c: [0; params::LIMBS] };
    let mut x: params::Fp =  params::Fp { c: [0; params::LIMBS] };
    let mut y: params::Fp =  params::Fp { c: [0; params::LIMBS] };
    let mut z: params::Fp =  params::Fp { c: [0; params::LIMBS] };

    /*
    fp_add3(&a, &P->x, &P->z);
    fp_sub3(&b, &P->x, &P->z);
    fp_add3(&c, &Q->x, &Q->z);
    fp_sub3(&d, &Q->x, &Q->z);
    fp_mul2(&a, &d);
    fp_mul2(&b, &c);
    fp_add3(&c, &a, &b);
    fp_sub3(&d, &a, &b);
    fp_sq1(&c);
    fp_sq1(&d);
    fp_mul3(&S->x, &PQ->z, &c);
    fp_mul3(&S->z, &PQ->x, &d);
     */
    fp::fp_add3(&mut x, &p.x, &p.z);
    fp::fp_sub3(&mut y, &p.x, &p.z);
    fp::fp_add3(&mut z, &q.x, &q.z);
    fp::fp_sub3(&mut w, &q.x, &q.z);
    fp::fp_mul2(&mut x, &w);
    fp::fp_mul2(&mut y, &z);
    fp::fp_add3(&mut z, &x, &y);
    fp::fp_sub3(&mut w, &x, &y);
    fp::fp_sq1(&mut z);
    fp::fp_sq1(&mut w);
    fp::fp_mul3(&mut s.x, &pq.z, &z);
    fp::fp_mul3(&mut s.z, &pq.x, &w);

}

/* Montgomery ladder. */
/* P must not be the unique point of order 2. */
/* not constant-time! */
pub fn xMUL(q: &mut params::Proj, a: &params::Proj, p: &params::Proj, k: &params::UInt) {
    let mut r: params::Proj = *p;

    let pcopy: params::Proj = *p;

    q.x = constants::FP_1;
    q.z = constants::FP_0;

    let mut i: u64 = (64 * params::LIMBS).try_into().unwrap();
    while i > 0 && !uint::uint_bit(k, i) {
        i -= 1;
    }

    loop {
        let bit: bool = !uint::uint_bit(k, i);

        
        if bit {
            std::mem::swap(&mut (*q), &mut r);
        }
        
        let mut q_tmp = params::Proj { x: constants::FP_0, z: constants::FP_0 };
        let mut r_tmp = params::Proj { x: constants::FP_0, z: constants::FP_0 };
        for i in 0..params::LIMBS {
            q_tmp.x.c[i] = q.x.c[i];
            q_tmp.z.c[i] = q.z.c[i];
            r_tmp.x.c[i] = r.x.c[i];
            r_tmp.z.c[i] = r.z.c[i];
        }

        x_dbladd(q, &mut r, &q_tmp, &r_tmp, &pcopy, a);

        if bit {
            std::mem::swap(&mut (*q), &mut r);
        }
        i -= 1;
    }
}

/* computes the isogeny with kernel point K of order k */
/* returns the new curve coefficient A and the image of P */
/* (obviously) not constant time in k */
pub fn x_isog(a: &mut params::Proj, p: &mut params::Proj, k: &params::Proj, l: u64) {
    assert!(l >= 3);
    assert!(l % 2 == 1);

    let mut tmp0: params::Fp = params::Fp { c: [0; params::LIMBS] };
    let mut tmp1: params::Fp = params::Fp { c: [0; params::LIMBS] };

    let mut t: [params::Fp; 4] = [
        k.z,
        k.x,
        k.x,
        k.x,
    ];

    let mut q: params::Proj = params::Proj { x: constants::FP_0, z: constants::FP_0 };

    fp::fp_mul3(&mut q.x, &p.x, &k.x);
    fp::fp_mul3(&mut tmp0, &p.z, &k.z);
    fp::fp_sub2(&mut q.x, &tmp0);

    fp::fp_mul3(&mut q.z, &p.x, &k.z);
    fp::fp_mul3(&mut tmp0, &p.z, &k.x);
    fp::fp_sub2(&mut q.z, &tmp0);

    let mut m: [params::Proj; 3] = [
        params::Proj { x: k.x, z: k.z },
        params::Proj { x: constants::FP_0, z: constants::FP_0 },
        params::Proj { x: constants::FP_0, z: constants::FP_0 },
    ];

    x_dbl(&mut m[1], a, k);

    let mut m_tmp0: [params::Proj; 3] = [
        params::Proj { x: constants::FP_0, z: constants::FP_0 },
        params::Proj { x: constants::FP_0, z: constants::FP_0 },
        params::Proj { x: constants::FP_0, z: constants::FP_0 },
    ];
    let mut m_tmp1: [params::Proj; 3] = [
        params::Proj { x: constants::FP_0, z: constants::FP_0 },
        params::Proj { x: constants::FP_0, z: constants::FP_0 },
        params::Proj { x: constants::FP_0, z: constants::FP_0 },
    ];

    for i in 1..l/2 {
        m_tmp0[i as usize] = m[i as usize];
        m_tmp1[i as usize] = m[i as usize];

        if i >= 2 {
            x_add(&mut m[(i % 3) as usize], &m_tmp0[((i - 1) % 3) as usize], k, &m_tmp1[((i - 2) % 3) as usize]);
        }

        fp::fp_mul3(&mut tmp0, &m[(i % 3) as usize].x, &t[0]);
        fp::fp_mul3(&mut tmp1, &m[(i % 3) as usize].z, &t[1]);
        fp::fp_add3(&mut t[0], &tmp0, &tmp1);

        fp::fp_mul2(&mut t[1], &m[(i % 3) as usize].x);

        fp::fp_mul3(&mut tmp0, &m[(i % 3) as usize].z, &t[2]);
        fp::fp_mul3(&mut tmp1, &m[(i % 3) as usize].x, &t[3]);
        fp::fp_add3(&mut t[2], &tmp0, &tmp1);

        fp::fp_mul2(&mut t[3], &m[(i % 3) as usize].z);

        fp::fp_mul3(&mut tmp0, &p.x, &m[(i % 3) as usize].x);
        fp::fp_mul3(&mut tmp1, &p.z, &m[(i % 3) as usize].z);
        fp::fp_sub2(&mut tmp0, &tmp1);
        fp::fp_mul2(&mut q.x, &tmp0);

 
        fp::fp_mul3(&mut tmp0, &p.x, &m[(i % 3) as usize].z);
        fp::fp_mul3(&mut tmp1, &p.z, &m[(i % 3) as usize].x);
        fp::fp_sub2(&mut tmp0, &tmp1);
        fp::fp_mul2(&mut q.z, &tmp0);

    }
    let t_tmp: [params::Fp; 4] = [
        t[0],
        t[1],
        t[2],
        t[3],
    ];
    fp::fp_mul2(&mut t[0], &t_tmp[1]);
    fp::fp_add2(&mut t[0], &t_tmp[0]);

    fp::fp_sq1(&mut t[1]);

    fp::fp_mul2(&mut t[2], &t_tmp[3]);
    fp::fp_add2(&mut t[2], &t_tmp[2]);

    fp::fp_sq1(&mut t[3]);

    /* Ax := T[1] * T[3] * Ax - 3 * Az * (T[1] * T[2] - T[0] * T[3]) */

    fp::fp_mul3(&mut tmp0, &t[1], &t[2]);
    fp::fp_mul3(&mut tmp1, &t[0], &t[3]);
    fp::fp_sub2(&mut tmp0, &tmp1);
    fp::fp_mul2(&mut tmp0, &a.z);
    fp::fp_add3(&mut tmp1, &tmp0, &tmp0);

    fp::fp_mul3(&mut tmp1, &t[1], &t[3]);
    fp::fp_mul2(&mut tmp1, &a.x);

    fp::fp_sub3(&mut a.x, &tmp1, &tmp0);

    /* Az := Az * T[3]^2 */
    fp::fp_sq1(&mut t[3]);
    fp::fp_mul2(&mut a.z, &t[3]);

    /* X := X * Xim^2, Z := Z * Zim^2 */
    fp::fp_sq1(&mut q.x);
    fp::fp_sq1(&mut q.z);
    fp::fp_mul2(&mut p.x, &q.x);
    fp::fp_mul2(&mut p.z, &q.z);

}