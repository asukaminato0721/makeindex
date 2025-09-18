unsafe extern "C" {
   // pub type _IO_wide_data;
   // pub type _IO_codecvt;
   // pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn tolower(_: libc::c_int) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut letter_ordering: libc::c_int;
    static mut verbose: libc::c_int;
    static mut german_sort: libc::c_int;
    static mut idx_ropen: libc::c_char;
    static mut idx_rclose: libc::c_char;
    static mut ilg_fp: *mut FILE;
    static mut idx_key: *mut FIELD_PTR;
    static mut idx_dot: libc::c_int;
    static mut idx_gt: libc::c_int;
    static mut idx_dc: libc::c_int;
    fn group_type(str: *mut libc::c_char) -> libc::c_int;
    fn qqsort(
        base: *mut libc::c_char,
        n: libc::c_int,
        size: libc::c_int,
        compar: Option::<
            unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_char) -> libc::c_int,
        >,
    );
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
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
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
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
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct KFIELD {
    pub sf: [*mut libc::c_char; 3],
    pub af: [*mut libc::c_char; 3],
    pub group: libc::c_int,
    pub lpg: [libc::c_char; 16],
    pub npg: [libc::c_short; 10],
    pub count: libc::c_short,
    pub type_0: libc::c_short,
    pub encap: *mut libc::c_char,
    pub fn_0: *mut libc::c_char,
    pub lc: libc::c_int,
}
pub type FIELD_PTR = *mut KFIELD;
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
    idx_dc = 0 as libc::c_int;
    idx_gc = 0 as libc::c_long;
    qqsort(
        idx_key as *mut libc::c_char,
        idx_gt,
        ::core::mem::size_of::<FIELD_PTR>() as libc::c_ulong as libc::c_int,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut *mut KFIELD, *mut *mut KFIELD) -> libc::c_int,
            >,
            Option::<
                unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_char) -> libc::c_int,
            >,
        >(
            Some(
                compare
                    as unsafe extern "C" fn(
                        *mut *mut KFIELD,
                        *mut *mut KFIELD,
                    ) -> libc::c_int,
            ),
        ),
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
unsafe extern "C" fn compare(
    mut a: *mut FIELD_PTR,
    mut b: *mut FIELD_PTR,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut dif: libc::c_int = 0;
    idx_gc += 1;
    idx_gc;
    idx_dot = 1 as libc::c_int;
    let fresh0 = idx_dc;
    idx_dc = idx_dc + 1;
    if fresh0 == 0 as libc::c_int {
        if verbose != 0 {
            fprintf(stderr, b".\0" as *const u8 as *const libc::c_char);
        }
        fprintf(ilg_fp, b".\0" as *const u8 as *const libc::c_char);
    }
    if idx_dc == 1500 as libc::c_int {
        idx_dc = 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        dif = compare_one((**a).sf[i as usize], (**b).sf[i as usize]);
        if dif != 0 as libc::c_int {
            break;
        }
        dif = compare_one((**a).af[i as usize], (**b).af[i as usize]);
        if dif != 0 as libc::c_int {
            break;
        }
        i += 1;
        i;
    }
    if i == 3 as libc::c_int {
        dif = compare_page(a, b);
    }
    return dif;
}
unsafe extern "C" fn compare_one(
    mut x: *mut libc::c_char,
    mut y: *mut libc::c_char,
) -> libc::c_int {
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    if *x.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
        && *y.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        return 0 as libc::c_int;
    }
    if *x.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if *y.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
        return 1 as libc::c_int;
    }
    m = group_type(x);
    n = group_type(y);
    if m >= 0 as libc::c_int && n >= 0 as libc::c_int {
        return m - n;
    }
    if m >= 0 as libc::c_int {
        if german_sort != 0 {
            return 1 as libc::c_int
        } else {
            return if n == -(1 as libc::c_int) {
                1 as libc::c_int
            } else {
                -(1 as libc::c_int)
            }
        }
    }
    if n >= 0 as libc::c_int {
        if german_sort != 0 {
            return -(1 as libc::c_int)
        } else {
            return if m == -(1 as libc::c_int) {
                -(1 as libc::c_int)
            } else {
                1 as libc::c_int
            }
        }
    }
    if m == -(1 as libc::c_int) && n == -(1 as libc::c_int) {
        return check_mixsym(x, y);
    }
    if m == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if n == -(1 as libc::c_int) {
        return 1 as libc::c_int;
    }
    return compare_string(x as *mut libc::c_uchar, y as *mut libc::c_uchar);
}
unsafe extern "C" fn check_mixsym(
    mut x: *mut libc::c_char,
    mut y: *mut libc::c_char,
) -> libc::c_int {
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    m = ('0' as i32 <= *x.offset(0 as libc::c_int as isize) as libc::c_int
        && *x.offset(0 as libc::c_int as isize) as libc::c_int <= '9' as i32)
        as libc::c_int;
    n = ('0' as i32 <= *y.offset(0 as libc::c_int as isize) as libc::c_int
        && *y.offset(0 as libc::c_int as isize) as libc::c_int <= '9' as i32)
        as libc::c_int;
    if m != 0 && n == 0 {
        return 1 as libc::c_int;
    }
    if m == 0 && n != 0 {
        return -(1 as libc::c_int);
    }
    return strcmp(x, y);
}
unsafe extern "C" fn compare_string(
    mut a: *mut libc::c_uchar,
    mut b: *mut libc::c_uchar,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    let mut al: libc::c_int = 0;
    let mut bl: libc::c_int = 0;
    while *a.offset(i as isize) as libc::c_int != '\0' as i32
        || *b.offset(j as isize) as libc::c_int != '\0' as i32
    {
        if *a.offset(i as isize) as libc::c_int == '\0' as i32 {
            return -(1 as libc::c_int);
        }
        if *b.offset(j as isize) as libc::c_int == '\0' as i32 {
            return 1 as libc::c_int;
        }
        if letter_ordering != 0 {
            if *a.offset(i as isize) as libc::c_int == ' ' as i32 {
                i += 1;
                i;
            }
            if *b.offset(j as isize) as libc::c_int == ' ' as i32 {
                j += 1;
                j;
            }
        }
        al = if *(*__ctype_b_loc()).offset(*a.offset(i as isize) as libc::c_int as isize)
            as libc::c_int & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            tolower(*a.offset(i as isize) as libc::c_int) as libc::c_uchar as libc::c_int
        } else {
            *a.offset(i as isize) as libc::c_int
        };
        bl = if *(*__ctype_b_loc()).offset(*b.offset(j as isize) as libc::c_int as isize)
            as libc::c_int & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            tolower(*b.offset(j as isize) as libc::c_int) as libc::c_uchar as libc::c_int
        } else {
            *b.offset(j as isize) as libc::c_int
        };
        if al != bl {
            return al - bl;
        }
        i += 1;
        i;
        j += 1;
        j;
    }
    if german_sort != 0 {
        return new_strcmp(a, b, 0 as libc::c_int)
    } else {
        return strcmp(a as *mut libc::c_char, b as *mut libc::c_char)
    };
}
unsafe extern "C" fn compare_page(
    mut a: *mut FIELD_PTR,
    mut b: *mut FIELD_PTR,
) -> libc::c_int {
    let mut m: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_short = 0 as libc::c_int as libc::c_short;
    while (i as libc::c_int) < (**a).count as libc::c_int
        && (i as libc::c_int) < (**b).count as libc::c_int
        && {
            m = (**a).npg[i as usize] as libc::c_int
                - (**b).npg[i as usize] as libc::c_int;
            m == 0 as libc::c_int
        }
    {
        i += 1;
        i;
    }
    if m == 0 as libc::c_int {
        if i as libc::c_int == (**a).count as libc::c_int
            && i as libc::c_int == (**b).count as libc::c_int
        {
            if (*(**a).encap as libc::c_int == idx_ropen as libc::c_int
                || *(**a).encap as libc::c_int == idx_rclose as libc::c_int)
                && (*(**b).encap as libc::c_int == idx_ropen as libc::c_int
                    || *(**b).encap as libc::c_int == idx_rclose as libc::c_int)
            {
                m = (**a).lc - (**b).lc;
            } else if strcmp((**a).encap, (**b).encap) == 0 as libc::c_int {
                if (**a).type_0 as libc::c_int != 9999 as libc::c_int
                    && (**b).type_0 as libc::c_int != 9999 as libc::c_int
                {
                    (**b).type_0 = 9999 as libc::c_int as libc::c_short;
                }
            } else if *(**a).encap as libc::c_int == idx_ropen as libc::c_int
                || *(**a).encap as libc::c_int == idx_rclose as libc::c_int
                || (*(**b).encap as libc::c_int == idx_ropen as libc::c_int
                    || *(**b).encap as libc::c_int == idx_rclose as libc::c_int)
            {
                m = (**a).lc - (**b).lc;
            } else {
                m = compare_string(
                    (**a).encap as *mut libc::c_uchar,
                    (**b).encap as *mut libc::c_uchar,
                );
            }
        } else if i as libc::c_int == (**a).count as libc::c_int
            && (i as libc::c_int) < (**b).count as libc::c_int
        {
            m = -(1 as libc::c_int);
        } else if (i as libc::c_int) < (**a).count as libc::c_int
            && i as libc::c_int == (**b).count as libc::c_int
        {
            m = 1 as libc::c_int;
        }
    }
    return m;
}
unsafe extern "C" fn new_strcmp(
    mut s1: *mut libc::c_uchar,
    mut s2: *mut libc::c_uchar,
    mut option: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while *s1.offset(i as isize) as libc::c_int == *s2.offset(i as isize) as libc::c_int
    {
        let fresh1 = i;
        i = i + 1;
        if *s1.offset(fresh1 as isize) as libc::c_int == '\0' as i32 {
            return 0 as libc::c_int;
        }
    }
    if option != 0 {
        return if *(*__ctype_b_loc())
            .offset(*s1.offset(i as isize) as libc::c_int as isize) as libc::c_int
            & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            -(1 as libc::c_int)
        } else {
            1 as libc::c_int
        }
    } else {
        return if *(*__ctype_b_loc())
            .offset(*s1.offset(i as isize) as libc::c_int as isize) as libc::c_int
            & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }
    };
}
