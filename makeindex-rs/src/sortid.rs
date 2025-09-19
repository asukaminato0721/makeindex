use libc::*;
use libc_stdhandle::*;
unsafe extern "C" {
    // pub type _IO_wide_data;
    // pub type _IO_codecvt;
    // pub type _IO_marker;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn tolower(_: i32) -> i32;
    static mut letter_ordering: i32;
    static mut verbose: i32;
    static mut german_sort: i32;
    static mut idx_ropen: libc::c_char;
    static mut idx_rclose: libc::c_char;
    static mut ilg_fp: *mut FILE;
    static mut idx_key: *mut FIELD_PTR;
    static mut idx_dot: i32;
    static mut idx_gt: i32;
    static mut idx_dc: i32;
    fn group_type(str: *mut libc::c_char) -> i32;
    fn qqsort(
        base: *mut libc::c_char,
        n: i32,
        size: i32,
        compar: Option<unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_char) -> i32>,
    );
}
type size_t = libc::c_ulong;
type __off_t = libc::c_long;
type __off64_t = libc::c_long;
type _IO_lock_t = ();
const _ISalnum: u32 = 8;
const _ISpunct: u32 = 4;
const _IScntrl: u32 = 2;
const _ISblank: u32 = 1;
const _ISgraph: u32 = 32768;
const _ISprint: u32 = 16384;
const _ISspace: u32 = 8192;
const _ISxdigit: u32 = 4096;
const _ISdigit: u32 = 2048;
const _ISalpha: u32 = 1024;
const _ISlower: u32 = 512;
const _ISupper: u32 = 256;
#[derive(Copy, Clone)]
#[repr(C)]
struct KFIELD {
    pub sf: [*mut libc::c_char; 3],
    pub af: [*mut libc::c_char; 3],
    pub group: i32,
    pub lpg: [libc::c_char; 16],
    pub npg: [i16; 10],
    pub count: i16,
    pub type_0: i16,
    pub encap: *mut libc::c_char,
    pub fn_0: *mut libc::c_char,
    pub lc: i32,
}
type FIELD_PTR = *mut KFIELD;
static mut idx_gc: libc::c_long = 0;
#[no_mangle]
pub unsafe extern "C" fn sort_idx() {
    if verbose != 0 {
        fprintf(
            stderr(),
            b"Sorting entries...\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    fprintf(
        ilg_fp,
        b"Sorting entries...\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    idx_dc = 0;
    idx_gc = 0;
    qqsort(
        idx_key as *mut libc::c_char,
        idx_gt,
        ::core::mem::size_of::<FIELD_PTR>() as libc::c_ulong as i32,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut *mut KFIELD, *mut *mut KFIELD) -> i32>,
            Option<unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_char) -> i32>,
        >(Some(
            compare as unsafe extern "C" fn(*mut *mut KFIELD, *mut *mut KFIELD) -> i32,
        )),
    );
    if verbose != 0 {
        fprintf(
            stderr(),
            b"done (%ld comparisons).\n\0" as *const u8 as *const libc::c_char,
            idx_gc,
        );
    }
    fprintf(
        ilg_fp,
        b"done (%ld comparisons).\n\0" as *const u8 as *const libc::c_char,
        idx_gc,
    );
}
unsafe extern "C" fn compare(a: *mut FIELD_PTR, b: *mut FIELD_PTR) -> i32 {
    let mut i = 0;
    let mut dif = 0;
    idx_gc += 1;
    idx_dot = 1;
    let fresh0 = idx_dc;
    idx_dc += 1;
    if fresh0 == 0 {
        if verbose != 0 {
            fprintf(stderr(), b".\0" as *const u8 as *const libc::c_char);
        }
        fprintf(ilg_fp, b".\0" as *const u8 as *const libc::c_char);
    }
    if idx_dc == 1500 {
        idx_dc = 0;
    }
    i = 0;
    while i < 3 {
        dif = compare_one((**a).sf[i as usize], (**b).sf[i as usize]);
        if dif != 0 {
            break;
        }
        dif = compare_one((**a).af[i as usize], (**b).af[i as usize]);
        if dif != 0 {
            break;
        }
        i += 1;
    }
    if i == 3 {
        dif = compare_page(a, b);
    }
    dif
}
unsafe extern "C" fn compare_one(x: *mut libc::c_char, y: *mut libc::c_char) -> i32 {
    let mut m = 0;
    let mut n = 0;
    if *x.offset(0) as i32 == '\0' as i32 && *y.offset(0) as i32 == '\0' as i32 {
        return 0;
    }
    if *x.offset(0) as i32 == '\0' as i32 {
        return -1;
    }
    if *y.offset(0) as i32 == '\0' as i32 {
        return 1;
    }
    m = group_type(x);
    n = group_type(y);
    if m >= 0 && n >= 0 {
        return m - n;
    }
    if m >= 0 {
        if german_sort != 0 {
            return 1;
        } else {
            return if n == -1 { 1 } else { -1 };
        }
    }
    if n >= 0 {
        if german_sort != 0 {
            return -1;
        } else {
            return if m == -1 { -1 } else { 1 };
        }
    }
    if m == -1 && n == -1 {
        return check_mixsym(x, y);
    }
    if m == -1 {
        return -1;
    }
    if n == -1 {
        return 1;
    }
    compare_string(x as *mut libc::c_uchar, y as *mut libc::c_uchar)
}
unsafe extern "C" fn check_mixsym(x: *mut libc::c_char, y: *mut libc::c_char) -> i32 {
    let mut m = false;
    let mut n = false;
    m = '0' as i32 <= *x.offset(0) as i32 && *x.offset(0) as i32 <= '9' as i32;
    n = '0' as i32 <= *y.offset(0) as i32 && *y.offset(0) as i32 <= '9' as i32;
    if m && !n {
        return 1;
    }
    if !m && n {
        return -1;
    }
    strcmp(x, y)
}
unsafe extern "C" fn compare_string(a: *mut libc::c_uchar, b: *mut libc::c_uchar) -> i32 {
    let mut i = 0;
    let mut j = 0;
    let mut al = 0;
    let mut bl = 0;
    while *a.offset(i) as i32 != '\0' as i32 || *b.offset(j) as i32 != '\0' as i32 {
        if *a.offset(i) as i32 == '\0' as i32 {
            return -1;
        }
        if *b.offset(j) as i32 == '\0' as i32 {
            return 1;
        }
        if letter_ordering != 0 {
            if *a.offset(i) as i32 == ' ' as i32 {
                i += 1;
            }
            if *b.offset(j) as i32 == ' ' as i32 {
                j += 1;
            }
        }
        al = if *(*__ctype_b_loc()).offset(*a.offset(i) as i32 as isize) as i32
            & _ISupper as i32 as libc::c_ushort as i32
            != 0
        {
            tolower(*a.offset(i) as i32) as libc::c_uchar as i32
        } else {
            *a.offset(i) as i32
        };
        bl = if *(*__ctype_b_loc()).offset(*b.offset(j) as i32 as isize) as i32
            & _ISupper as i32 as libc::c_ushort as i32
            != 0
        {
            tolower(*b.offset(j) as i32) as libc::c_uchar as i32
        } else {
            *b.offset(j) as i32
        };
        if al != bl {
            return al - bl;
        }
        i += 1;

        j += 1;
    }
    if german_sort != 0 {
        new_strcmp(a, b, 0)
    } else {
        strcmp(a as *mut libc::c_char, b as *mut libc::c_char)
    }
}
unsafe extern "C" fn compare_page(a: *mut FIELD_PTR, b: *mut FIELD_PTR) -> i32 {
    let mut m = 0;
    let mut i = 0_i16;
    while (i as i32) < (**a).count as i32 && (i as i32) < (**b).count as i32 && {
        m = (**a).npg[i as usize] as i32 - (**b).npg[i as usize] as i32;
        m == 0
    } {
        i += 1;
    }
    if m == 0 {
        if i as i32 == (**a).count as i32 && i as i32 == (**b).count as i32 {
            if (*(**a).encap as i32 == idx_ropen as i32 || *(**a).encap as i32 == idx_rclose as i32)
                && (*(**b).encap as i32 == idx_ropen as i32
                    || *(**b).encap as i32 == idx_rclose as i32)
            {
                m = (**a).lc - (**b).lc;
            } else if strcmp((**a).encap, (**b).encap) == 0 {
                if (**a).type_0 as i32 != 9999 && (**b).type_0 as i32 != 9999 {
                    (**b).type_0 = 9999;
                }
            } else if *(**a).encap as i32 == idx_ropen as i32
                || *(**a).encap as i32 == idx_rclose as i32
                || (*(**b).encap as i32 == idx_ropen as i32
                    || *(**b).encap as i32 == idx_rclose as i32)
            {
                m = (**a).lc - (**b).lc;
            } else {
                m = compare_string(
                    (**a).encap as *mut libc::c_uchar,
                    (**b).encap as *mut libc::c_uchar,
                );
            }
        } else if i as i32 == (**a).count as i32 && (i as i32) < (**b).count as i32 {
            m = -1;
        } else if (i as i32) < (**a).count as i32 && i as i32 == (**b).count as i32 {
            m = 1;
        }
    }
    m
}
unsafe extern "C" fn new_strcmp(
    s1: *mut libc::c_uchar,
    s2: *mut libc::c_uchar,
    option: i32,
) -> i32 {
    let mut i = 0;
    i = 0;
    while *s1.offset(i) as i32 == *s2.offset(i) as i32 {
        let fresh1 = i;
        i += 1;
        if *s1.offset(fresh1) as i32 == '\0' as i32 {
            return 0;
        }
    }
    if option != 0 {
        if *(*__ctype_b_loc()).offset(*s1.offset(i) as i32 as isize) as i32
            & _ISupper as i32 as libc::c_ushort as i32
            != 0
        {
            -1
        } else {
            1
        }
    } else if *(*__ctype_b_loc()).offset(*s1.offset(i) as i32 as isize) as i32
        & _ISupper as i32 as libc::c_ushort as i32
        != 0
    {
        1
    } else {
        -1
    }
}
