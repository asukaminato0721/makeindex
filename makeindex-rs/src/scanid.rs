#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use libc::*;
use libc_stdhandle::*;
extern "C" {
    // pub type _IO_wide_data;
    // pub type _IO_codecvt;
    // pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn getc(__stream: *mut FILE) -> i32;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    static mut compress_blanks: i32;
    static mut verbose: i32;
    static mut german_sort: i32;
    static mut idx_keyword: [libc::c_char; 1024];
    static mut idx_aopen: libc::c_char;
    static mut idx_aclose: libc::c_char;
    static mut idx_level: libc::c_char;
    static mut idx_quote: libc::c_char;
    static mut idx_actual: libc::c_char;
    static mut idx_encap: libc::c_char;
    static mut idx_escape: libc::c_char;
    static mut page_comp: [libc::c_char; 1024];
    static mut page_offset: [i32; 5];
    static mut idx_fp: *mut FILE;
    static mut ilg_fp: *mut FILE;
    static mut idx_fn: *mut libc::c_char;
    static mut pgm_fn: *mut libc::c_char;
    static mut idx_dot: i32;
    static mut idx_tt: i32;
    static mut idx_et: i32;
    fn strtoint(str: *mut libc::c_char) -> i32;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: i32) -> !;
}
type size_t = libc::c_ulong;
type __off_t = libc::c_long;
type __off64_t = libc::c_long;

