use crate::params;
use crate::uint;
use crate::fp;

pub fn xDBLADD(r: &mut params::Proj, s: &mut params::Proj, p: &params::Proj, q: &params::Proj, pq: &params::Proj, a: &params::Proj) {
    let mut w: params::Fp =  params::Fp { c: [0; params::LIMBS] };
    let mut x: params::Fp =  params::Fp { c: [0; params::LIMBS] };
    let mut y: params::Fp =  params::Fp { c: [0; params::LIMBS] };
    let mut z: params::Fp =  params::Fp { c: [0; params::LIMBS] };

    /*
        fp_add3(&a, &Q->x, &Q->z);
    fp_sub3(&b, &Q->x, &Q->z);
    fp_add3(&c, &P->x, &P->z);
    fp_sub3(&d, &P->x, &P->z);

    fp_sq2(&R->x, &c);
    fp_sq2(&S->x, &d);

    fp_mul2(&c, &b);
    fp_mul2(&d, &a);

    fp_sub3(&b, &R->x, &S->x);
    fp_add3(&a, &A->z, &A->z); /* multiplication by 2 */

    fp_mul3(&R->z, &a, &S->x);

    fp_add3(&S->x, &A->x, &a);
    fp_add2(&R->z, &R->z); /* multiplication by 2 */

    fp_mul2(&R->x, &R->z);
    fp_mul2(&S->x, &b);

    fp_sub3(&S->z, &c, &d);
    fp_add2(&R->z, &S->x);
    fp_add3(&S->x, &c, &d);

    fp_mul2(&R->z, &b);
    fp_sq2(&d, &S->z);
    fp_sq2(&b, &S->x);

    fp_mul3(&S->x, &PQ->z, &b);
    fp_mul3(&S->z, &PQ->x, &d);
     */
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