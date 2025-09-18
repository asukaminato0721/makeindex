unsafe extern "C" {
    // pub type _IO_wide_data;
    // pub type _IO_codecvt;
    // pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> i32;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn tolower(_: i32) -> i32;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> i32;
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
#[derive(Copy, Clone)]
#[repr(C)]
struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    // pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    // pub _codecvt: *mut _IO_codecvt,
    // pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [libc::c_char; 20],
}
type _IO_lock_t = ();
type FILE = _IO_FILE;
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
    pub npg: [libc::c_short; 10],
    pub count: libc::c_short,
    pub type_0: libc::c_short,
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
            stderr,
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
            stderr,
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
unsafe extern "C" fn compare(mut a: *mut FIELD_PTR, mut b: *mut FIELD_PTR) -> i32 {
    let mut i: i32 = 0;
    let mut dif: i32 = 0;
    idx_gc += 1;
    idx_dot = 1;
    let fresh0 = idx_dc;
    idx_dc = idx_dc + 1;
    if fresh0 == 0 as i32 {
        if verbose != 0 {
            fprintf(stderr, b".\0" as *const u8 as *const libc::c_char);
        }
        fprintf(ilg_fp, b".\0" as *const u8 as *const libc::c_char);
    }
    if idx_dc == 1500 as i32 {
        idx_dc = 0;
    }
    i = 0;
    while i < 3 as i32 {
        dif = compare_one((**a).sf[i as usize], (**b).sf[i as usize]);
        if dif != 0 as i32 {
            break;
        }
        dif = compare_one((**a).af[i as usize], (**b).af[i as usize]);
        if dif != 0 as i32 {
            break;
        }
        i += 1;
    }
    if i == 3 as i32 {
        dif = compare_page(a, b);
    }
    return dif;
}
unsafe extern "C" fn compare_one(mut x: *mut libc::c_char, mut y: *mut libc::c_char) -> i32 {
    let mut m = 0;
    let mut n = 0;
    if *x.offset(0) as i32 == '\0' as i32 && *y.offset(0) as i32 == '\0' as i32 {
        return 0 as i32;
    }
    if *x.offset(0) as i32 == '\0' as i32 {
        return -(1 as i32);
    }
    if *y.offset(0) as i32 == '\0' as i32 {
        return 1 as i32;
    }
    m = group_type(x);
    n = group_type(y);
    if m >= 0 as i32 && n >= 0 as i32 {
        return m - n;
    }
    if m >= 0 as i32 {
        if german_sort != 0 {
            return 1 as i32;
        } else {
            return if n == -(1 as i32) {
                1 as i32
            } else {
                -(1 as i32)
            };
        }
    }
    if n >= 0 as i32 {
        if german_sort != 0 {
            return -(1 as i32);
        } else {
            return if m == -(1 as i32) {
                -(1 as i32)
            } else {
                1 as i32
            };
        }
    }
    if m == -(1 as i32) && n == -(1 as i32) {
        return check_mixsym(x, y);
    }
    if m == -(1 as i32) {
        return -(1 as i32);
    }
    if n == -(1 as i32) {
        return 1 as i32;
    }
    return compare_string(x as *mut libc::c_uchar, y as *mut libc::c_uchar);
}
unsafe extern "C" fn check_mixsym(mut x: *mut libc::c_char, mut y: *mut libc::c_char) -> i32 {
    let mut m = false;
    let mut n = false;
    m = ('0' as i32 <= *x.offset(0) as i32 && *x.offset(0) as i32 <= '9' as i32);
    n = ('0' as i32 <= *y.offset(0) as i32 && *y.offset(0) as i32 <= '9' as i32);
    if m != false && n == false {
        return 1 as i32;
    }
    if m == false && n != false {
        return -(1 as i32);
    }
    return strcmp(x, y);
}
unsafe extern "C" fn compare_string(mut a: *mut libc::c_uchar, mut b: *mut libc::c_uchar) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut j: i32 = 0 as i32;
    let mut al: i32 = 0;
    let mut bl: i32 = 0;
    while *a.offset(i as isize) as i32 != '\0' as i32 || *b.offset(j as isize) as i32 != '\0' as i32
    {
        if *a.offset(i as isize) as i32 == '\0' as i32 {
            return -(1 as i32);
        }
        if *b.offset(j as isize) as i32 == '\0' as i32 {
            return 1 as i32;
        }
        if letter_ordering != 0 {
            if *a.offset(i as isize) as i32 == ' ' as i32 {
                i += 1;
            }
            if *b.offset(j as isize) as i32 == ' ' as i32 {
                j += 1;
            }
        }
        al = if *(*__ctype_b_loc()).offset(*a.offset(i as isize) as i32 as isize) as i32
            & _ISupper as i32 as libc::c_ushort as i32
            != 0
        {
            tolower(*a.offset(i as isize) as i32) as libc::c_uchar as i32
        } else {
            *a.offset(i as isize) as i32
        };
        bl = if *(*__ctype_b_loc()).offset(*b.offset(j as isize) as i32 as isize) as i32
            & _ISupper as i32 as libc::c_ushort as i32
            != 0
        {
            tolower(*b.offset(j as isize) as i32) as libc::c_uchar as i32
        } else {
            *b.offset(j as isize) as i32
        };
        if al != bl {
            return al - bl;
        }
        i += 1;

        j += 1;
    }
    if german_sort != 0 {
        return new_strcmp(a, b, 0 as i32);
    } else {
        return strcmp(a as *mut libc::c_char, b as *mut libc::c_char);
    };
}
unsafe extern "C" fn compare_page(mut a: *mut FIELD_PTR, mut b: *mut FIELD_PTR) -> i32 {
    let mut m: i32 = 0 as i32;
    let mut i: libc::c_short = 0 as i32 as libc::c_short;
    while (i as i32) < (**a).count as i32 && (i as i32) < (**b).count as i32 && {
        m = (**a).npg[i as usize] as i32 - (**b).npg[i as usize] as i32;
        m == 0 as i32
    } {
        i += 1;
    }
    if m == 0 as i32 {
        if i as i32 == (**a).count as i32 && i as i32 == (**b).count as i32 {
            if (*(**a).encap as i32 == idx_ropen as i32 || *(**a).encap as i32 == idx_rclose as i32)
                && (*(**b).encap as i32 == idx_ropen as i32
                    || *(**b).encap as i32 == idx_rclose as i32)
            {
                m = (**a).lc - (**b).lc;
            } else if strcmp((**a).encap, (**b).encap) == 0 as i32 {
                if (**a).type_0 as i32 != 9999 as i32 && (**b).type_0 as i32 != 9999 as i32 {
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
            m = -(1 as i32);
        } else if (i as i32) < (**a).count as i32 && i as i32 == (**b).count as i32 {
            m = 1;
        }
    }
    return m;
}
unsafe extern "C" fn new_strcmp(
    mut s1: *mut libc::c_uchar,
    mut s2: *mut libc::c_uchar,
    mut option: i32,
) -> i32 {
    let mut i: i32 = 0;
    i = 0;
    while *s1.offset(i as isize) as i32 == *s2.offset(i as isize) as i32 {
        let fresh1 = i;
        i = i + 1;
        if *s1.offset(fresh1 as isize) as i32 == '\0' as i32 {
            return 0 as i32;
        }
    }
    if option != 0 {
        return if *(*__ctype_b_loc()).offset(*s1.offset(i as isize) as i32 as isize) as i32
            & _ISupper as i32 as libc::c_ushort as i32
            != 0
        {
            -(1 as i32)
        } else {
            1 as i32
        };
    } else {
        return if *(*__ctype_b_loc()).offset(*s1.offset(i as isize) as i32 as isize) as i32
            & _ISupper as i32 as libc::c_ushort as i32
            != 0
        {
            1 as i32
        } else {
            -(1 as i32)
        };
    };
}
