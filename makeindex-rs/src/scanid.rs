#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
   // pub type _IO_wide_data;
   // pub type _IO_codecvt;
   // pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut compress_blanks: libc::c_int;
    static mut verbose: libc::c_int;
    static mut german_sort: libc::c_int;
    static mut idx_keyword: [libc::c_char; 1024];
    static mut idx_aopen: libc::c_char;
    static mut idx_aclose: libc::c_char;
    static mut idx_level: libc::c_char;
    static mut idx_quote: libc::c_char;
    static mut idx_actual: libc::c_char;
    static mut idx_encap: libc::c_char;
    static mut idx_escape: libc::c_char;
    static mut page_comp: [libc::c_char; 1024];
    static mut page_offset: [libc::c_int; 5];
    static mut idx_fp: *mut FILE;
    static mut ilg_fp: *mut FILE;
    static mut idx_fn: *mut libc::c_char;
    static mut pgm_fn: *mut libc::c_char;
    static mut idx_dot: libc::c_int;
    static mut idx_tt: libc::c_int;
    static mut idx_et: libc::c_int;
    fn strtoint(str: *mut libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
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
  //  pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
  //  pub _codecvt: *mut _IO_codecvt,
  //  pub _wide_data: *mut _IO_wide_data,
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
pub type FIELD = KFIELD;
pub type FIELD_PTR = *mut KFIELD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct KNODE {
    pub data: FIELD,
    pub next: *mut KNODE,
}
pub type NODE = KNODE;
pub type NODE_PTR = *mut KNODE;
#[no_mangle]
pub static mut idx_lc: libc::c_int = 0;
#[no_mangle]
pub static mut idx_tc: libc::c_int = 0;
#[no_mangle]
pub static mut idx_ec: libc::c_int = 0;
#[no_mangle]
pub static mut idx_dc: libc::c_int = 0;
static mut first_entry: libc::c_int = 1 as libc::c_int;
static mut comp_len: libc::c_int = 0;
static mut key: [libc::c_char; 1024] = [0; 1024];
static mut no: [libc::c_char; 16] = [0; 16];
#[no_mangle]
pub static mut head: NODE_PTR = 0 as *const KNODE as *mut KNODE;
#[no_mangle]
pub static mut tail: NODE_PTR = 0 as *const KNODE as *mut KNODE;
#[no_mangle]
pub unsafe extern "C" fn scan_idx() {
    let mut keyword: [libc::c_char; 1024] = [0; 1024];
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut not_eof: libc::c_int = 1 as libc::c_int;
    let mut arg_count: libc::c_int = -(1 as libc::c_int);
    if verbose != 0 {
        fprintf(
            stderr,
            b"Scanning input file %s...\0" as *const u8 as *const libc::c_char,
            idx_fn,
        );
    }
    fprintf(
        ilg_fp,
        b"Scanning input file %s...\0" as *const u8 as *const libc::c_char,
        idx_fn,
    );
    idx_dc = 0 as libc::c_int;
    idx_ec = idx_dc;
    idx_tc = idx_ec;
    idx_lc = idx_tc;
    comp_len = strlen(page_comp.as_mut_ptr()) as libc::c_int;
    while not_eof != 0 {
        c = getc(idx_fp);
        match c {
            -1 => {
                if arg_count == 2 as libc::c_int {
                    idx_lc += 1;
                    idx_lc;
                    if make_key() != 0 {
                        idx_dot = 1 as libc::c_int;
                        let fresh0 = idx_dc;
                        idx_dc = idx_dc + 1;
                        if fresh0 == 0 as libc::c_int {
                            if verbose != 0 {
                                fprintf(stderr, b".\0" as *const u8 as *const libc::c_char);
                            }
                            fprintf(ilg_fp, b".\0" as *const u8 as *const libc::c_char);
                        }
                        if idx_dc == 1000 as libc::c_int {
                            idx_dc = 0 as libc::c_int;
                        }
                    }
                    arg_count = -(1 as libc::c_int);
                } else {
                    not_eof = 0 as libc::c_int;
                }
            }
            10 => {
                idx_lc += 1;
                idx_lc;
                if arg_count == 2 as libc::c_int {
                    if make_key() != 0 {
                        idx_dot = 1 as libc::c_int;
                        let fresh1 = idx_dc;
                        idx_dc = idx_dc + 1;
                        if fresh1 == 0 as libc::c_int {
                            if verbose != 0 {
                                fprintf(stderr, b".\0" as *const u8 as *const libc::c_char);
                            }
                            fprintf(ilg_fp, b".\0" as *const u8 as *const libc::c_char);
                        }
                        if idx_dc == 1000 as libc::c_int {
                            idx_dc = 0 as libc::c_int;
                        }
                    }
                    arg_count = -(1 as libc::c_int);
                } else if arg_count > -(1 as libc::c_int) {
                    if idx_dot != 0 {
                        fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                        idx_dot = 0 as libc::c_int;
                    }
                    fprintf(
                        ilg_fp,
                        b"!! Input index error (file = %s, line = %d):\n   -- \0"
                            as *const u8 as *const libc::c_char,
                        idx_fn,
                        idx_lc,
                    );
                    fprintf(
                        ilg_fp,
                        b"Missing arguments -- need two (premature LFD).\n\0"
                            as *const u8 as *const libc::c_char,
                        0 as *mut libc::c_void,
                    );
                    idx_ec += 1;
                    idx_ec;
                    arg_count = -(1 as libc::c_int);
                }
            }
            9 | 32 => {}
            _ => {
                match arg_count {
                    -1 => {
                        i = 0 as libc::c_int;
                        let fresh2 = i;
                        i = i + 1;
                        keyword[fresh2 as usize] = c as libc::c_char;
                        arg_count += 1;
                        arg_count;
                        idx_tc += 1;
                        idx_tc;
                    }
                    0 => {
                        if c == idx_aopen as libc::c_int {
                            arg_count += 1;
                            arg_count;
                            keyword[i as usize] = '\0' as i32 as libc::c_char;
                            if strcmp(keyword.as_mut_ptr(), idx_keyword.as_mut_ptr())
                                == 0 as libc::c_int
                            {
                                if scan_arg1() == 0 {
                                    arg_count = -(1 as libc::c_int);
                                }
                            } else {
                                let mut tmp: libc::c_int = 0;
                                loop {
                                    tmp = getc(idx_fp);
                                    if !(tmp != '\n' as i32) {
                                        break;
                                    }
                                    if tmp == -(1 as libc::c_int) {
                                        break;
                                    }
                                }
                                idx_lc += 1;
                                idx_lc;
                                arg_count = -(1 as libc::c_int);
                                if idx_dot != 0 {
                                    fprintf(
                                        ilg_fp,
                                        b"\n\0" as *const u8 as *const libc::c_char,
                                    );
                                    idx_dot = 0 as libc::c_int;
                                }
                                fprintf(
                                    ilg_fp,
                                    b"!! Input index error (file = %s, line = %d):\n   -- \0"
                                        as *const u8 as *const libc::c_char,
                                    idx_fn,
                                    idx_lc,
                                );
                                fprintf(
                                    ilg_fp,
                                    b"Unknown index keyword %s.\n\0" as *const u8
                                        as *const libc::c_char,
                                    keyword.as_mut_ptr(),
                                );
                                idx_ec += 1;
                                idx_ec;
                            }
                        } else if i < 1024 as libc::c_int {
                            let fresh3 = i;
                            i = i + 1;
                            keyword[fresh3 as usize] = c as libc::c_char;
                        } else {
                            let mut tmp_0: libc::c_int = 0;
                            loop {
                                tmp_0 = getc(idx_fp);
                                if !(tmp_0 != '\n' as i32) {
                                    break;
                                }
                                if tmp_0 == -(1 as libc::c_int) {
                                    break;
                                }
                            }
                            idx_lc += 1;
                            idx_lc;
                            arg_count = -(1 as libc::c_int);
                            if idx_dot != 0 {
                                fprintf(
                                    ilg_fp,
                                    b"\n\0" as *const u8 as *const libc::c_char,
                                );
                                idx_dot = 0 as libc::c_int;
                            }
                            fprintf(
                                ilg_fp,
                                b"!! Input index error (file = %s, line = %d):\n   -- \0"
                                    as *const u8 as *const libc::c_char,
                                idx_fn,
                                idx_lc,
                            );
                            fprintf(
                                ilg_fp,
                                b"Index keyword %s too long (max %d).\n\0" as *const u8
                                    as *const libc::c_char,
                                keyword.as_mut_ptr(),
                                1024 as libc::c_int,
                            );
                            idx_ec += 1;
                            idx_ec;
                        }
                    }
                    1 => {
                        if c == idx_aopen as libc::c_int {
                            arg_count += 1;
                            arg_count;
                            if scan_arg2() == 0 {
                                arg_count = -(1 as libc::c_int);
                            }
                        } else {
                            let mut tmp_1: libc::c_int = 0;
                            loop {
                                tmp_1 = getc(idx_fp);
                                if !(tmp_1 != '\n' as i32) {
                                    break;
                                }
                                if tmp_1 == -(1 as libc::c_int) {
                                    break;
                                }
                            }
                            idx_lc += 1;
                            idx_lc;
                            arg_count = -(1 as libc::c_int);
                            if idx_dot != 0 {
                                fprintf(
                                    ilg_fp,
                                    b"\n\0" as *const u8 as *const libc::c_char,
                                );
                                idx_dot = 0 as libc::c_int;
                            }
                            fprintf(
                                ilg_fp,
                                b"!! Input index error (file = %s, line = %d):\n   -- \0"
                                    as *const u8 as *const libc::c_char,
                                idx_fn,
                                idx_lc,
                            );
                            fprintf(
                                ilg_fp,
                                b"No opening delimiter for second argument (illegal character `%c').\n\0"
                                    as *const u8 as *const libc::c_char,
                                c,
                            );
                            idx_ec += 1;
                            idx_ec;
                        }
                    }
                    2 => {
                        let mut tmp_2: libc::c_int = 0;
                        loop {
                            tmp_2 = getc(idx_fp);
                            if !(tmp_2 != '\n' as i32) {
                                break;
                            }
                            if tmp_2 == -(1 as libc::c_int) {
                                break;
                            }
                        }
                        idx_lc += 1;
                        idx_lc;
                        arg_count = -(1 as libc::c_int);
                        if idx_dot != 0 {
                            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                            idx_dot = 0 as libc::c_int;
                        }
                        fprintf(
                            ilg_fp,
                            b"!! Input index error (file = %s, line = %d):\n   -- \0"
                                as *const u8 as *const libc::c_char,
                            idx_fn,
                            idx_lc,
                        );
                        fprintf(
                            ilg_fp,
                            b"No closing delimiter for second argument (illegal character `%c').\n\0"
                                as *const u8 as *const libc::c_char,
                            c,
                        );
                        idx_ec += 1;
                        idx_ec;
                    }
                    _ => {}
                }
            }
        }
    }
    idx_tt += idx_tc;
    idx_et += idx_ec;
    if verbose != 0 {
        fprintf(
            stderr,
            b"done (%d %s, %d %s).\n\0" as *const u8 as *const libc::c_char,
            idx_tc - idx_ec,
            b"entries accepted\0" as *const u8 as *const libc::c_char,
            idx_ec,
            b"rejected\0" as *const u8 as *const libc::c_char,
        );
    }
    fprintf(
        ilg_fp,
        b"done (%d %s, %d %s).\n\0" as *const u8 as *const libc::c_char,
        idx_tc - idx_ec,
        b"entries accepted\0" as *const u8 as *const libc::c_char,
        idx_ec,
        b"rejected\0" as *const u8 as *const libc::c_char,
    );
    fclose(idx_fp);
}
unsafe extern "C" fn flush_to_eol() {
    let mut a: libc::c_int = 0;
    loop {
        a = getc(idx_fp);
        if !(a != '\n' as i32 && a != -(1 as libc::c_int)) {
            break;
        }
    };
}
unsafe extern "C" fn make_key() -> libc::c_int {
    let mut ptr: NODE_PTR = 0 as *mut KNODE;
    let mut i: libc::c_int = 0;
    ptr = malloc(::core::mem::size_of::<NODE>() as libc::c_ulong) as NODE_PTR;
    if ptr.is_null() {
        fprintf(
            stderr,
            b"Not enough core...abort.\n\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"Usage: %s [-ilqrcg] [-s sty] [-o ind] [-t log] [-p num] [idx0 idx1 ...]\n\0"
                as *const u8 as *const libc::c_char,
            pgm_fn,
        );
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        (*ptr)
            .data
            .sf[i
            as usize] = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        (*ptr)
            .data
            .af[i
            as usize] = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        i += 1;
        i;
    }
    (*ptr).data.encap = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*ptr).data.lpg[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    (*ptr).data.count = 0 as libc::c_int as libc::c_short;
    (*ptr).data.type_0 = -(9999 as libc::c_int) as libc::c_short;
    if scan_key(&mut (*ptr).data) == 0 {
        return 0 as libc::c_int;
    }
    (*ptr).data.group = group_type((*ptr).data.sf[0 as libc::c_int as usize]);
    strcpy(((*ptr).data.lpg).as_mut_ptr(), no.as_mut_ptr());
    if scan_no(
        no.as_mut_ptr(),
        ((*ptr).data.npg).as_mut_ptr(),
        &mut (*ptr).data.count,
        &mut (*ptr).data.type_0,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if first_entry != 0 {
        tail = ptr;
        head = tail;
        first_entry = 0 as libc::c_int;
    } else {
        (*tail).next = ptr;
        tail = ptr;
    }
    (*ptr).data.lc = idx_lc;
    (*ptr).data.fn_0 = idx_fn;
    (*tail).next = 0 as *mut KNODE;
    return 1 as libc::c_int;
}
unsafe extern "C" fn make_string(mut ppstr: *mut *mut libc::c_char, mut n: libc::c_int) {
    if *(*ppstr).offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
        *ppstr = malloc(n as libc::c_ulong) as *mut libc::c_char;
        if (*ppstr).is_null() {
            fprintf(
                stderr,
                b"Not enough core...abort.\n\0" as *const u8 as *const libc::c_char,
                b"\0" as *const u8 as *const libc::c_char,
            );
            fprintf(
                stderr,
                b"Usage: %s [-ilqrcg] [-s sty] [-o ind] [-t log] [-p num] [idx0 idx1 ...]\n\0"
                    as *const u8 as *const libc::c_char,
                pgm_fn,
            );
            exit(1 as libc::c_int);
        }
        *(*ppstr).offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    }
}
unsafe extern "C" fn scan_key(mut data: FIELD_PTR) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut second_round: libc::c_int = 0 as libc::c_int;
    let mut last: libc::c_int = 3 as libc::c_int - 1 as libc::c_int;
    while !(key[n as usize] as libc::c_int == '\0' as i32) {
        if key[n as usize] as libc::c_int == idx_encap as libc::c_int {
            n += 1;
            n;
            make_string(
                &mut (*data).encap,
                (strlen(key.as_mut_ptr()))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            );
            if scan_field(
                &mut n,
                (*data).encap,
                strlen(key.as_mut_ptr()) as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            ) != 0
            {
                break;
            }
            return 0 as libc::c_int;
        } else if key[n as usize] as libc::c_int == idx_actual as libc::c_int {
            n += 1;
            n;
            if i == last {
                make_string(
                    &mut *((*data).af).as_mut_ptr().offset(i as isize),
                    (strlen(key.as_mut_ptr()))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                );
                if scan_field(
                    &mut n,
                    (*data).af[i as usize],
                    strlen(key.as_mut_ptr()) as libc::c_int,
                    0 as libc::c_int,
                    1 as libc::c_int,
                    0 as libc::c_int,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
            } else {
                make_string(
                    &mut *((*data).af).as_mut_ptr().offset(i as isize),
                    (strlen(key.as_mut_ptr()))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                );
                if scan_field(
                    &mut n,
                    (*data).af[i as usize],
                    strlen(key.as_mut_ptr()) as libc::c_int,
                    1 as libc::c_int,
                    1 as libc::c_int,
                    0 as libc::c_int,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
            }
        } else {
            if second_round != 0 {
                i += 1;
                i;
                n += 1;
                n;
            }
            if i == last {
                make_string(
                    &mut *((*data).sf).as_mut_ptr().offset(i as isize),
                    (strlen(key.as_mut_ptr()))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                );
                if scan_field(
                    &mut n,
                    (*data).sf[i as usize],
                    strlen(key.as_mut_ptr()) as libc::c_int,
                    0 as libc::c_int,
                    1 as libc::c_int,
                    1 as libc::c_int,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
            } else {
                make_string(
                    &mut *((*data).sf).as_mut_ptr().offset(i as isize),
                    (strlen(key.as_mut_ptr()))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                );
                if scan_field(
                    &mut n,
                    (*data).sf[i as usize],
                    strlen(key.as_mut_ptr()) as libc::c_int,
                    1 as libc::c_int,
                    1 as libc::c_int,
                    1 as libc::c_int,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
            }
            second_round = 1 as libc::c_int;
            if german_sort != 0
                && !(strchr((*data).sf[i as usize], '"' as i32)).is_null()
            {
                make_string(
                    &mut *((*data).af).as_mut_ptr().offset(i as isize),
                    (strlen((*data).sf[i as usize]))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                );
                search_quote((*data).sf[i as usize], (*data).af[i as usize]);
            }
        }
    }
    if *(*data).sf[0 as libc::c_int as usize] as libc::c_int == '\0' as i32 {
        if idx_dot != 0 {
            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
            idx_dot = 0 as libc::c_int;
        }
        fprintf(
            ilg_fp,
            b"!! Input index error (file = %s, line = %d):\n   -- \0" as *const u8
                as *const libc::c_char,
            idx_fn,
            idx_lc,
        );
        fprintf(
            ilg_fp,
            b"Illegal null field.\n\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        idx_ec += 1;
        idx_ec;
        return 0 as libc::c_int;
    }
    i = 1 as libc::c_int;
    while i < 3 as libc::c_int - 1 as libc::c_int {
        if *(*data).sf[i as usize] as libc::c_int == '\0' as i32
            && (*(*data).af[i as usize] as libc::c_int != '\0' as i32
                || *(*data).sf[(i + 1 as libc::c_int) as usize] as libc::c_int
                    != '\0' as i32)
        {
            if idx_dot != 0 {
                fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                idx_dot = 0 as libc::c_int;
            }
            fprintf(
                ilg_fp,
                b"!! Input index error (file = %s, line = %d):\n   -- \0" as *const u8
                    as *const libc::c_char,
                idx_fn,
                idx_lc,
            );
            fprintf(
                ilg_fp,
                b"Illegal null field.\n\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_void,
            );
            idx_ec += 1;
            idx_ec;
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    if *(*data).sf[i as usize] as libc::c_int == '\0' as i32
        && *(*data).af[i as usize] as libc::c_int != '\0' as i32
    {
        if idx_dot != 0 {
            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
            idx_dot = 0 as libc::c_int;
        }
        fprintf(
            ilg_fp,
            b"!! Input index error (file = %s, line = %d):\n   -- \0" as *const u8
                as *const libc::c_char,
            idx_fn,
            idx_lc,
        );
        fprintf(
            ilg_fp,
            b"Illegal null field.\n\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        idx_ec += 1;
        idx_ec;
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn scan_field(
    mut n: *mut libc::c_int,
    mut field: *mut libc::c_char,
    mut len_field: libc::c_int,
    mut ck_level: libc::c_int,
    mut ck_encap: libc::c_int,
    mut ck_actual: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut nbsh: libc::c_int = 0;
    if compress_blanks != 0
        && (key[*n as usize] as libc::c_int == ' ' as i32
            || key[*n as usize] as libc::c_int == '\t' as i32)
    {
        *n += 1;
        *n;
    }
    loop {
        nbsh = 0 as libc::c_int;
        loop {
            if !(key[*n as usize] as libc::c_int == idx_escape as libc::c_int) {
                current_block = 10886091980245723256;
                break;
            }
            nbsh += 1;
            nbsh;
            let fresh4 = i;
            i = i + 1;
            *field.offset(fresh4 as isize) = key[*n as usize];
            if i > len_field {
                current_block = 12901009814790378471;
                break;
            }
            *n += 1;
            *n;
        }
        match current_block {
            10886091980245723256 => {
                if key[*n as usize] as libc::c_int == idx_quote as libc::c_int {
                    if nbsh % 2 as libc::c_int == 0 as libc::c_int {
                        *n += 1;
                        let fresh5 = i;
                        i = i + 1;
                        *field.offset(fresh5 as isize) = key[*n as usize];
                    } else {
                        let fresh6 = i;
                        i = i + 1;
                        *field.offset(fresh6 as isize) = key[*n as usize];
                    }
                    if i > len_field {
                        current_block = 12901009814790378471;
                    } else {
                        current_block = 1622411330066726685;
                    }
                } else if ck_level != 0
                    && key[*n as usize] as libc::c_int == idx_level as libc::c_int
                    || ck_encap != 0
                        && key[*n as usize] as libc::c_int == idx_encap as libc::c_int
                    || ck_actual != 0
                        && key[*n as usize] as libc::c_int == idx_actual as libc::c_int
                    || key[*n as usize] as libc::c_int == '\0' as i32
                {
                    if i > 0 as libc::c_int && compress_blanks != 0
                        && *field.offset((i - 1 as libc::c_int) as isize) as libc::c_int
                            == ' ' as i32
                    {
                        *field
                            .offset(
                                (i - 1 as libc::c_int) as isize,
                            ) = '\0' as i32 as libc::c_char;
                    } else {
                        *field.offset(i as isize) = '\0' as i32 as libc::c_char;
                    }
                    return 1 as libc::c_int;
                } else {
                    let fresh7 = i;
                    i = i + 1;
                    *field.offset(fresh7 as isize) = key[*n as usize];
                    if i > len_field {
                        current_block = 12901009814790378471;
                    } else {
                        if ck_level == 0
                            && key[*n as usize] as libc::c_int
                                == idx_level as libc::c_int
                        {
                            if idx_dot != 0 {
                                fprintf(
                                    ilg_fp,
                                    b"\n\0" as *const u8 as *const libc::c_char,
                                );
                                idx_dot = 0 as libc::c_int;
                            }
                            fprintf(
                                ilg_fp,
                                b"!! Input index error (file = %s, line = %d):\n   -- \0"
                                    as *const u8 as *const libc::c_char,
                                idx_fn,
                                idx_lc,
                            );
                            fprintf(
                                ilg_fp,
                                b"Extra `%c' at position %d of first argument.\n\0"
                                    as *const u8 as *const libc::c_char,
                                idx_level as libc::c_int,
                                *n + 1 as libc::c_int,
                            );
                            idx_ec += 1;
                            idx_ec;
                            return 0 as libc::c_int;
                        } else if ck_encap == 0
                            && key[*n as usize] as libc::c_int
                                == idx_encap as libc::c_int
                        {
                            if idx_dot != 0 {
                                fprintf(
                                    ilg_fp,
                                    b"\n\0" as *const u8 as *const libc::c_char,
                                );
                                idx_dot = 0 as libc::c_int;
                            }
                            fprintf(
                                ilg_fp,
                                b"!! Input index error (file = %s, line = %d):\n   -- \0"
                                    as *const u8 as *const libc::c_char,
                                idx_fn,
                                idx_lc,
                            );
                            fprintf(
                                ilg_fp,
                                b"Extra `%c' at position %d of first argument.\n\0"
                                    as *const u8 as *const libc::c_char,
                                idx_encap as libc::c_int,
                                *n + 1 as libc::c_int,
                            );
                            idx_ec += 1;
                            idx_ec;
                            return 0 as libc::c_int;
                        } else if ck_actual == 0
                            && key[*n as usize] as libc::c_int
                                == idx_actual as libc::c_int
                        {
                            if idx_dot != 0 {
                                fprintf(
                                    ilg_fp,
                                    b"\n\0" as *const u8 as *const libc::c_char,
                                );
                                idx_dot = 0 as libc::c_int;
                            }
                            fprintf(
                                ilg_fp,
                                b"!! Input index error (file = %s, line = %d):\n   -- \0"
                                    as *const u8 as *const libc::c_char,
                                idx_fn,
                                idx_lc,
                            );
                            fprintf(
                                ilg_fp,
                                b"Extra `%c' at position %d of first argument.\n\0"
                                    as *const u8 as *const libc::c_char,
                                idx_actual as libc::c_int,
                                *n + 1 as libc::c_int,
                            );
                            idx_ec += 1;
                            idx_ec;
                            return 0 as libc::c_int;
                        }
                        current_block = 1622411330066726685;
                    }
                }
                match current_block {
                    12901009814790378471 => {}
                    _ => {
                        if !(i > len_field) {
                            *n += 1;
                            *n;
                            continue;
                        }
                    }
                }
            }
            _ => {}
        }
        if ck_encap == 0 {
            if idx_dot != 0 {
                fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                idx_dot = 0 as libc::c_int;
            }
            fprintf(
                ilg_fp,
                b"!! Input index error (file = %s, line = %d):\n   -- \0" as *const u8
                    as *const libc::c_char,
                idx_fn,
                idx_lc,
            );
            fprintf(
                ilg_fp,
                b"Encapsulator of page number too long (max. %d).\n\0" as *const u8
                    as *const libc::c_char,
                len_field,
            );
            idx_ec += 1;
            idx_ec;
        } else if ck_actual != 0 {
            if idx_dot != 0 {
                fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                idx_dot = 0 as libc::c_int;
            }
            fprintf(
                ilg_fp,
                b"!! Input index error (file = %s, line = %d):\n   -- \0" as *const u8
                    as *const libc::c_char,
                idx_fn,
                idx_lc,
            );
            fprintf(
                ilg_fp,
                b"Index sort key too long (max. %d).\n\0" as *const u8
                    as *const libc::c_char,
                len_field,
            );
            idx_ec += 1;
            idx_ec;
        } else {
            if idx_dot != 0 {
                fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                idx_dot = 0 as libc::c_int;
            }
            fprintf(
                ilg_fp,
                b"!! Input index error (file = %s, line = %d):\n   -- \0" as *const u8
                    as *const libc::c_char,
                idx_fn,
                idx_lc,
            );
            fprintf(
                ilg_fp,
                b"Text of key entry too long (max. %d).\n\0" as *const u8
                    as *const libc::c_char,
                len_field,
            );
            idx_ec += 1;
            idx_ec;
        }
        return 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn group_type(mut str: *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    while *str.offset(i as isize) as libc::c_int != '\0' as i32
        && ('0' as i32 <= *str.offset(i as isize) as libc::c_int
            && *str.offset(i as isize) as libc::c_int <= '9' as i32)
    {
        i += 1;
        i;
    }
    if *str.offset(i as isize) as libc::c_int == '\0' as i32 {
        sscanf(
            str,
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut i as *mut libc::c_int,
        );
        return i;
    } else if '!' as i32 <= *str.offset(0 as libc::c_int as isize) as libc::c_int
        && *str.offset(0 as libc::c_int as isize) as libc::c_int <= '@' as i32
        || '[' as i32 <= *str.offset(0 as libc::c_int as isize) as libc::c_int
            && *str.offset(0 as libc::c_int as isize) as libc::c_int <= '`' as i32
        || '{' as i32 <= *str.offset(0 as libc::c_int as isize) as libc::c_int
            && *str.offset(0 as libc::c_int as isize) as libc::c_int <= '~' as i32
    {
        return -(1 as libc::c_int)
    } else {
        return -(2 as libc::c_int)
    };
}
unsafe extern "C" fn scan_no(
    mut no_0: *mut libc::c_char,
    mut npg: *mut libc::c_short,
    mut count: *mut libc::c_short,
    mut type_0: *mut libc::c_short,
) -> libc::c_int {
    let mut i: libc::c_int = 1 as libc::c_int;
    if *(*__ctype_b_loc())
        .offset(*no_0.offset(0 as libc::c_int as isize) as libc::c_int as isize)
        as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        *type_0 = 2 as libc::c_int as libc::c_short;
        if scan_arabic(no_0, npg, count) == 0 {
            return 0 as libc::c_int;
        }
    } else if (*no_0.offset(0 as libc::c_int as isize) as libc::c_int == 'i' as i32
        || *no_0.offset(0 as libc::c_int as isize) as libc::c_int == 'v' as i32
        || *no_0.offset(0 as libc::c_int as isize) as libc::c_int == 'x' as i32
        || *no_0.offset(0 as libc::c_int as isize) as libc::c_int == 'l' as i32
        || *no_0.offset(0 as libc::c_int as isize) as libc::c_int == 'c' as i32
        || *no_0.offset(0 as libc::c_int as isize) as libc::c_int == 'd' as i32
        || *no_0.offset(0 as libc::c_int as isize) as libc::c_int == 'm' as i32)
        && !(strncmp(
            &mut *no_0.offset(i as isize),
            page_comp.as_mut_ptr(),
            comp_len as libc::c_ulong,
        ) == 0 as libc::c_int)
    {
        *type_0 = 0 as libc::c_int as libc::c_short;
        if scan_roman_lower(no_0, npg, count) == 0 {
            return 0 as libc::c_int;
        }
    } else if (*no_0.offset(0 as libc::c_int as isize) as libc::c_int == 'I' as i32
        || *no_0.offset(0 as libc::c_int as isize) as libc::c_int == 'V' as i32
        || *no_0.offset(0 as libc::c_int as isize) as libc::c_int == 'X' as i32
        || *no_0.offset(0 as libc::c_int as isize) as libc::c_int == 'L' as i32
        || *no_0.offset(0 as libc::c_int as isize) as libc::c_int == 'C' as i32
        || *no_0.offset(0 as libc::c_int as isize) as libc::c_int == 'D' as i32
        || *no_0.offset(0 as libc::c_int as isize) as libc::c_int == 'M' as i32)
        && (*no_0.offset(0 as libc::c_int as isize) as libc::c_int == 'I' as i32
            || !(strncmp(
                &mut *no_0.offset(i as isize),
                page_comp.as_mut_ptr(),
                comp_len as libc::c_ulong,
            ) == 0 as libc::c_int))
    {
        *type_0 = 1 as libc::c_int as libc::c_short;
        if scan_roman_upper(no_0, npg, count) == 0 {
            return 0 as libc::c_int;
        }
    } else if 'a' as i32 <= *no_0.offset(0 as libc::c_int as isize) as libc::c_int
        && *no_0.offset(0 as libc::c_int as isize) as libc::c_int <= 'z' as i32
    {
        *type_0 = 3 as libc::c_int as libc::c_short;
        if scan_alpha_lower(no_0, npg, count) == 0 {
            return 0 as libc::c_int;
        }
    } else if 'A' as i32 <= *no_0.offset(0 as libc::c_int as isize) as libc::c_int
        && *no_0.offset(0 as libc::c_int as isize) as libc::c_int <= 'Z' as i32
    {
        *type_0 = 4 as libc::c_int as libc::c_short;
        if scan_alpha_upper(no_0, npg, count) == 0 {
            return 0 as libc::c_int;
        }
    } else {
        if idx_dot != 0 {
            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
            idx_dot = 0 as libc::c_int;
        }
        fprintf(
            ilg_fp,
            b"!! Input index error (file = %s, line = %d):\n   -- \0" as *const u8
                as *const libc::c_char,
            idx_fn,
            idx_lc,
        );
        fprintf(
            ilg_fp,
            b"Illegal page number %s.\n\0" as *const u8 as *const libc::c_char,
            no_0,
        );
        idx_ec += 1;
        idx_ec;
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn scan_arabic(
    mut no_0: *mut libc::c_char,
    mut npg: *mut libc::c_short,
    mut count: *mut libc::c_short,
) -> libc::c_int {
    let mut i: libc::c_short = 0 as libc::c_int as libc::c_short;
    let mut str: [libc::c_char; 6] = [0; 6];
    while *no_0.offset(i as isize) as libc::c_int != '\0' as i32
        && i as libc::c_int <= 5 as libc::c_int
        && !(strncmp(
            &mut *no_0.offset(i as isize),
            page_comp.as_mut_ptr(),
            comp_len as libc::c_ulong,
        ) == 0 as libc::c_int)
    {
        if *(*__ctype_b_loc()).offset(*no_0.offset(i as isize) as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            str[i as usize] = *no_0.offset(i as isize);
            i += 1;
            i;
        } else {
            if idx_dot != 0 {
                fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                idx_dot = 0 as libc::c_int;
            }
            fprintf(
                ilg_fp,
                b"!! Input index error (file = %s, line = %d):\n   -- \0" as *const u8
                    as *const libc::c_char,
                idx_fn,
                idx_lc,
            );
            fprintf(
                ilg_fp,
                b"Illegal Arabic digit: position %d in %s.\n\0" as *const u8
                    as *const libc::c_char,
                i as libc::c_int + 1 as libc::c_int,
                no_0,
            );
            idx_ec += 1;
            idx_ec;
            return 0 as libc::c_int;
        }
    }
    if i as libc::c_int > 5 as libc::c_int {
        if idx_dot != 0 {
            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
            idx_dot = 0 as libc::c_int;
        }
        fprintf(
            ilg_fp,
            b"!! Input index error (file = %s, line = %d):\n   -- \0" as *const u8
                as *const libc::c_char,
            idx_fn,
            idx_lc,
        );
        fprintf(
            ilg_fp,
            b"Arabic page number %s too big (max %d digits).\n\0" as *const u8
                as *const libc::c_char,
            no_0,
            5 as libc::c_int,
        );
        idx_ec += 1;
        idx_ec;
        return 0 as libc::c_int;
    }
    str[i as usize] = '\0' as i32 as libc::c_char;
    if *count as libc::c_int >= 10 as libc::c_int {
        if idx_dot != 0 {
            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
            idx_dot = 0 as libc::c_int;
        }
        fprintf(
            ilg_fp,
            b"!! Input index error (file = %s, line = %d):\n   -- \0" as *const u8
                as *const libc::c_char,
            idx_fn,
            idx_lc,
        );
        fprintf(
            ilg_fp,
            b"Page number %s has too many fields (max. %d).\0" as *const u8
                as *const libc::c_char,
            no_0,
            10 as libc::c_int,
        );
        idx_ec += 1;
        idx_ec;
        return 0 as libc::c_int;
    }
    *npg
        .offset(
            *count as isize,
        ) = (strtoint(str.as_mut_ptr()) + page_offset[2 as libc::c_int as usize])
        as libc::c_short;
    *count += 1;
    *count;
    if strncmp(
        &mut *no_0.offset(i as isize),
        page_comp.as_mut_ptr(),
        comp_len as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        return scan_no(
            &mut *no_0.offset((i as libc::c_int + comp_len) as isize),
            npg,
            count,
            &mut i,
        )
    } else {
        return 1 as libc::c_int
    };
}
unsafe extern "C" fn scan_roman_lower(
    mut no_0: *mut libc::c_char,
    mut npg: *mut libc::c_short,
    mut count: *mut libc::c_short,
) -> libc::c_int {
    let mut i: libc::c_short = 0 as libc::c_int as libc::c_short;
    let mut inp: libc::c_int = 0 as libc::c_int;
    let mut prev: libc::c_int = 0 as libc::c_int;
    let mut the_new: libc::c_int = 0;
    while *no_0.offset(i as isize) as libc::c_int != '\0' as i32
        && (i as libc::c_int) < 16 as libc::c_int
        && !(strncmp(
            &mut *no_0.offset(i as isize),
            page_comp.as_mut_ptr(),
            comp_len as libc::c_ulong,
        ) == 0 as libc::c_int)
    {
        if (*no_0.offset(i as isize) as libc::c_int == 'i' as i32
            || *no_0.offset(i as isize) as libc::c_int == 'v' as i32
            || *no_0.offset(i as isize) as libc::c_int == 'x' as i32
            || *no_0.offset(i as isize) as libc::c_int == 'l' as i32
            || *no_0.offset(i as isize) as libc::c_int == 'c' as i32
            || *no_0.offset(i as isize) as libc::c_int == 'd' as i32
            || *no_0.offset(i as isize) as libc::c_int == 'm' as i32)
            && {
                the_new = (if *no_0.offset(i as isize) as libc::c_int == 'i' as i32 {
                    1 as libc::c_int
                } else {
                    (if *no_0.offset(i as isize) as libc::c_int == 'v' as i32 {
                        5 as libc::c_int
                    } else {
                        (if *no_0.offset(i as isize) as libc::c_int == 'x' as i32 {
                            10 as libc::c_int
                        } else {
                            (if *no_0.offset(i as isize) as libc::c_int == 'l' as i32 {
                                50 as libc::c_int
                            } else {
                                (if *no_0.offset(i as isize) as libc::c_int == 'c' as i32 {
                                    100 as libc::c_int
                                } else {
                                    (if *no_0.offset(i as isize) as libc::c_int == 'd' as i32 {
                                        500 as libc::c_int
                                    } else {
                                        (if *no_0.offset(i as isize) as libc::c_int == 'm' as i32 {
                                            1000 as libc::c_int
                                        } else {
                                            0 as libc::c_int
                                        })
                                    })
                                })
                            })
                        })
                    })
                });
                the_new != 0 as libc::c_int
            }
        {
            if prev == 0 as libc::c_int {
                prev = the_new;
            } else {
                if prev < the_new {
                    prev = the_new - prev;
                    the_new = 0 as libc::c_int;
                }
                inp += prev;
                prev = the_new;
            }
        } else {
            if idx_dot != 0 {
                fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                idx_dot = 0 as libc::c_int;
            }
            fprintf(
                ilg_fp,
                b"!! Input index error (file = %s, line = %d):\n   -- \0" as *const u8
                    as *const libc::c_char,
                idx_fn,
                idx_lc,
            );
            fprintf(
                ilg_fp,
                b"Illegal Roman number: position %d in %s.\n\0" as *const u8
                    as *const libc::c_char,
                i as libc::c_int + 1 as libc::c_int,
                no_0,
            );
            idx_ec += 1;
            idx_ec;
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    if i as libc::c_int == 16 as libc::c_int {
        if idx_dot != 0 {
            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
            idx_dot = 0 as libc::c_int;
        }
        fprintf(
            ilg_fp,
            b"!! Input index error (file = %s, line = %d):\n   -- \0" as *const u8
                as *const libc::c_char,
            idx_fn,
            idx_lc,
        );
        fprintf(
            ilg_fp,
            b"Roman page number %s too big (max %d digits).\n\0" as *const u8
                as *const libc::c_char,
            no_0,
            16 as libc::c_int,
        );
        idx_ec += 1;
        idx_ec;
        return 0 as libc::c_int;
    }
    inp += prev;
    if *count as libc::c_int >= 10 as libc::c_int {
        if idx_dot != 0 {
            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
            idx_dot = 0 as libc::c_int;
        }
        fprintf(
            ilg_fp,
            b"!! Input index error (file = %s, line = %d):\n   -- \0" as *const u8
                as *const libc::c_char,
            idx_fn,
            idx_lc,
        );
        fprintf(
            ilg_fp,
            b"Page number %s has too many fields (max. %d).\0" as *const u8
                as *const libc::c_char,
            no_0,
            10 as libc::c_int,
        );
        idx_ec += 1;
        idx_ec;
        return 0 as libc::c_int;
    }
    *npg
        .offset(
            *count as isize,
        ) = (inp + page_offset[0 as libc::c_int as usize]) as libc::c_short;
    *count += 1;
    *count;
    if strncmp(
        &mut *no_0.offset(i as isize),
        page_comp.as_mut_ptr(),
        comp_len as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        return scan_no(
            &mut *no_0.offset((i as libc::c_int + comp_len) as isize),
            npg,
            count,
            &mut i,
        )
    } else {
        return 1 as libc::c_int
    };
}
unsafe extern "C" fn scan_roman_upper(
    mut no_0: *mut libc::c_char,
    mut npg: *mut libc::c_short,
    mut count: *mut libc::c_short,
) -> libc::c_int {
    let mut i: libc::c_short = 0 as libc::c_int as libc::c_short;
    let mut inp: libc::c_int = 0 as libc::c_int;
    let mut prev: libc::c_int = 0 as libc::c_int;
    let mut the_new: libc::c_int = 0;
    while *no_0.offset(i as isize) as libc::c_int != '\0' as i32
        && (i as libc::c_int) < 16 as libc::c_int
        && !(strncmp(
            &mut *no_0.offset(i as isize),
            page_comp.as_mut_ptr(),
            comp_len as libc::c_ulong,
        ) == 0 as libc::c_int)
    {
        if (*no_0.offset(i as isize) as libc::c_int == 'I' as i32
            || *no_0.offset(i as isize) as libc::c_int == 'V' as i32
            || *no_0.offset(i as isize) as libc::c_int == 'X' as i32
            || *no_0.offset(i as isize) as libc::c_int == 'L' as i32
            || *no_0.offset(i as isize) as libc::c_int == 'C' as i32
            || *no_0.offset(i as isize) as libc::c_int == 'D' as i32
            || *no_0.offset(i as isize) as libc::c_int == 'M' as i32)
            && {
                the_new = (if *no_0.offset(i as isize) as libc::c_int == 'I' as i32 {
                    1 as libc::c_int
                } else {
                    (if *no_0.offset(i as isize) as libc::c_int == 'V' as i32 {
                        5 as libc::c_int
                    } else {
                        (if *no_0.offset(i as isize) as libc::c_int == 'X' as i32 {
                            10 as libc::c_int
                        } else {
                            (if *no_0.offset(i as isize) as libc::c_int == 'L' as i32 {
                                50 as libc::c_int
                            } else {
                                (if *no_0.offset(i as isize) as libc::c_int == 'C' as i32 {
                                    100 as libc::c_int
                                } else {
                                    (if *no_0.offset(i as isize) as libc::c_int == 'D' as i32 {
                                        500 as libc::c_int
                                    } else {
                                        (if *no_0.offset(i as isize) as libc::c_int == 'M' as i32 {
                                            1000 as libc::c_int
                                        } else {
                                            0 as libc::c_int
                                        })
                                    })
                                })
                            })
                        })
                    })
                });
                the_new != 0 as libc::c_int
            }
        {
            if prev == 0 as libc::c_int {
                prev = the_new;
            } else {
                if prev < the_new {
                    prev = the_new - prev;
                    the_new = 0 as libc::c_int;
                }
                inp += prev;
                prev = the_new;
            }
        } else {
            if idx_dot != 0 {
                fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                idx_dot = 0 as libc::c_int;
            }
            fprintf(
                ilg_fp,
                b"!! Input index error (file = %s, line = %d):\n   -- \0" as *const u8
                    as *const libc::c_char,
                idx_fn,
                idx_lc,
            );
            fprintf(
                ilg_fp,
                b"Illegal Roman number: position %d in %s.\n\0" as *const u8
                    as *const libc::c_char,
                i as libc::c_int + 1 as libc::c_int,
                no_0,
            );
            idx_ec += 1;
            idx_ec;
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    if i as libc::c_int == 16 as libc::c_int {
        if idx_dot != 0 {
            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
            idx_dot = 0 as libc::c_int;
        }
        fprintf(
            ilg_fp,
            b"!! Input index error (file = %s, line = %d):\n   -- \0" as *const u8
                as *const libc::c_char,
            idx_fn,
            idx_lc,
        );
        fprintf(
            ilg_fp,
            b"Roman page number %s too big (max %d digits).\n\0" as *const u8
                as *const libc::c_char,
            no_0,
            16 as libc::c_int,
        );
        idx_ec += 1;
        idx_ec;
        return 0 as libc::c_int;
    }
    inp += prev;
    if *count as libc::c_int >= 10 as libc::c_int {
        if idx_dot != 0 {
            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
            idx_dot = 0 as libc::c_int;
        }
        fprintf(
            ilg_fp,
            b"!! Input index error (file = %s, line = %d):\n   -- \0" as *const u8
                as *const libc::c_char,
            idx_fn,
            idx_lc,
        );
        fprintf(
            ilg_fp,
            b"Page number %s has too many fields (max. %d).\0" as *const u8
                as *const libc::c_char,
            no_0,
            10 as libc::c_int,
        );
        idx_ec += 1;
        idx_ec;
        return 0 as libc::c_int;
    }
    *npg
        .offset(
            *count as isize,
        ) = (inp + page_offset[1 as libc::c_int as usize]) as libc::c_short;
    *count += 1;
    *count;
    if strncmp(
        &mut *no_0.offset(i as isize),
        page_comp.as_mut_ptr(),
        comp_len as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        return scan_no(
            &mut *no_0.offset((i as libc::c_int + comp_len) as isize),
            npg,
            count,
            &mut i,
        )
    } else {
        return 1 as libc::c_int
    };
}
unsafe extern "C" fn scan_alpha_lower(
    mut no_0: *mut libc::c_char,
    mut npg: *mut libc::c_short,
    mut count: *mut libc::c_short,
) -> libc::c_int {
    let mut i: libc::c_short = 0;
    if *count as libc::c_int >= 10 as libc::c_int {
        if idx_dot != 0 {
            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
            idx_dot = 0 as libc::c_int;
        }
        fprintf(
            ilg_fp,
            b"!! Input index error (file = %s, line = %d):\n   -- \0" as *const u8
                as *const libc::c_char,
            idx_fn,
            idx_lc,
        );
        fprintf(
            ilg_fp,
            b"Page number %s has too many fields (max. %d).\0" as *const u8
                as *const libc::c_char,
            no_0,
            10 as libc::c_int,
        );
        idx_ec += 1;
        idx_ec;
        return 0 as libc::c_int;
    }
    *npg
        .offset(
            *count as isize,
        ) = ((if 'A' as i32 <= *no_0.offset(0 as libc::c_int as isize) as libc::c_int
        && *no_0.offset(0 as libc::c_int as isize) as libc::c_int <= 'Z' as i32
    {
        *no_0.offset(0 as libc::c_int as isize) as libc::c_int - 'A' as i32
    } else {
        (if 'a' as i32 <= *no_0.offset(0 as libc::c_int as isize) as libc::c_int
            && *no_0.offset(0 as libc::c_int as isize) as libc::c_int <= 'z' as i32
        {
            *no_0.offset(0 as libc::c_int as isize) as libc::c_int - 'a' as i32
        } else {
            0 as libc::c_int
        })
    }) + page_offset[3 as libc::c_int as usize]) as libc::c_short;
    *count += 1;
    *count;
    i = 1 as libc::c_int as libc::c_short;
    if strncmp(
        &mut *no_0.offset(i as isize),
        page_comp.as_mut_ptr(),
        comp_len as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        return scan_no(
            &mut *no_0.offset((comp_len + 1 as libc::c_int) as isize),
            npg,
            count,
            &mut i,
        )
    } else {
        return 1 as libc::c_int
    };
}
unsafe extern "C" fn scan_alpha_upper(
    mut no_0: *mut libc::c_char,
    mut npg: *mut libc::c_short,
    mut count: *mut libc::c_short,
) -> libc::c_int {
    let mut i: libc::c_short = 0;
    if *count as libc::c_int >= 10 as libc::c_int {
        if idx_dot != 0 {
            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
            idx_dot = 0 as libc::c_int;
        }
        fprintf(
            ilg_fp,
            b"!! Input index error (file = %s, line = %d):\n   -- \0" as *const u8
                as *const libc::c_char,
            idx_fn,
            idx_lc,
        );
        fprintf(
            ilg_fp,
            b"Page number %s has too many fields (max. %d).\0" as *const u8
                as *const libc::c_char,
            no_0,
            10 as libc::c_int,
        );
        idx_ec += 1;
        idx_ec;
        return 0 as libc::c_int;
    }
    *npg
        .offset(
            *count as isize,
        ) = ((if 'A' as i32 <= *no_0.offset(0 as libc::c_int as isize) as libc::c_int
        && *no_0.offset(0 as libc::c_int as isize) as libc::c_int <= 'Z' as i32
    {
        *no_0.offset(0 as libc::c_int as isize) as libc::c_int - 'A' as i32
    } else {
        (if 'a' as i32 <= *no_0.offset(0 as libc::c_int as isize) as libc::c_int
            && *no_0.offset(0 as libc::c_int as isize) as libc::c_int <= 'z' as i32
        {
            *no_0.offset(0 as libc::c_int as isize) as libc::c_int - 'a' as i32
        } else {
            0 as libc::c_int
        })
    }) + page_offset[4 as libc::c_int as usize]) as libc::c_short;
    *count += 1;
    *count;
    i = 1 as libc::c_int as libc::c_short;
    if strncmp(
        &mut *no_0.offset(i as isize),
        page_comp.as_mut_ptr(),
        comp_len as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        return scan_no(
            &mut *no_0.offset((comp_len + 1 as libc::c_int) as isize),
            npg,
            count,
            &mut i,
        )
    } else {
        return 1 as libc::c_int
    };
}
unsafe extern "C" fn scan_arg1() -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut a: libc::c_int = 0;
    if compress_blanks != 0 {
        loop {
            a = getc(idx_fp);
            if !(a == ' ' as i32 || a == '\t' as i32) {
                break;
            }
        }
    } else {
        a = getc(idx_fp);
    }
    while i < 1024 as libc::c_int && a != -(1 as libc::c_int) {
        if a == idx_quote as libc::c_int || a == idx_escape as libc::c_int {
            let fresh8 = i;
            i = i + 1;
            key[fresh8 as usize] = a as libc::c_char;
            a = getc(idx_fp);
            let fresh9 = i;
            i = i + 1;
            key[fresh9 as usize] = a as libc::c_char;
        } else if a == idx_aopen as libc::c_int {
            let fresh10 = i;
            i = i + 1;
            key[fresh10 as usize] = a as libc::c_char;
            n += 1;
            n;
        } else if a == idx_aclose as libc::c_int {
            if n == 0 as libc::c_int {
                if compress_blanks != 0
                    && key[(i - 1 as libc::c_int) as usize] as libc::c_int == ' ' as i32
                {
                    key[(i - 1 as libc::c_int) as usize] = '\0' as i32 as libc::c_char;
                } else {
                    key[i as usize] = '\0' as i32 as libc::c_char;
                }
                return 1 as libc::c_int;
            } else {
                let fresh11 = i;
                i = i + 1;
                key[fresh11 as usize] = a as libc::c_char;
                n -= 1;
                n;
            }
        } else {
            let mut current_block_34: u64;
            match a {
                10 => {
                    idx_lc += 1;
                    idx_lc;
                    if idx_dot != 0 {
                        fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                        idx_dot = 0 as libc::c_int;
                    }
                    fprintf(
                        ilg_fp,
                        b"!! Input index error (file = %s, line = %d):\n   -- \0"
                            as *const u8 as *const libc::c_char,
                        idx_fn,
                        idx_lc,
                    );
                    fprintf(
                        ilg_fp,
                        b"Incomplete first argument (premature LFD).\n\0" as *const u8
                            as *const libc::c_char,
                        b"\0" as *const u8 as *const libc::c_char,
                    );
                    idx_ec += 1;
                    idx_ec;
                    return 0 as libc::c_int;
                }
                9 | 32 => {
                    if compress_blanks != 0 {
                        if i > 0 as libc::c_int
                            && key[(i - 1 as libc::c_int) as usize] as libc::c_int
                                != ' ' as i32
                            && key[(i - 1 as libc::c_int) as usize] as libc::c_int
                                != '\t' as i32
                        {
                            let fresh12 = i;
                            i = i + 1;
                            key[fresh12 as usize] = ' ' as i32 as libc::c_char;
                        }
                        current_block_34 = 17500079516916021833;
                    } else {
                        current_block_34 = 18279107430620107848;
                    }
                }
                _ => {
                    current_block_34 = 18279107430620107848;
                }
            }
            match current_block_34 {
                18279107430620107848 => {
                    let fresh13 = i;
                    i = i + 1;
                    key[fresh13 as usize] = a as libc::c_char;
                }
                _ => {}
            }
        }
        a = getc(idx_fp);
    }
    flush_to_eol();
    idx_lc += 1;
    idx_lc;
    if idx_dot != 0 {
        fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
        idx_dot = 0 as libc::c_int;
    }
    fprintf(
        ilg_fp,
        b"!! Input index error (file = %s, line = %d):\n   -- \0" as *const u8
            as *const libc::c_char,
        idx_fn,
        idx_lc,
    );
    fprintf(
        ilg_fp,
        b"First argument too long (max %d).\n\0" as *const u8 as *const libc::c_char,
        1024 as libc::c_int,
    );
    idx_ec += 1;
    idx_ec;
    return 0 as libc::c_int;
}
unsafe extern "C" fn scan_arg2() -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut a: libc::c_int = 0;
    let mut hit_blank: libc::c_int = 0 as libc::c_int;
    loop {
        a = getc(idx_fp);
        if !(a == ' ' as i32 || a == '\t' as i32) {
            break;
        }
    }
    while i < 16 as libc::c_int {
        if a == idx_aclose as libc::c_int {
            no[i as usize] = '\0' as i32 as libc::c_char;
            return 1 as libc::c_int;
        } else {
            match a {
                10 => {
                    idx_lc += 1;
                    idx_lc;
                    if idx_dot != 0 {
                        fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                        idx_dot = 0 as libc::c_int;
                    }
                    fprintf(
                        ilg_fp,
                        b"!! Input index error (file = %s, line = %d):\n   -- \0"
                            as *const u8 as *const libc::c_char,
                        idx_fn,
                        idx_lc,
                    );
                    fprintf(
                        ilg_fp,
                        b"Incomplete second argument (premature LFD).\n\0" as *const u8
                            as *const libc::c_char,
                        b"\0" as *const u8 as *const libc::c_char,
                    );
                    idx_ec += 1;
                    idx_ec;
                    return 0 as libc::c_int;
                }
                9 | 32 => {
                    hit_blank = 1 as libc::c_int;
                }
                _ => {
                    if hit_blank != 0 {
                        flush_to_eol();
                        idx_lc += 1;
                        idx_lc;
                        if idx_dot != 0 {
                            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                            idx_dot = 0 as libc::c_int;
                        }
                        fprintf(
                            ilg_fp,
                            b"!! Input index error (file = %s, line = %d):\n   -- \0"
                                as *const u8 as *const libc::c_char,
                            idx_fn,
                            idx_lc,
                        );
                        fprintf(
                            ilg_fp,
                            b"Illegal space within numerals in second argument.\n\0"
                                as *const u8 as *const libc::c_char,
                            b"\0" as *const u8 as *const libc::c_char,
                        );
                        idx_ec += 1;
                        idx_ec;
                        return 0 as libc::c_int;
                    }
                    let fresh14 = i;
                    i = i + 1;
                    no[fresh14 as usize] = a as libc::c_char;
                }
            }
        }
        a = getc(idx_fp);
    }
    flush_to_eol();
    idx_lc += 1;
    idx_lc;
    if idx_dot != 0 {
        fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
        idx_dot = 0 as libc::c_int;
    }
    fprintf(
        ilg_fp,
        b"!! Input index error (file = %s, line = %d):\n   -- \0" as *const u8
            as *const libc::c_char,
        idx_fn,
        idx_lc,
    );
    fprintf(
        ilg_fp,
        b"Second argument too long (max %d).\n\0" as *const u8 as *const libc::c_char,
        16 as libc::c_int,
    );
    idx_ec += 1;
    idx_ec;
    return 0 as libc::c_int;
}
unsafe extern "C" fn search_quote(
    mut sort_key: *mut libc::c_char,
    mut actual_key: *mut libc::c_char,
) {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sort: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut char_found: libc::c_int = 0 as libc::c_int;
    strcpy(actual_key, sort_key as *const libc::c_char);
    ptr = strchr(sort_key as *const libc::c_char, '"' as i32);
    while !ptr.is_null() {
        sort = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        match *ptr.offset(1 as libc::c_int as isize) as libc::c_int {
            97 | 65 => {
                sort = (if *(*__ctype_b_loc())
                    .offset(
                        *ptr.offset(1 as libc::c_int as isize) as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    b"Ae\0" as *const u8 as *const libc::c_char
                } else {
                    b"ae\0" as *const u8 as *const libc::c_char
                }) as *mut libc::c_char;
            }
            111 | 79 => {
                sort = (if *(*__ctype_b_loc())
                    .offset(
                        *ptr.offset(1 as libc::c_int as isize) as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    b"Oe\0" as *const u8 as *const libc::c_char
                } else {
                    b"oe\0" as *const u8 as *const libc::c_char
                }) as *mut libc::c_char;
            }
            117 | 85 => {
                sort = (if *(*__ctype_b_loc())
                    .offset(
                        *ptr.offset(1 as libc::c_int as isize) as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    b"Ue\0" as *const u8 as *const libc::c_char
                } else {
                    b"ue\0" as *const u8 as *const libc::c_char
                }) as *mut libc::c_char;
            }
            115 => {
                sort = b"ss\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            _ => {}
        }
        if *sort.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
            char_found = 1 as libc::c_int;
            *ptr = *sort.offset(0 as libc::c_int as isize);
            *ptr
                .offset(
                    1 as libc::c_int as isize,
                ) = *sort.offset(1 as libc::c_int as isize);
        }
        ptr = strchr(ptr.offset(1 as libc::c_int as isize), '"' as i32);
    }
    if char_found == 0 {
        *actual_key.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    }
}