type _IO_lock_t = ();
type u32 = libc::c_uint;
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
type FIELD = KFIELD;
type FIELD_PTR = *mut KFIELD;
#[derive(Copy, Clone)]
#[repr(C)]
struct KNODE {
    pub data: FIELD,
    pub next: *mut KNODE,
}
type NODE = KNODE;
type NODE_PTR = *mut KNODE;
#[no_mangle]
pub static mut idx_lc: i32 = 0;
#[no_mangle]
pub static mut idx_tc: i32 = 0;
#[no_mangle]
pub static mut idx_ec: i32 = 0;
#[no_mangle]
pub static mut idx_dc: i32 = 0;
static mut first_entry: i32 = 1;
static mut comp_len: i32 = 0;
static mut key: [libc::c_char; 1024] = [0; 1024];
static mut no: [libc::c_char; 16] = [0; 16];
#[no_mangle]
static mut head: NODE_PTR = 0 as *const KNODE as *mut KNODE;
#[no_mangle]
static mut tail: NODE_PTR = 0 as *const KNODE as *mut KNODE;
#[no_mangle]
pub unsafe extern "C" fn scan_idx() {
    let mut keyword = [0; 1024];
    let mut c = 0;
    let mut i = 0;
    let mut not_eof = 1;
    let mut arg_count = -1;
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
    idx_dc = 0;
    idx_ec = idx_dc;
    idx_tc = idx_ec;
    idx_lc = idx_tc;
    comp_len = strlen(page_comp.as_mut_ptr()) as i32;
    while not_eof != 0 {
        c = getc(idx_fp);
        match c {
            -1 => {
                if arg_count == 2 {
                    idx_lc += 1;

                    if make_key() != 0 {
                        idx_dot = 1;
                        let fresh0 = idx_dc;
                        idx_dc += 1;
                        if fresh0 == 0 {
                            if verbose != 0 {
                                fprintf(stderr, b".\0" as *const u8 as *const libc::c_char);
                            }
                            fprintf(ilg_fp, b".\0" as *const u8 as *const libc::c_char);
                        }
                        if idx_dc == 1000 {
                            idx_dc = 0;
                        }
                    }
                    arg_count = -1;
                } else {
                    not_eof = 0;
                }
            }
            10 => {
                idx_lc += 1;

                if arg_count == 2 {
                    if make_key() != 0 {
                        idx_dot = 1;
                        let fresh1 = idx_dc;
                        idx_dc += 1;
                        if fresh1 == 0 {
                            if verbose != 0 {
                                fprintf(stderr, b".\0" as *const u8 as *const libc::c_char);
                            }
                            fprintf(ilg_fp, b".\0" as *const u8 as *const libc::c_char);
                        }
                        if idx_dc == 1000 {
                            idx_dc = 0;
                        }
                    }
                    arg_count = -1;
                } else if arg_count > -1 {
                    if idx_dot != 0 {
                        fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                        idx_dot = 0;
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
                        b"Missing arguments -- need two (premature LFD).\n\0" as *const u8
                            as *const libc::c_char,
                        std::ptr::null_mut::<libc::c_void>(),
                    );
                    idx_ec += 1;

                    arg_count = -1;
                }
            }
            9 | 32 => {}
            _ => match arg_count {
                -1 => {
                    i = 0;
                    let fresh2 = i;
                    i += 1;
                    keyword[fresh2 as usize] = c as libc::c_char;
                    arg_count += 1;
                    arg_count;
                    idx_tc += 1;
                    idx_tc;
                }
                0 => {
                    if c == idx_aopen as i32 {
                        arg_count += 1;
                        arg_count;
                        keyword[i as usize] = '\0' as i32 as libc::c_char;
                        if strcmp(keyword.as_mut_ptr(), idx_keyword.as_mut_ptr()) == 0 {
                            if scan_arg1() == 0 {
                                arg_count = -1;
                            }
                        } else {
                            let mut tmp = 0;
                            loop {
                                tmp = getc(idx_fp);
                                if tmp == '\n' as i32 {
                                    break;
                                }
                                if tmp == -1 {
                                    break;
                                }
                            }
                            idx_lc += 1;

                            arg_count = -1;
                            if idx_dot != 0 {
                                fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                                idx_dot = 0;
                            }
                            fprintf(
                                ilg_fp,
                                b"!! Input index error (file = %s, line = %d):\n   -- \0"
                                    as *const u8
                                    as *const libc::c_char,
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
                        }
                    } else if i < 1024 {
                        let fresh3 = i;
                        i += 1;
                        keyword[fresh3 as usize] = c as libc::c_char;
                    } else {
                        let mut tmp_0 = 0;
                        loop {
                            tmp_0 = getc(idx_fp);
                            if tmp_0 == '\n' as i32 {
                                break;
                            }
                            if tmp_0 == -1 {
                                break;
                            }
                        }
                        idx_lc += 1;

                        arg_count = -1;
                        if idx_dot != 0 {
                            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                            idx_dot = 0;
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
                            b"Index keyword %s too long (max %d).\n\0" as *const u8
                                as *const libc::c_char,
                            keyword.as_mut_ptr(),
                            1024,
                        );
                        idx_ec += 1;
                    }
                }
                1 => {
                    if c == idx_aopen as i32 {
                        arg_count += 1;
                        arg_count;
                        if scan_arg2() == 0 {
                            arg_count = -1;
                        }
                    } else {
                        let mut tmp_1 = 0;
                        loop {
                            tmp_1 = getc(idx_fp);
                            if tmp_1 == '\n' as i32 {
                                break;
                            }
                            if tmp_1 == -1 {
                                break;
                            }
                        }
                        idx_lc += 1;

                        arg_count = -1;
                        if idx_dot != 0 {
                            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                            idx_dot = 0;
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
                                b"No opening delimiter for second argument (illegal character `%c').\n\0"
                                    as *const u8 as *const libc::c_char,
                                c,
                            );
                        idx_ec += 1;
                    }
                }
                2 => {
                    let mut tmp_2 = 0;
                    loop {
                        tmp_2 = getc(idx_fp);
                        if tmp_2 == '\n' as i32 {
                            break;
                        }
                        if tmp_2 == -1 {
                            break;
                        }
                    }
                    idx_lc += 1;

                    arg_count = -1;
                    if idx_dot != 0 {
                        fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                        idx_dot = 0;
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
                        b"No closing delimiter for second argument (illegal character `%c').\n\0"
                            as *const u8 as *const libc::c_char,
                        c,
                    );
                    idx_ec += 1;
                }
                _ => {}
            },
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
    let mut a = 0;
    loop {
        a = getc(idx_fp);
        if !(a != '\n' as i32 && a != -1) {
            break;
        }
    }
}
unsafe extern "C" fn make_key() -> i32 {
    let mut ptr = std::ptr::null_mut::<KNODE>();
    let mut i = 0;
    ptr = malloc(::core::mem::size_of::<NODE>().try_into().unwrap()) as NODE_PTR;
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
        exit(1);
    }
    i = 0;
    while i < 3 {
        (*ptr).data.sf[i as usize] = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        (*ptr).data.af[i as usize] = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        i += 1;
        i;
    }
    (*ptr).data.encap = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*ptr).data.lpg[0] = '\0' as i32 as libc::c_char;
    (*ptr).data.count = 0 as libc::c_short;
    (*ptr).data.type_0 = -9999 as libc::c_short;
    if scan_key(&mut (*ptr).data) == 0 {
        return 0;
    }
    (*ptr).data.group = group_type((*ptr).data.sf[0]);
    strcpy(((*ptr).data.lpg).as_mut_ptr(), no.as_mut_ptr());
    if scan_no(
        no.as_mut_ptr(),
        ((*ptr).data.npg).as_mut_ptr(),
        &mut (*ptr).data.count,
        &mut (*ptr).data.type_0,
    ) == 0
    {
        return 0;
    }
    if first_entry != 0 {
        tail = ptr;
        head = tail;
        first_entry = 0;
    } else {
        (*tail).next = ptr;
        tail = ptr;
    }
    (*ptr).data.lc = idx_lc;
    (*ptr).data.fn_0 = idx_fn;
    (*tail).next = std::ptr::null_mut::<KNODE>();
    1
}
unsafe extern "C" fn make_string(mut ppstr: *mut *mut libc::c_char, mut n: i32) {
    if *(*ppstr).offset(0) as i32 == '\0' as i32 {
        *ppstr = malloc(n.try_into().unwrap()) as *mut libc::c_char;
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
            exit(1);
        }
        *(*ppstr).offset(0) = '\0' as i32 as libc::c_char;
    }
}
unsafe extern "C" fn scan_key(mut data: FIELD_PTR) -> i32 {
    let mut i = 0;
    let mut n = 0;
    let mut second_round = 0;
    let mut last = 3 - 1;
    while key[n as usize] as i32 != '\0' as i32 {
        if key[n as usize] as i32 == idx_encap as i32 {
            n += 1;
          
            make_string(
                &mut (*data).encap,
                (strlen(key.as_mut_ptr())).wrapping_add(1) as i32,
            );
            if scan_field(
                &mut n,
                (*data).encap,
                strlen(key.as_mut_ptr()) as i32,
                0,
                0,
                0,
            ) != 0
            {
                break;
            }
            return 0;
        } else if key[n as usize] as i32 == idx_actual as i32 {
            n += 1;
            if i == last {
                make_string(
                    &mut *((*data).af).as_mut_ptr().offset(i as isize),
                    (strlen(key.as_mut_ptr())).wrapping_add(1) as i32,
                );
                if scan_field(
                    &mut n,
                    (*data).af[i as usize],
                    strlen(key.as_mut_ptr()) as i32,
                    0,
                    1,
                    0,
                ) == 0
                {
                    return 0;
                }
            } else {
                make_string(
                    &mut *((*data).af).as_mut_ptr().offset(i as isize),
                    (strlen(key.as_mut_ptr())).wrapping_add(1) as i32,
                );
                if scan_field(
                    &mut n,
                    (*data).af[i as usize],
                    strlen(key.as_mut_ptr()) as i32,
                    1,
                    1,
                    0,
                ) == 0
                {
                    return 0;
                }
            }
        } else {
            if second_round != 0 {
                i += 1;

                n += 1;
            }
            if i == last {
                make_string(
                    &mut *((*data).sf).as_mut_ptr().offset(i as isize),
                    (strlen(key.as_mut_ptr())).wrapping_add(1) as i32,
                );
                if scan_field(
                    &mut n,
                    (*data).sf[i as usize],
                    strlen(key.as_mut_ptr()) as i32,
                    0,
                    1,
                    1,
                ) == 0
                {
                    return 0;
                }
            } else {
                make_string(
                    &mut *((*data).sf).as_mut_ptr().offset(i as isize),
                    (strlen(key.as_mut_ptr())).wrapping_add(1) as i32,
                );
                if scan_field(
                    &mut n,
                    (*data).sf[i as usize],
                    strlen(key.as_mut_ptr()) as i32,
                    1,
                    1,
                    1,
                ) == 0
                {
                    return 0;
                }
            }
            second_round = 1;
            if german_sort != 0 && !(strchr((*data).sf[i as usize], '"' as i32)).is_null() {
                make_string(
                    &mut *((*data).af).as_mut_ptr().offset(i as isize),
                    (strlen((*data).sf[i as usize])).wrapping_add(1) as i32,
                );
                search_quote((*data).sf[i as usize], (*data).af[i as usize]);
            }
        }
    }
    if *(*data).sf[0] as i32 == '\0' as i32 {
        if idx_dot != 0 {
            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
            idx_dot = 0;
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
            std::ptr::null_mut::<libc::c_void>(),
        );
        idx_ec += 1;

        return 0;
    }
    i = 1;
    while i < 3 - 1 {
        if *(*data).sf[i as usize] as i32 == '\0' as i32
            && (*(*data).af[i as usize] as i32 != '\0' as i32
                || *(*data).sf[(i + 1) as usize] as i32 != '\0' as i32)
        {
            if idx_dot != 0 {
                fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                idx_dot = 0;
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
                std::ptr::null_mut::<libc::c_void>(),
            );
            idx_ec += 1;

            return 0;
        }
        i += 1;
        i;
    }
    if *(*data).sf[i as usize] as i32 == '\0' as i32
        && *(*data).af[i as usize] as i32 != '\0' as i32
    {
        if idx_dot != 0 {
            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
            idx_dot = 0;
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
            std::ptr::null_mut::<libc::c_void>(),
        );
        idx_ec += 1;

        return 0;
    }
    1
}
unsafe extern "C" fn scan_field(
    mut n: *mut i32,
    mut field: *mut libc::c_char,
    mut len_field: i32,
    mut ck_level: i32,
    mut ck_encap: i32,
    mut ck_actual: i32,
) -> i32 {
    let mut current_block: u64;
    let mut i = 0;
    let mut nbsh = 0;
    if compress_blanks != 0
        && (key[*n as usize] as i32 == ' ' as i32 || key[*n as usize] as i32 == '\t' as i32)
    {
        *n += 1;
    }
    loop {
        nbsh = 0;
        loop {
            if key[*n as usize] as i32 != idx_escape as i32 {
                current_block = 10886091980245723256;
                break;
            }
            nbsh += 1;
            nbsh;
            let fresh4 = i;
            i += 1;
            *field.offset(fresh4 as isize) = key[*n as usize];
            if i > len_field {
                current_block = 12901009814790378471;
                break;
            }
            *n += 1;
        }
        if current_block == 10886091980245723256 {
            if key[*n as usize] as i32 == idx_quote as i32 {
                if nbsh % 2 == 0 {
                    *n += 1;
                    let fresh5 = i;
                    i += 1;
                    *field.offset(fresh5 as isize) = key[*n as usize];
                } else {
                    let fresh6 = i;
                    i += 1;
                    *field.offset(fresh6 as isize) = key[*n as usize];
                }
                if i > len_field {
                    current_block = 12901009814790378471;
                } else {
                    current_block = 1622411330066726685;
                }
            } else if ck_level != 0 && key[*n as usize] as i32 == idx_level as i32
                || ck_encap != 0 && key[*n as usize] as i32 == idx_encap as i32
                || ck_actual != 0 && key[*n as usize] as i32 == idx_actual as i32
                || key[*n as usize] as i32 == '\0' as i32
            {
                if i > 0
                    && compress_blanks != 0
                    && *field.offset((i - 1) as isize) as i32 == ' ' as i32
                {
                    *field.offset((i - 1) as isize) = '\0' as i32 as libc::c_char;
                } else {
                    *field.offset(i as isize) = '\0' as i32 as libc::c_char;
                }
                return 1;
            } else {
                let fresh7 = i;
                i += 1;
                *field.offset(fresh7 as isize) = key[*n as usize];
                if i > len_field {
                    current_block = 12901009814790378471;
                } else {
                    if ck_level == 0 && key[*n as usize] as i32 == idx_level as i32 {
                        if idx_dot != 0 {
                            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                            idx_dot = 0;
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
                            b"Extra `%c' at position %d of first argument.\n\0" as *const u8
                                as *const libc::c_char,
                            idx_level as i32,
                            *n + 1,
                        );
                        idx_ec += 1;

                        return 0;
                    } else if ck_encap == 0 && key[*n as usize] as i32 == idx_encap as i32 {
                        if idx_dot != 0 {
                            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                            idx_dot = 0;
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
                            b"Extra `%c' at position %d of first argument.\n\0" as *const u8
                                as *const libc::c_char,
                            idx_encap as i32,
                            *n + 1,
                        );
                        idx_ec += 1;

                        return 0;
                    } else if ck_actual == 0 && key[*n as usize] as i32 == idx_actual as i32 {
                        if idx_dot != 0 {
                            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                            idx_dot = 0;
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
                            b"Extra `%c' at position %d of first argument.\n\0" as *const u8
                                as *const libc::c_char,
                            idx_actual as i32,
                            *n + 1,
                        );
                        idx_ec += 1;

                        return 0;
                    }
                    current_block = 1622411330066726685;
                }
            }
            match current_block {
                12901009814790378471 => {}
                _ => {
                    if i <= len_field {
                        *n += 1;

                        continue;
                    }
                }
            }
        }
        if ck_encap == 0 {
            if idx_dot != 0 {
                fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                idx_dot = 0;
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
        } else if ck_actual != 0 {
            if idx_dot != 0 {
                fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                idx_dot = 0;
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
                b"Index sort key too long (max. %d).\n\0" as *const u8 as *const libc::c_char,
                len_field,
            );
            idx_ec += 1;
        } else {
            if idx_dot != 0 {
                fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                idx_dot = 0;
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
                b"Text of key entry too long (max. %d).\n\0" as *const u8 as *const libc::c_char,
                len_field,
            );
            idx_ec += 1;
        }
        return 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn group_type(mut str: *mut libc::c_char) -> i32 {
    let mut i = 0;
    while *str.offset(i as isize) as i32 != '\0' as i32
        && ('0' as i32 <= *str.offset(i as isize) as i32
            && *str.offset(i as isize) as i32 <= '9' as i32)
    {
        i += 1;
        i;
    }
    if *str.offset(i as isize) as i32 == '\0' as i32 {
        sscanf(
            str,
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut i as *mut i32,
        );
        i
    } else if '!' as i32 <= *str.offset(0) as i32 && *str.offset(0) as i32 <= '@' as i32
        || '[' as i32 <= *str.offset(0) as i32 && *str.offset(0) as i32 <= '`' as i32
        || '{' as i32 <= *str.offset(0) as i32 && *str.offset(0) as i32 <= '~' as i32
    {
        -1
    } else {
        -2
    }
}
unsafe extern "C" fn scan_no(
    mut no_0: *mut libc::c_char,
    mut npg: *mut libc::c_short,
    mut count: *mut libc::c_short,
    mut type_0: *mut libc::c_short,
) -> i32 {
    let mut i = 1;
    if *(*__ctype_b_loc()).offset(*no_0.offset(0) as i32 as isize) as i32
        & _ISdigit as i32 as libc::c_ushort as i32
        != 0
    {
        *type_0 = 2 as libc::c_short;
        if scan_arabic(no_0, npg, count) == 0 {
            return 0;
        }
    } else if (*no_0.offset(0) as i32 == 'i' as i32
        || *no_0.offset(0) as i32 == 'v' as i32
        || *no_0.offset(0) as i32 == 'x' as i32
        || *no_0.offset(0) as i32 == 'l' as i32
        || *no_0.offset(0) as i32 == 'c' as i32
        || *no_0.offset(0) as i32 == 'd' as i32
        || *no_0.offset(0) as i32 == 'm' as i32)
        && (strncmp(
            &mut *no_0.offset(i as isize),
            page_comp.as_mut_ptr(),
            comp_len.try_into().unwrap(),
        ) != 0)
    {
        *type_0 = 0 as libc::c_short;
        if scan_roman_lower(no_0, npg, count) == 0 {
            return 0;
        }
    } else if (*no_0.offset(0) as i32 == 'I' as i32
        || *no_0.offset(0) as i32 == 'V' as i32
        || *no_0.offset(0) as i32 == 'X' as i32
        || *no_0.offset(0) as i32 == 'L' as i32
        || *no_0.offset(0) as i32 == 'C' as i32
        || *no_0.offset(0) as i32 == 'D' as i32
        || *no_0.offset(0) as i32 == 'M' as i32)
        && (*no_0.offset(0) as i32 == 'I' as i32
            || (strncmp(
                &mut *no_0.offset(i as isize),
                page_comp.as_mut_ptr(),
                comp_len.try_into().unwrap(),
            ) != 0))
    {
        *type_0 = 1 as libc::c_short;
        if scan_roman_upper(no_0, npg, count) == 0 {
            return 0;
        }
    } else if 'a' as i32 <= *no_0.offset(0) as i32 && *no_0.offset(0) as i32 <= 'z' as i32 {
        *type_0 = 3 as libc::c_short;
        if scan_alpha_lower(no_0, npg, count) == 0 {
            return 0;
        }
    } else if 'A' as i32 <= *no_0.offset(0) as i32 && *no_0.offset(0) as i32 <= 'Z' as i32 {
        *type_0 = 4 as libc::c_short;
        if scan_alpha_upper(no_0, npg, count) == 0 {
            return 0;
        }
    } else {
        if idx_dot != 0 {
            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
            idx_dot = 0;
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

        return 0;
    }
    1
}
unsafe extern "C" fn scan_arabic(
    mut no_0: *mut libc::c_char,
    mut npg: *mut libc::c_short,
    mut count: *mut libc::c_short,
) -> i32 {
    let mut i = 0;
    let mut str = [0; 6];
    while *no_0.offset(i as isize) as i32 != '\0' as i32
        && i as i32 <= 5
        && (strncmp(
            &mut *no_0.offset(i as isize),
            page_comp.as_mut_ptr(),
            comp_len.try_into().unwrap(),
        ) != 0)
    {
        if *(*__ctype_b_loc()).offset(*no_0.offset(i as isize) as i32 as isize) as i32
            & _ISdigit as i32 as libc::c_ushort as i32
            != 0
        {
            str[i as usize] = *no_0.offset(i as isize);
            i += 1;
            i;
        } else {
            if idx_dot != 0 {
                fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                idx_dot = 0;
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
                b"Illegal Arabic digit: position %d in %s.\n\0" as *const u8 as *const libc::c_char,
                i as i32 + 1,
                no_0,
            );
            idx_ec += 1;

            return 0;
        }
    }
    if i as i32 > 5 {
        if idx_dot != 0 {
            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
            idx_dot = 0;
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
            5,
        );
        idx_ec += 1;

        return 0;
    }
    str[i as usize] = '\0' as i32 as libc::c_char;
    if *count as i32 >= 10 {
        if idx_dot != 0 {
            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
            idx_dot = 0;
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
            b"Page number %s has too many fields (max. %d).\0" as *const u8 as *const libc::c_char,
            no_0,
            10,
        );
        idx_ec += 1;

        return 0;
    }
    *npg.offset(*count as isize) = (strtoint(str.as_mut_ptr()) + page_offset[2]) as libc::c_short;
    *count += 1;
    *count;
    if strncmp(
        &mut *no_0.offset(i as isize),
        page_comp.as_mut_ptr(),
        comp_len.try_into().unwrap(),
    ) == 0
    {
        scan_no(
            &mut *no_0.offset((i as i32 + comp_len) as isize),
            npg,
            count,
            &mut i,
        )
    } else {
        1
    }
}
unsafe extern "C" fn scan_roman_lower(
    mut no_0: *mut libc::c_char,
    mut npg: *mut libc::c_short,
    mut count: *mut libc::c_short,
) -> i32 {
    let mut i = 0;
    let mut inp = 0;
    let mut prev = 0;
    let mut the_new = 0;
    while *no_0.offset(i as isize) as i32 != '\0' as i32
        && (i as i32) < 16
        && (strncmp(
            &mut *no_0.offset(i as isize),
            page_comp.as_mut_ptr(),
            comp_len.try_into().unwrap(),
        ) != 0)
    {
        if (*no_0.offset(i as isize) as i32 == 'i' as i32
            || *no_0.offset(i as isize) as i32 == 'v' as i32
            || *no_0.offset(i as isize) as i32 == 'x' as i32
            || *no_0.offset(i as isize) as i32 == 'l' as i32
            || *no_0.offset(i as isize) as i32 == 'c' as i32
            || *no_0.offset(i as isize) as i32 == 'd' as i32
            || *no_0.offset(i as isize) as i32 == 'm' as i32)
            && {
                the_new = if *no_0.offset(i as isize) as i32 == 'i' as i32 {
                    1
                } else if *no_0.offset(i as isize) as i32 == 'v' as i32 {
                    5
                } else if *no_0.offset(i as isize) as i32 == 'x' as i32 {
                    10
                } else if *no_0.offset(i as isize) as i32 == 'l' as i32 {
                    50
                } else if *no_0.offset(i as isize) as i32 == 'c' as i32 {
                    100
                } else if *no_0.offset(i as isize) as i32 == 'd' as i32 {
                    500
                } else if *no_0.offset(i as isize) as i32 == 'm' as i32 {
                    1000
                } else {
                    0
                };
                the_new != 0
            }
        {
            if prev == 0 {
                prev = the_new;
            } else {
                if prev < the_new {
                    prev = the_new - prev;
                    the_new = 0;
                }
                inp += prev;
                prev = the_new;
            }
        } else {
            if idx_dot != 0 {
                fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                idx_dot = 0;
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
                b"Illegal Roman number: position %d in %s.\n\0" as *const u8 as *const libc::c_char,
                i as i32 + 1,
                no_0,
            );
            idx_ec += 1;

            return 0;
        }
        i += 1;
        i;
    }
    if i as i32 == 16 {
        if idx_dot != 0 {
            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
            idx_dot = 0;
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
            16,
        );
        idx_ec += 1;

        return 0;
    }
    inp += prev;
    if *count as i32 >= 10 {
        if idx_dot != 0 {
            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
            idx_dot = 0;
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
            b"Page number %s has too many fields (max. %d).\0" as *const u8 as *const libc::c_char,
            no_0,
            10,
        );
        idx_ec += 1;

        return 0;
    }
    *npg.offset(*count as isize) = (inp + page_offset[0]) as libc::c_short;
    *count += 1;
    *count;
    if strncmp(
        &mut *no_0.offset(i as isize),
        page_comp.as_mut_ptr(),
        comp_len.try_into().unwrap(),
    ) == 0
    {
        scan_no(
            &mut *no_0.offset((i as i32 + comp_len) as isize),
            npg,
            count,
            &mut i,
        )
    } else {
        1
    }
}
unsafe extern "C" fn scan_roman_upper(
    mut no_0: *mut libc::c_char,
    mut npg: *mut libc::c_short,
    mut count: *mut libc::c_short,
) -> i32 {
    let mut i = 0;
    let mut inp = 0;
    let mut prev = 0;
    let mut the_new = 0;
    while *no_0.offset(i as isize) as i32 != '\0' as i32
        && (i as i32) < 16
        && (strncmp(
            &mut *no_0.offset(i as isize),
            page_comp.as_mut_ptr(),
            comp_len.try_into().unwrap(),
        ) != 0)
    {
        if (*no_0.offset(i as isize) as i32 == 'I' as i32
            || *no_0.offset(i as isize) as i32 == 'V' as i32
            || *no_0.offset(i as isize) as i32 == 'X' as i32
            || *no_0.offset(i as isize) as i32 == 'L' as i32
            || *no_0.offset(i as isize) as i32 == 'C' as i32
            || *no_0.offset(i as isize) as i32 == 'D' as i32
            || *no_0.offset(i as isize) as i32 == 'M' as i32)
            && {
                the_new = if *no_0.offset(i as isize) as i32 == 'I' as i32 {
                    1
                } else if *no_0.offset(i as isize) as i32 == 'V' as i32 {
                    5
                } else if *no_0.offset(i as isize) as i32 == 'X' as i32 {
                    10
                } else if *no_0.offset(i as isize) as i32 == 'L' as i32 {
                    50
                } else if *no_0.offset(i as isize) as i32 == 'C' as i32 {
                    100
                } else if *no_0.offset(i as isize) as i32 == 'D' as i32 {
                    500
                } else if *no_0.offset(i as isize) as i32 == 'M' as i32 {
                    1000
                } else {
                    0
                };
                the_new != 0
            }
        {
            if prev == 0 {
                prev = the_new;
            } else {
                if prev < the_new {
                    prev = the_new - prev;
                    the_new = 0;
                }
                inp += prev;
                prev = the_new;
            }
        } else {
            if idx_dot != 0 {
                fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                idx_dot = 0;
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
                b"Illegal Roman number: position %d in %s.\n\0" as *const u8 as *const libc::c_char,
                i as i32 + 1,
                no_0,
            );
            idx_ec += 1;

            return 0;
        }
        i += 1;
        i;
    }
    if i as i32 == 16 {
        if idx_dot != 0 {
            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
            idx_dot = 0;
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
            16,
        );
        idx_ec += 1;

        return 0;
    }
    inp += prev;
    if *count as i32 >= 10 {
        if idx_dot != 0 {
            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
            idx_dot = 0;
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
            b"Page number %s has too many fields (max. %d).\0" as *const u8 as *const libc::c_char,
            no_0,
            10,
        );
        idx_ec += 1;

        return 0;
    }
    *npg.offset(*count as isize) = (inp + page_offset[1]) as libc::c_short;
    *count += 1;
    *count;
    if strncmp(
        &mut *no_0.offset(i as isize),
        page_comp.as_mut_ptr(),
        comp_len.try_into().unwrap(),
    ) == 0
    {
        scan_no(
            &mut *no_0.offset((i as i32 + comp_len) as isize),
            npg,
            count,
            &mut i,
        )
    } else {
        1
    }
}
unsafe extern "C" fn scan_alpha_lower(
    mut no_0: *mut libc::c_char,
    mut npg: *mut libc::c_short,
    mut count: *mut libc::c_short,
) -> i32 {
    let mut i = 0;
    if *count as i32 >= 10 {
        if idx_dot != 0 {
            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
            idx_dot = 0;
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
            b"Page number %s has too many fields (max. %d).\0" as *const u8 as *const libc::c_char,
            no_0,
            10,
        );
        idx_ec += 1;

        return 0;
    }
    *npg.offset(*count as isize) =
        ((if 'A' as i32 <= *no_0.offset(0) as i32 && *no_0.offset(0) as i32 <= 'Z' as i32 {
            *no_0.offset(0) as i32 - 'A' as i32
        } else if 'a' as i32 <= *no_0.offset(0) as i32 && *no_0.offset(0) as i32 <= 'z' as i32 {
            *no_0.offset(0) as i32 - 'a' as i32
        } else {
            0
        }) + page_offset[3]) as libc::c_short;
    *count += 1;
    *count;
    i = 1 as libc::c_short;
    if strncmp(
        &mut *no_0.offset(i as isize),
        page_comp.as_mut_ptr(),
        comp_len.try_into().unwrap(),
    ) == 0
    {
        scan_no(
            &mut *no_0.offset((comp_len + 1) as isize),
            npg,
            count,
            &mut i,
        )
    } else {
        1
    }
}
unsafe extern "C" fn scan_alpha_upper(
    mut no_0: *mut libc::c_char,
    mut npg: *mut libc::c_short,
    mut count: *mut libc::c_short,
) -> i32 {
    let mut i = 0;
    if *count as i32 >= 10 {
        if idx_dot != 0 {
            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
            idx_dot = 0;
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
            b"Page number %s has too many fields (max. %d).\0" as *const u8 as *const libc::c_char,
            no_0,
            10,
        );
        idx_ec += 1;

        return 0;
    }
    *npg.offset(*count as isize) =
        ((if 'A' as i32 <= *no_0.offset(0) as i32 && *no_0.offset(0) as i32 <= 'Z' as i32 {
            *no_0.offset(0) as i32 - 'A' as i32
        } else if 'a' as i32 <= *no_0.offset(0) as i32 && *no_0.offset(0) as i32 <= 'z' as i32 {
            *no_0.offset(0) as i32 - 'a' as i32
        } else {
            0
        }) + page_offset[4]) as libc::c_short;
    *count += 1;
    *count;
    i = 1 as libc::c_short;
    if strncmp(
        &mut *no_0.offset(i as isize),
        page_comp.as_mut_ptr(),
        comp_len.try_into().unwrap(),
    ) == 0
    {
        scan_no(
            &mut *no_0.offset((comp_len + 1) as isize),
            npg,
            count,
            &mut i,
        )
    } else {
        1
    }
}
unsafe extern "C" fn scan_arg1() -> i32 {
    let mut i = 0;
    let mut n = 0;
    let mut a = 0;
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
    while i < 1024 && a != -1 {
        if a == idx_quote as i32 || a == idx_escape as i32 {
            let fresh8 = i;
            i += 1;
            key[fresh8 as usize] = a as libc::c_char;
            a = getc(idx_fp);
            let fresh9 = i;
            i += 1;
            key[fresh9 as usize] = a as libc::c_char;
        } else if a == idx_aopen as i32 {
            let fresh10 = i;
            i += 1;
            key[fresh10 as usize] = a as libc::c_char;
            n += 1;
          
        } else if a == idx_aclose as i32 {
            if n == 0 {
                if compress_blanks != 0 && key[(i - 1) as usize] as i32 == ' ' as i32 {
                    key[(i - 1) as usize] = '\0' as i32 as libc::c_char;
                } else {
                    key[i as usize] = '\0' as i32 as libc::c_char;
                }
                return 1;
            } else {
                let fresh11 = i;
                i += 1;
                key[fresh11 as usize] = a as libc::c_char;
                n -= 1;
              
            }
        } else {
            let mut current_block_34: u64;
            match a {
                10 => {
                    idx_lc += 1;

                    if idx_dot != 0 {
                        fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                        idx_dot = 0;
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
                        b"Incomplete first argument (premature LFD).\n\0" as *const u8
                            as *const libc::c_char,
                        b"\0" as *const u8 as *const libc::c_char,
                    );
                    idx_ec += 1;

                    return 0;
                }
                9 | 32 => {
                    if compress_blanks != 0 {
                        if i > 0
                            && key[(i - 1) as usize] as i32 != ' ' as i32
                            && key[(i - 1) as usize] as i32 != '\t' as i32
                        {
                            let fresh12 = i;
                            i += 1;
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
            if current_block_34 == 18279107430620107848 {
                let fresh13 = i;
                i += 1;
                key[fresh13 as usize] = a as libc::c_char;
            }
        }
        a = getc(idx_fp);
    }
    flush_to_eol();
    idx_lc += 1;
    idx_lc;
    if idx_dot != 0 {
        fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
        idx_dot = 0;
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
        1024,
    );
    idx_ec += 1;
    idx_ec;
    0
}
unsafe extern "C" fn scan_arg2() -> i32 {
    let mut i = 0;
    let mut a = 0;
    let mut hit_blank = 0;
    loop {
        a = getc(idx_fp);
        if !(a == ' ' as i32 || a == '\t' as i32) {
            break;
        }
    }
    while i < 16 {
        if a == idx_aclose as i32 {
            no[i as usize] = '\0' as i32 as libc::c_char;
            return 1;
        } else {
            match a {
                10 => {
                    idx_lc += 1;

                    if idx_dot != 0 {
                        fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                        idx_dot = 0;
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
                        b"Incomplete second argument (premature LFD).\n\0" as *const u8
                            as *const libc::c_char,
                        b"\0" as *const u8 as *const libc::c_char,
                    );
                    idx_ec += 1;

                    return 0;
                }
                9 | 32 => {
                    hit_blank = 1;
                }
                _ => {
                    if hit_blank != 0 {
                        flush_to_eol();
                        idx_lc += 1;

                        if idx_dot != 0 {
                            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                            idx_dot = 0;
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
                            b"Illegal space within numerals in second argument.\n\0" as *const u8
                                as *const libc::c_char,
                            b"\0" as *const u8 as *const libc::c_char,
                        );
                        idx_ec += 1;

                        return 0;
                    }
                    let fresh14 = i;
                    i += 1;
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
        idx_dot = 0;
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
        16,
    );
    idx_ec += 1;
    idx_ec;
    0
}
unsafe extern "C" fn search_quote(
    mut sort_key: *mut libc::c_char,
    mut actual_key: *mut libc::c_char,
) {
    let mut ptr = std::ptr::null_mut::<libc::c_char>();
    let mut sort = std::ptr::null_mut::<libc::c_char>();
    let mut char_found = 0;
    strcpy(actual_key, sort_key as *const libc::c_char);
    ptr = strchr(sort_key as *const libc::c_char, '"' as i32);
    while !ptr.is_null() {
        sort = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        match *ptr.offset(1) as i32 {
            97 | 65 => {
                sort = (if *(*__ctype_b_loc()).offset(*ptr.offset(1) as i32 as isize) as i32
                    & _ISupper as i32 as libc::c_ushort as i32
                    != 0
                {
                    b"Ae\0" as *const u8 as *const libc::c_char
                } else {
                    b"ae\0" as *const u8 as *const libc::c_char
                }) as *mut libc::c_char;
            }
            111 | 79 => {
                sort = (if *(*__ctype_b_loc()).offset(*ptr.offset(1) as i32 as isize) as i32
                    & _ISupper as i32 as libc::c_ushort as i32
                    != 0
                {
                    b"Oe\0" as *const u8 as *const libc::c_char
                } else {
                    b"oe\0" as *const u8 as *const libc::c_char
                }) as *mut libc::c_char;
            }
            117 | 85 => {
                sort = (if *(*__ctype_b_loc()).offset(*ptr.offset(1) as i32 as isize) as i32
                    & _ISupper as i32 as libc::c_ushort as i32
                    != 0
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
        if *sort.offset(0) as i32 != '\0' as i32 {
            char_found = 1;
            *ptr = *sort.offset(0);
            *ptr.offset(1) = *sort.offset(1);
        }
        ptr = strchr(ptr.offset(1), '"' as i32);
    }
    if char_found == 0 {
        *actual_key.offset(0) = '\0' as i32 as libc::c_char;
    }
}
