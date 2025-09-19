static mut qsz: isize = 0;
static mut thresh: isize = 0;
static mut mthresh: isize = 0;
static mut qcmp: Option<unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_char) -> i32> = None;
#[no_mangle]
pub unsafe extern "C" fn qqsort(
    base: *mut libc::c_char,
    n: isize,
    size: isize,
    compar: Option<unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_char) -> i32>,
) {
    let mut i;
    let mut j;
    let mut lo;
    let mut hi;
    let mut min;
    let mut c;
    
    if n <= 1 {
        return;
    }
    qsz = size;
    qcmp = compar;
    thresh = qsz * 4;
    mthresh = qsz * 6;
    let max = base.offset(n * qsz);
    if n >= 4 {
        qst(base, max);
        hi = base.offset(thresh);
    } else {
        hi = max;
    }
    lo = base;
    j = lo;
    loop {
        lo = lo.offset(qsz);
        if lo >= hi {
            break;
        }
        if qcmp.expect("non-null function pointer")(j, lo) > 0 {
            j = lo;
        }
    }
    if j != base {
        i = base;
        hi = base.offset(qsz);
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
        min = min.offset(qsz);
        hi = min;
        if hi >= max {
            break;
        }
        loop {
            hi = hi.offset(-(qsz));
            if qcmp.expect("non-null function pointer")(hi, min) <= 0 {
                break;
            }
        }
        hi = hi.offset(qsz);
        if hi != min {
            lo = min.offset(qsz);
            loop {
                lo = lo.offset(-1);
                if lo < min {
                    break;
                }
                c = *lo;
                j = lo;
                i = j;
                loop {
                    j = j.offset(-(qsz));
                    if j < hi {
                        break;
                    }
                    *i = *j;
                    i = j;
                }
                *i = c;
            }
        }
    }
}
unsafe extern "C" fn qst(mut base: *mut libc::c_char, mut max: *mut libc::c_char) {
    let mut i;
    let mut j;
    let mut jj = std::ptr::null_mut::<libc::c_char>();
    let mut mid;
    let mut ii;
    let mut c;
    let mut tmp = std::ptr::null_mut::<libc::c_char>();
    let mut lo;
    let mut hi;
    lo = max.offset_from(base);
    loop {
        i = base.offset((qsz).wrapping_mul((lo / qsz) >> 1));
        mid = i;
        if lo >= mthresh {
            jj = base;
            j = if qcmp.expect("non-null function pointer")(jj, i) > 0 {
                jj
            } else {
                i
            };
            tmp = max.offset(-(qsz));
            if qcmp.expect("non-null function pointer")(j, tmp) > 0 {
                j = if j == jj { i } else { jj };
                if qcmp.expect("non-null function pointer")(j, tmp) < 0 {
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
                    if ii == 0 {
                        break;
                    }
                }
            }
        }
        let mut current_block_39: u64;
        i = base;
        j = max.offset(-(qsz));
        loop {
            while i < mid && qcmp.expect("non-null function pointer")(i, mid) <= 0 {
                i = i.offset(qsz);
            }
            loop {
                if j <= mid {
                    current_block_39 = 17788412896529399552;
                    break;
                }
                if qcmp.expect("non-null function pointer")(mid, j) <= 0 {
                    j = j.offset(-(qsz));
                } else {
                    tmp = i.offset(qsz);
                    if i == mid {
                        jj = j;
                        mid = jj;
                    } else {
                        jj = j;
                        j = j.offset(-(qsz));
                    }
                    current_block_39 = 156004533561089281;
                    break;
                }
            }
            if current_block_39 == 17788412896529399552 {
                if i == mid {
                    break;
                }
                jj = mid;
                mid = i;
                tmp = mid;
                j = j.offset(-(qsz));
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
                if ii == 0 {
                    break;
                }
            }
            i = tmp;
        }
        j = mid;
        i = j.offset(qsz);
        lo = j.offset_from(base);
        hi = max.offset_from(i);
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
        if lo < thresh {
            break;
        }
    }
}
