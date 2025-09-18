static mut qsz: libc::c_int = 0;
static mut thresh: libc::c_int = 0;
static mut mthresh: libc::c_int = 0;
static mut qcmp: Option::<
    unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_char) -> libc::c_int,
> = None;
#[no_mangle]
pub unsafe extern "C" fn qqsort(
    mut base: *mut libc::c_char,
    mut n: libc::c_int,
    mut size: libc::c_int,
    mut compar: Option::<
        unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_char) -> libc::c_int,
    >,
) {
    let mut i: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut j: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lo: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hi: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut min: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_char = 0;
    let mut max: *mut libc::c_char = 0 as *mut libc::c_char;
    if n <= 1 as libc::c_int {
        return;
    }
    qsz = size;
    qcmp = compar;
    thresh = qsz * 4 as libc::c_int;
    mthresh = qsz * 6 as libc::c_int;
    max = base.offset((n * qsz) as isize);
    if n >= 4 as libc::c_int {
        qst(base, max);
        hi = base.offset(thresh as isize);
    } else {
        hi = max;
    }
    lo = base;
    j = lo;
    loop {
        lo = lo.offset(qsz as isize);
        if !(lo < hi) {
            break;
        }
        if (Some(qcmp.expect("non-null function pointer")))
            .expect("non-null function pointer")(j, lo) > 0 as libc::c_int
        {
            j = lo;
        }
    }
    if j != base {
        i = base;
        hi = base.offset(qsz as isize);
        while i < hi {
            c = *j;
            let fresh0 = j;
            j = j.offset(1);
            *fresh0 = *i;
            let fresh1 = i;
            i = i.offset(1);
            *fresh1 = c;
        }
    }
    min = base;
    loop {
        min = min.offset(qsz as isize);
        hi = min;
        if !(hi < max) {
            break;
        }
        loop {
            hi = hi.offset(-(qsz as isize));
            if !((Some(qcmp.expect("non-null function pointer")))
                .expect("non-null function pointer")(hi, min) > 0 as libc::c_int)
            {
                break;
            }
        }
        hi = hi.offset(qsz as isize);
        if hi != min {
            lo = min.offset(qsz as isize);
            loop {
                lo = lo.offset(-1);
                if !(lo >= min) {
                    break;
                }
                c = *lo;
                j = lo;
                i = j;
                loop {
                    j = j.offset(-(qsz as isize));
                    if !(j >= hi) {
                        break;
                    }
                    *i = *j;
                    i = j;
                }
                *i = c;
            }
        }
    };
}
unsafe extern "C" fn qst(mut base: *mut libc::c_char, mut max: *mut libc::c_char) {
    let mut i: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut j: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut jj: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mid: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ii: libc::c_int = 0;
    let mut c: libc::c_char = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lo: libc::c_int = 0;
    let mut hi: libc::c_int = 0;
    lo = max.offset_from(base) as libc::c_long as libc::c_int;
    loop {
        i = base
            .offset(
                (qsz as libc::c_uint)
                    .wrapping_mul((lo / qsz) as libc::c_uint >> 1 as libc::c_int)
                    as isize,
            );
        mid = i;
        if lo >= mthresh {
            jj = base;
            j = if (Some(qcmp.expect("non-null function pointer")))
                .expect("non-null function pointer")(jj, i) > 0 as libc::c_int
            {
                jj
            } else {
                i
            };
            tmp = max.offset(-(qsz as isize));
            if (Some(qcmp.expect("non-null function pointer")))
                .expect("non-null function pointer")(j, tmp) > 0 as libc::c_int
            {
                j = if j == jj { i } else { jj };
                if (Some(qcmp.expect("non-null function pointer")))
                    .expect("non-null function pointer")(j, tmp) < 0 as libc::c_int
                {
                    j = tmp;
                }
            }
            if j != i {
                ii = qsz;
                loop {
                    c = *i;
                    let fresh2 = i;
                    i = i.offset(1);
                    *fresh2 = *j;
                    let fresh3 = j;
                    j = j.offset(1);
                    *fresh3 = c;
                    ii -= 1;
                    if !(ii != 0) {
                        break;
                    }
                }
            }
        }
        let mut current_block_39: u64;
        i = base;
        j = max.offset(-(qsz as isize));
        loop {
            while i < mid
                && (Some(qcmp.expect("non-null function pointer")))
                    .expect("non-null function pointer")(i, mid) <= 0 as libc::c_int
            {
                i = i.offset(qsz as isize);
            }
            loop {
                if !(j > mid) {
                    current_block_39 = 17788412896529399552;
                    break;
                }
                if (Some(qcmp.expect("non-null function pointer")))
                    .expect("non-null function pointer")(mid, j) <= 0 as libc::c_int
                {
                    j = j.offset(-(qsz as isize));
                } else {
                    tmp = i.offset(qsz as isize);
                    if i == mid {
                        jj = j;
                        mid = jj;
                    } else {
                        jj = j;
                        j = j.offset(-(qsz as isize));
                    }
                    current_block_39 = 156004533561089281;
                    break;
                }
            }
            match current_block_39 {
                17788412896529399552 => {
                    if i == mid {
                        break;
                    }
                    jj = mid;
                    mid = i;
                    tmp = mid;
                    j = j.offset(-(qsz as isize));
                }
                _ => {}
            }
            ii = qsz;
            loop {
                c = *i;
                let fresh4 = i;
                i = i.offset(1);
                *fresh4 = *jj;
                let fresh5 = jj;
                jj = jj.offset(1);
                *fresh5 = c;
                ii -= 1;
                if !(ii != 0) {
                    break;
                }
            }
            i = tmp;
        }
        j = mid;
        i = j.offset(qsz as isize);
        lo = j.offset_from(base) as libc::c_long as libc::c_int;
        hi = max.offset_from(i) as libc::c_long as libc::c_int;
        if lo <= hi {
            if lo >= thresh {
                qst(base, j);
            }
            base = i;
            lo = hi;
        } else {
            if hi >= thresh {
                qst(i, max);
            }
            max = j;
        }
        if !(lo >= thresh) {
            break;
        }
    };
}
