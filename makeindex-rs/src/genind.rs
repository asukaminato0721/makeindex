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
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    static mut init_page: libc::c_int;
    static mut merge_page: libc::c_int;
    static mut even_odd: libc::c_int;
    static mut verbose: libc::c_int;
    static mut german_sort: libc::c_int;
    static mut idx_ropen: libc::c_char;
    static mut idx_rclose: libc::c_char;
    static mut preamble: [libc::c_char; 1024];
    static mut postamble: [libc::c_char; 1024];
    static mut setpage_open: [libc::c_char; 1024];
    static mut setpage_close: [libc::c_char; 1024];
    static mut group_skip: [libc::c_char; 1024];
    static mut headings_flag: libc::c_int;
    static mut heading_pre: [libc::c_char; 1024];
    static mut heading_suf: [libc::c_char; 1024];
    static mut symhead_pos: [libc::c_char; 1024];
    static mut symhead_neg: [libc::c_char; 1024];
    static mut numhead_pos: [libc::c_char; 1024];
    static mut numhead_neg: [libc::c_char; 1024];
    static mut prelen: libc::c_int;
    static mut postlen: libc::c_int;
    static mut skiplen: libc::c_int;
    static mut headprelen: libc::c_int;
    static mut headsuflen: libc::c_int;
    static mut setpagelen: libc::c_int;
    static mut item_r: [[libc::c_char; 1024]; 3];
    static mut item_u: [[libc::c_char; 1024]; 3];
    static mut item_x: [[libc::c_char; 1024]; 3];
    static mut ilen_r: [libc::c_int; 3];
    static mut ilen_u: [libc::c_int; 3];
    static mut ilen_x: [libc::c_int; 3];
    static mut delim_p: [[libc::c_char; 1024]; 3];
    static mut delim_n: [libc::c_char; 1024];
    static mut delim_r: [libc::c_char; 1024];
    static mut delim_t: [libc::c_char; 1024];
    static mut suffix_2p: [libc::c_char; 1024];
    static mut suffix_3p: [libc::c_char; 1024];
    static mut suffix_mp: [libc::c_char; 1024];
    static mut encap_p: [libc::c_char; 1024];
    static mut encap_i: [libc::c_char; 1024];
    static mut encap_s: [libc::c_char; 1024];
    static mut linemax: libc::c_int;
    static mut indent_space: [libc::c_char; 1024];
    static mut indent_length: libc::c_int;
    static mut ind_fp: *mut FILE;
    static mut ilg_fp: *mut FILE;
    static mut ind_fn: *mut libc::c_char;
    static mut pageno: [libc::c_char; 0];
    static mut idx_key: *mut FIELD_PTR;
    static mut idx_dot: libc::c_int;
    static mut idx_gt: libc::c_int;
    static mut idx_dc: libc::c_int;
    fn strtoint(str: *mut libc::c_char) -> libc::c_int;
}

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
static mut curr: FIELD_PTR = 0 as *const KFIELD as FIELD_PTR;
static mut prev: FIELD_PTR = 0 as *const KFIELD as FIELD_PTR;
static mut begin: FIELD_PTR = 0 as *const KFIELD as FIELD_PTR;
static mut end: FIELD_PTR = 0 as *const KFIELD as FIELD_PTR;
static mut range_ptr: FIELD_PTR = 0 as *const KFIELD as *mut KFIELD;
static mut level: libc::c_int = 0 as libc::c_int;
static mut prev_level: libc::c_int = 0 as libc::c_int;
static mut encap: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut prev_encap: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut in_range: libc::c_int = 0 as libc::c_int;
static mut encap_range: libc::c_int = 0 as libc::c_int;
static mut buff: [libc::c_char; 2048] = [0; 2048];
static mut line: [libc::c_char; 2048] = [0; 2048];
static mut ind_lc: libc::c_int = 0 as libc::c_int;
static mut ind_ec: libc::c_int = 0 as libc::c_int;
static mut ind_indent: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn gen_ind() {
    let mut n = 0;
    let mut tmp_lc = 0;
    if verbose != 0 {
        fprintf(
            stderr(),
            b"Generating output file %s...\0" as *const u8 as *const libc::c_char,
            ind_fn,
        );
    }
    fprintf(
        ilg_fp,
        b"Generating output file %s...\0" as *const u8 as *const libc::c_char,
        ind_fn,
    );
    fputs(preamble.as_mut_ptr(), ind_fp);
    ind_lc += prelen;
    if init_page != 0 {
        insert_page();
    }
    idx_dc = 0 as libc::c_int;
    n = 0 as libc::c_int;
    while n < idx_gt {
        if (**idx_key.offset(n as isize)).type_0 as libc::c_int != 9999 as libc::c_int
            && make_entry(n) != 0
        {
            idx_dot = 1;
            let fresh0 = idx_dc;
            idx_dc += 1;
            if fresh0 == 0 as libc::c_int {
                if verbose != 0 {
                    fprintf(stderr(), b".\0" as *const u8 as *const libc::c_char);
                }
                fprintf(ilg_fp, b".\0" as *const u8 as *const libc::c_char);
            }
            if idx_dc == 1000 as libc::c_int {
                idx_dc = 0 as libc::c_int;
            }
        }
        n += 1;
        n;
    }
    tmp_lc = ind_lc;
    if in_range != 0 {
        curr = range_ptr;
        if idx_dot != 0 {
            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
            idx_dot = 0 as libc::c_int;
        }
        fprintf(
            ilg_fp,
            b"## Warning (input = %s, line = %d; output = %s, line = %d):\n   -- \0" as *const u8
                as *const libc::c_char,
            (*curr).fn_0,
            (*curr).lc,
            ind_fn,
            ind_lc + 1,
        );
        fprintf(
            ilg_fp,
            b"Unmatched range opening operator %c.\n\0" as *const u8 as *const libc::c_char,
            idx_ropen as libc::c_int,
        );
        ind_ec += 1;
    }
    prev = curr;
    flush_line(1 as libc::c_int);
    fputs(delim_t.as_mut_ptr(), ind_fp);
    fputs(postamble.as_mut_ptr(), ind_fp);
    tmp_lc = ind_lc + postlen;
    if ind_ec == 1 {
        if verbose != 0 {
            fprintf(
                stderr(),
                b"done (%d %s, %d %s).\n\0" as *const u8 as *const libc::c_char,
                tmp_lc,
                b"lines written\0" as *const u8 as *const libc::c_char,
                ind_ec,
                b"warning\0" as *const u8 as *const libc::c_char,
            );
        }
        fprintf(
            ilg_fp,
            b"done (%d %s, %d %s).\n\0" as *const u8 as *const libc::c_char,
            tmp_lc,
            b"lines written\0" as *const u8 as *const libc::c_char,
            ind_ec,
            b"warning\0" as *const u8 as *const libc::c_char,
        );
    } else {
        if verbose != 0 {
            fprintf(
                stderr(),
                b"done (%d %s, %d %s).\n\0" as *const u8 as *const libc::c_char,
                tmp_lc,
                b"lines written\0" as *const u8 as *const libc::c_char,
                ind_ec,
                b"warnings\0" as *const u8 as *const libc::c_char,
            );
        }
        fprintf(
            ilg_fp,
            b"done (%d %s, %d %s).\n\0" as *const u8 as *const libc::c_char,
            tmp_lc,
            b"lines written\0" as *const u8 as *const libc::c_char,
            ind_ec,
            b"warnings\0" as *const u8 as *const libc::c_char,
        );
    };
}
unsafe extern "C" fn make_entry(mut n: libc::c_int) -> libc::c_int {
    let mut let_0 = 0;
    prev = curr;
    curr = *idx_key.offset(n as isize);
    if *(*curr).encap as libc::c_int == idx_ropen as libc::c_int
        || *(*curr).encap as libc::c_int == idx_rclose as libc::c_int
    {
        encap = &mut *((*curr).encap).offset(1) as *mut libc::c_char;
    } else {
        encap = (*curr).encap;
    }
    if n == 0 as libc::c_int {
        level = 0 as libc::c_int;
        prev_level = level;
        let_0 = *(*curr).sf[0 as libc::c_int as usize] as libc::c_int;
        put_header(let_0);
        make_item(b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    } else {
        prev_level = level;
        level = 0 as libc::c_int;
        while level < 3 as libc::c_int {
            if strcmp((*curr).sf[level as usize], (*prev).sf[level as usize]) != 0 as libc::c_int
                || strcmp((*curr).af[level as usize], (*prev).af[level as usize])
                    != 0 as libc::c_int
            {
                break;
            }
            level += 1;
            level;
        }
        if level < 3 as libc::c_int {
            new_entry();
        } else {
            old_entry();
        }
    }
    if *(*curr).encap as libc::c_int == idx_ropen as libc::c_int {
        if in_range != 0 {
            if idx_dot != 0 {
                fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                idx_dot = 0 as libc::c_int;
            }
            fprintf(
                ilg_fp,
                b"## Warning (input = %s, line = %d; output = %s, line = %d):\n   -- \0"
                    as *const u8 as *const libc::c_char,
                (*curr).fn_0,
                (*curr).lc,
                ind_fn,
                ind_lc + 1,
            );
            fprintf(
                ilg_fp,
                b"Extra range opening operator %c.\n\0" as *const u8 as *const libc::c_char,
                idx_ropen as libc::c_int,
            );
            ind_ec += 1;
        } else {
            in_range = 1;
            range_ptr = curr;
        }
    } else if *(*curr).encap as libc::c_int == idx_rclose as libc::c_int {
        if in_range != 0 {
            in_range = 0 as libc::c_int;
            if strcmp(
                &mut *((*curr).encap).offset(1),
                b"\0" as *const u8 as *const libc::c_char,
            ) != 0 as libc::c_int
                && strcmp(prev_encap, &mut *((*curr).encap).offset(1)) != 0 as libc::c_int
            {
                if idx_dot != 0 {
                    fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                    idx_dot = 0 as libc::c_int;
                }
                fprintf(
                    ilg_fp,
                    b"## Warning (input = %s, line = %d; output = %s, line = %d):\n   -- \0"
                        as *const u8 as *const libc::c_char,
                    (*curr).fn_0,
                    (*curr).lc,
                    ind_fn,
                    ind_lc + 1,
                );
                fprintf(
                    ilg_fp,
                    b"Range closing operator has an inconsistent encapsulator %s.\n\0" as *const u8
                        as *const libc::c_char,
                    &mut *((*curr).encap).offset(1) as *mut libc::c_char,
                );
                ind_ec += 1;
            }
        } else {
            if idx_dot != 0 {
                fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                idx_dot = 0 as libc::c_int;
            }
            fprintf(
                ilg_fp,
                b"## Warning (input = %s, line = %d; output = %s, line = %d):\n   -- \0"
                    as *const u8 as *const libc::c_char,
                (*curr).fn_0,
                (*curr).lc,
                ind_fn,
                ind_lc + 1,
            );
            fprintf(
                ilg_fp,
                b"Unmatched range closing operator %c.\n\0" as *const u8 as *const libc::c_char,
                idx_rclose as libc::c_int,
            );
            ind_ec += 1;
        }
    } else if *(*curr).encap as libc::c_int != '\0' as i32
        && strcmp((*curr).encap, prev_encap) != 0 as libc::c_int
        && in_range != 0
    {
        if idx_dot != 0 {
            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
            idx_dot = 0 as libc::c_int;
        }
        fprintf(
            ilg_fp,
            b"## Warning (input = %s, line = %d; output = %s, line = %d):\n   -- \0" as *const u8
                as *const libc::c_char,
            (*curr).fn_0,
            (*curr).lc,
            ind_fn,
            ind_lc + 1,
        );
        fprintf(
            ilg_fp,
            b"Inconsistent page encapsulator %s within range.\n\0" as *const u8
                as *const libc::c_char,
            (*curr).encap,
        );
        ind_ec += 1;
    }
    1
}
unsafe extern "C" fn make_item(mut term: *mut libc::c_char) {
    let mut i = 0;
    if level > prev_level {
        if *(*curr).af[level as usize] as libc::c_int == '\0' as i32 {
            sprintf(
                line.as_mut_ptr(),
                b"%s%s%s\0" as *const u8 as *const libc::c_char,
                term,
                (item_u[level as usize]).as_mut_ptr(),
                (*curr).sf[level as usize],
            );
        } else {
            sprintf(
                line.as_mut_ptr(),
                b"%s%s%s\0" as *const u8 as *const libc::c_char,
                term,
                (item_u[level as usize]).as_mut_ptr(),
                (*curr).af[level as usize],
            );
        }
        ind_lc += ilen_u[level as usize];
    } else {
        if *(*curr).af[level as usize] as libc::c_int == '\0' as i32 {
            sprintf(
                line.as_mut_ptr(),
                b"%s%s%s\0" as *const u8 as *const libc::c_char,
                term,
                (item_r[level as usize]).as_mut_ptr(),
                (*curr).sf[level as usize],
            );
        } else {
            sprintf(
                line.as_mut_ptr(),
                b"%s%s%s\0" as *const u8 as *const libc::c_char,
                term,
                (item_r[level as usize]).as_mut_ptr(),
                (*curr).af[level as usize],
            );
        }
        ind_lc += ilen_r[level as usize];
    }
    i = level + 1;
    while i < 3 as libc::c_int && *(*curr).sf[i as usize] as libc::c_int != '\0' as i32 {
        fputs(line.as_mut_ptr(), ind_fp);
        if *(*curr).af[i as usize] as libc::c_int == '\0' as i32 {
            sprintf(
                line.as_mut_ptr(),
                b"%s%s\0" as *const u8 as *const libc::c_char,
                (item_x[i as usize]).as_mut_ptr(),
                (*curr).sf[i as usize],
            );
        } else {
            sprintf(
                line.as_mut_ptr(),
                b"%s%s\0" as *const u8 as *const libc::c_char,
                (item_x[i as usize]).as_mut_ptr(),
                (*curr).af[i as usize],
            );
        }
        ind_lc += ilen_x[i as usize];
        level = i;
        i += 1;
    }
    ind_indent = 0 as libc::c_int;
    strcat(line.as_mut_ptr(), (delim_p[level as usize]).as_mut_ptr());
    end = curr;
    begin = end;
    prev_encap = encap;
}
unsafe extern "C" fn new_entry() {
    let mut let_0 = 0;
    let mut ptr = std::ptr::null_mut::<KFIELD>();
    if in_range != 0 {
        ptr = curr;
        curr = range_ptr;
        if idx_dot != 0 {
            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
            idx_dot = 0 as libc::c_int;
        }
        fprintf(
            ilg_fp,
            b"## Warning (input = %s, line = %d; output = %s, line = %d):\n   -- \0" as *const u8
                as *const libc::c_char,
            (*curr).fn_0,
            (*curr).lc,
            ind_fn,
            ind_lc + 1,
        );
        fprintf(
            ilg_fp,
            b"Unmatched range opening operator %c.\n\0" as *const u8 as *const libc::c_char,
            idx_ropen as libc::c_int,
        );
        ind_ec += 1;

        in_range = 0 as libc::c_int;
        curr = ptr;
    }
    flush_line(1 as libc::c_int);
    if (*curr).group != -(2 as libc::c_int)
        && (*curr).group != (*prev).group
        && (*prev).group == -(1 as libc::c_int)
        || (*curr).group == -(2 as libc::c_int) && {
            let_0 = if *(*__ctype_b_loc()).offset(
                *((*curr).sf[0 as libc::c_int as usize]).offset(0 as libc::c_int as isize)
                    as libc::c_uchar as libc::c_int as isize,
            ) as libc::c_int
                & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                tolower(
                    *((*curr).sf[0 as libc::c_int as usize]).offset(0 as libc::c_int as isize)
                        as libc::c_uchar as libc::c_int,
                ) as libc::c_uchar as libc::c_int
            } else {
                *((*curr).sf[0 as libc::c_int as usize]).offset(0 as libc::c_int as isize)
                    as libc::c_uchar as libc::c_int
            };
            let_0 as libc::c_uchar as libc::c_int
                != (if *(*__ctype_b_loc()).offset(
                    *((*prev).sf[0 as libc::c_int as usize]).offset(0 as libc::c_int as isize)
                        as libc::c_uchar as libc::c_int as isize,
                ) as libc::c_int
                    & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
                    != 0
                {
                    tolower(
                        *((*prev).sf[0 as libc::c_int as usize]).offset(0 as libc::c_int as isize)
                            as libc::c_uchar as libc::c_int,
                    ) as libc::c_uchar as libc::c_int
                } else {
                    *((*prev).sf[0 as libc::c_int as usize]).offset(0 as libc::c_int as isize)
                        as libc::c_uchar as libc::c_int
                })
        }
        || german_sort != 0
            && (*curr).group != -(2 as libc::c_int)
            && (*prev).group == -(2 as libc::c_int)
    {
        fputs(delim_t.as_mut_ptr(), ind_fp);
        fputs(group_skip.as_mut_ptr(), ind_fp);
        ind_lc += skiplen;
        put_header(let_0);
        make_item(b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    } else {
        make_item(delim_t.as_mut_ptr());
    };
}
unsafe extern "C" fn old_entry() {
    let mut diff = 0;
    diff = page_diff(end, curr);
    if (*prev).type_0 as libc::c_int == (*curr).type_0 as libc::c_int
        && diff != -(1 as libc::c_int)
        && (diff == 0 as libc::c_int
            && !prev_encap.is_null()
            && strcmp(encap, prev_encap) == 0 as libc::c_int
            || merge_page != 0
                && diff == 1
                && !prev_encap.is_null()
                && strcmp(encap, prev_encap) == 0 as libc::c_int
            || in_range != 0)
    {
        end = curr;
        if in_range != 0
            && *(*curr).encap as libc::c_int != '\0' as i32
            && *(*curr).encap as libc::c_int != idx_rclose as libc::c_int
            && strcmp((*curr).encap, prev_encap) != 0 as libc::c_int
        {
            sprintf(
                buff.as_mut_ptr(),
                b"%s%s%s%s%s\0" as *const u8 as *const libc::c_char,
                encap_p.as_mut_ptr(),
                (*curr).encap,
                encap_i.as_mut_ptr(),
                ((*curr).lpg).as_mut_ptr(),
                encap_s.as_mut_ptr(),
            );
            wrap_line(0 as libc::c_int);
        }
        if in_range != 0 {
            encap_range = 1;
        }
    } else {
        flush_line(0 as libc::c_int);
        if diff == 0 as libc::c_int
            && (*prev).type_0 as libc::c_int == (*curr).type_0 as libc::c_int
        {
            if idx_dot != 0 {
                fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                idx_dot = 0 as libc::c_int;
            }
            fprintf(
                ilg_fp,
                b"## Warning (input = %s, line = %d; output = %s, line = %d):\n   -- \0"
                    as *const u8 as *const libc::c_char,
                (*curr).fn_0,
                (*curr).lc,
                ind_fn,
                ind_lc + 1,
            );
            fprintf(
                ilg_fp,
                b"Conflicting entries: multiple encaps for the same page under same key.\n\0"
                    as *const u8 as *const libc::c_char,
                b"\0" as *const u8 as *const libc::c_char,
            );
            ind_ec += 1;
        } else if in_range != 0 && (*prev).type_0 as libc::c_int != (*curr).type_0 as libc::c_int {
            if idx_dot != 0 {
                fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                idx_dot = 0 as libc::c_int;
            }
            fprintf(
                ilg_fp,
                b"## Warning (input = %s, line = %d; output = %s, line = %d):\n   -- \0"
                    as *const u8 as *const libc::c_char,
                (*curr).fn_0,
                (*curr).lc,
                ind_fn,
                ind_lc + 1,
            );
            fprintf(
                ilg_fp,
                b"Illegal range formation: starting & ending pages are of different types.\n\0"
                    as *const u8 as *const libc::c_char,
                b"\0" as *const u8 as *const libc::c_char,
            );
            ind_ec += 1;
        } else if in_range != 0 && diff == -(1 as libc::c_int) {
            if idx_dot != 0 {
                fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                idx_dot = 0 as libc::c_int;
            }
            fprintf(
                ilg_fp,
                b"## Warning (input = %s, line = %d; output = %s, line = %d):\n   -- \0"
                    as *const u8 as *const libc::c_char,
                (*curr).fn_0,
                (*curr).lc,
                ind_fn,
                ind_lc + 1,
            );
            fprintf(
                ilg_fp,
                b"Illegal range formation: starting & ending pages cross chap/sec breaks.\n\0"
                    as *const u8 as *const libc::c_char,
                b"\0" as *const u8 as *const libc::c_char,
            );
            ind_ec += 1;
        }
        end = curr;
        begin = end;
        prev_encap = encap;
    };
}
unsafe extern "C" fn page_diff(mut a: FIELD_PTR, mut b: FIELD_PTR) -> libc::c_int {
    let mut i = 0;
    if (*a).count as libc::c_int != (*b).count as libc::c_int {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_short;
    while (i as libc::c_int) < (*a).count as libc::c_int - 1 {
        if (*a).npg[i as usize] as libc::c_int != (*b).npg[i as usize] as libc::c_int {
            return -(1 as libc::c_int);
        }
        i += 1;
    }
    (*b).npg[((*b).count as libc::c_int - 1) as usize] as libc::c_int
        - (*a).npg[((*a).count as libc::c_int - 1) as usize] as libc::c_int
}
unsafe extern "C" fn put_header(mut let_0: libc::c_int) {
    if headings_flag != 0 {
        fputs(heading_pre.as_mut_ptr(), ind_fp);
        ind_lc += headprelen;
        match (*curr).group {
            -1 => {
                if headings_flag > 0 as libc::c_int {
                    fputs(symhead_pos.as_mut_ptr(), ind_fp);
                } else {
                    fputs(symhead_neg.as_mut_ptr(), ind_fp);
                }
            }
            -2 => {
                if headings_flag > 0 as libc::c_int {
                    let_0 = if *(*__ctype_b_loc())
                        .offset(let_0 as libc::c_uchar as libc::c_int as isize)
                        as libc::c_int
                        & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
                        != 0
                    {
                        let_0 as libc::c_uchar as libc::c_int
                    } else {
                        toupper(let_0 as libc::c_uchar as libc::c_int) as libc::c_uchar
                            as libc::c_int
                    };
                    fputc(let_0, ind_fp);
                } else {
                    let_0 = if *(*__ctype_b_loc())
                        .offset(let_0 as libc::c_uchar as libc::c_int as isize)
                        as libc::c_int
                        & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
                        != 0
                    {
                        tolower(let_0 as libc::c_uchar as libc::c_int) as libc::c_uchar
                            as libc::c_int
                    } else {
                        let_0 as libc::c_uchar as libc::c_int
                    };
                    fputc(let_0, ind_fp);
                }
            }
            _ => {
                if headings_flag > 0 as libc::c_int {
                    fputs(numhead_pos.as_mut_ptr(), ind_fp);
                } else {
                    fputs(numhead_neg.as_mut_ptr(), ind_fp);
                }
            }
        }
        fputs(heading_suf.as_mut_ptr(), ind_fp);
        ind_lc += headsuflen;
    }
}
unsafe extern "C" fn flush_line(mut print: libc::c_int) {
    let mut tmp = [0; 2048];
    if page_diff(begin, end) != 0 as libc::c_int {
        if encap_range != 0
            || page_diff(begin, prev)
                > (if *suffix_2p.as_mut_ptr() as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    1
                })
        {
            let mut diff = page_diff(begin, end);
            if diff == 1 && *suffix_2p.as_mut_ptr() as libc::c_int != 0 {
                sprintf(
                    buff.as_mut_ptr(),
                    b"%s%s\0" as *const u8 as *const libc::c_char,
                    ((*begin).lpg).as_mut_ptr(),
                    suffix_2p.as_mut_ptr(),
                );
            } else if diff == 2 as libc::c_int && *suffix_3p.as_mut_ptr() as libc::c_int != 0 {
                sprintf(
                    buff.as_mut_ptr(),
                    b"%s%s\0" as *const u8 as *const libc::c_char,
                    ((*begin).lpg).as_mut_ptr(),
                    suffix_3p.as_mut_ptr(),
                );
            } else if diff >= 2 as libc::c_int && *suffix_mp.as_mut_ptr() as libc::c_int != 0 {
                sprintf(
                    buff.as_mut_ptr(),
                    b"%s%s\0" as *const u8 as *const libc::c_char,
                    ((*begin).lpg).as_mut_ptr(),
                    suffix_mp.as_mut_ptr(),
                );
            } else {
                sprintf(
                    buff.as_mut_ptr(),
                    b"%s%s%s\0" as *const u8 as *const libc::c_char,
                    ((*begin).lpg).as_mut_ptr(),
                    delim_r.as_mut_ptr(),
                    ((*end).lpg).as_mut_ptr(),
                );
            }
            encap_range = 0 as libc::c_int;
        } else {
            sprintf(
                buff.as_mut_ptr(),
                b"%s%s%s\0" as *const u8 as *const libc::c_char,
                ((*begin).lpg).as_mut_ptr(),
                delim_n.as_mut_ptr(),
                ((*end).lpg).as_mut_ptr(),
            );
        }
    } else {
        encap_range = 0 as libc::c_int;
        strcpy(buff.as_mut_ptr(), ((*begin).lpg).as_mut_ptr());
    }
    if *prev_encap as libc::c_int != '\0' as i32 {
        strcpy(tmp.as_mut_ptr(), buff.as_mut_ptr());
        sprintf(
            buff.as_mut_ptr(),
            b"%s%s%s%s%s\0" as *const u8 as *const libc::c_char,
            encap_p.as_mut_ptr(),
            prev_encap,
            encap_i.as_mut_ptr(),
            tmp.as_mut_ptr(),
            encap_s.as_mut_ptr(),
        );
    }
    wrap_line(print);
}
unsafe extern "C" fn wrap_line(mut print: libc::c_int) {
    let mut len = 0;
    len = (strlen(line.as_mut_ptr()))
        .wrapping_add(strlen(buff.as_mut_ptr()))
        .wrapping_add(ind_indent.try_into().unwrap()) as libc::c_int;
    if print != 0 {
        if len > linemax {
            fputs(line.as_mut_ptr(), ind_fp);
            fputc('\n' as i32, ind_fp);
            ind_lc += 1;
            ind_lc;
            fputs(indent_space.as_mut_ptr(), ind_fp);
            ind_indent = indent_length;
        } else {
            fputs(line.as_mut_ptr(), ind_fp);
        }
        fputs(buff.as_mut_ptr(), ind_fp);
    } else if len > linemax {
        fputs(line.as_mut_ptr(), ind_fp);
        fputc('\n' as i32, ind_fp);
        ind_lc += 1;
        ind_lc;
        sprintf(
            line.as_mut_ptr(),
            b"%s%s%s\0" as *const u8 as *const libc::c_char,
            indent_space.as_mut_ptr(),
            buff.as_mut_ptr(),
            delim_n.as_mut_ptr(),
        );
        ind_indent = indent_length;
    } else {
        strcat(buff.as_mut_ptr(), delim_n.as_mut_ptr());
        strcat(line.as_mut_ptr(), buff.as_mut_ptr());
    };
}
unsafe extern "C" fn insert_page() {
    let mut i = 0 as libc::c_int;
    let mut j = 0 as libc::c_int;
    let mut page = 0 as libc::c_int;
    if even_odd >= 0 as libc::c_int {
        loop {
            let fresh1 = i;
            i += 1;
            if *pageno.as_mut_ptr().offset(fresh1 as isize) as libc::c_int == '\0' as i32 {
                break;
            }
        }
        i -= 1;
        j = i;
        loop {
            i -= 1;
            if !(*(*__ctype_b_loc())
                .offset(*pageno.as_mut_ptr().offset(i as isize) as libc::c_int as isize)
                as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                != 0
                && i > 0 as libc::c_int)
            {
                break;
            }
        }
        if *(*__ctype_b_loc())
            .offset(*pageno.as_mut_ptr().offset(i as isize) as libc::c_int as isize)
            as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        {
            i += 1;
        }
        page = strtoint(&mut *pageno.as_mut_ptr().offset(i as isize)) + 1;
        if even_odd == 1 && page % 2 as libc::c_int == 0 as libc::c_int
            || even_odd == 2 as libc::c_int && page % 2 as libc::c_int == 1
        {
            page += 1;
            page;
        }
        *pageno.as_mut_ptr().offset((j + 1) as isize) = '\0' as i32 as libc::c_char;
        while page >= 10 as libc::c_int {
            let fresh2 = j;
            j -= 1;
            *pageno.as_mut_ptr().offset(fresh2 as isize) =
                (page % 10 as libc::c_int + 48 as libc::c_int) as libc::c_char;
            page /= 10 as libc::c_int;
        }
        *pageno.as_mut_ptr().offset(j as isize) = (page + 48 as libc::c_int) as libc::c_char;
        if i < j {
            while *pageno.as_mut_ptr().offset(j as isize) as libc::c_int != '\0' as i32 {
                let fresh3 = j;
                j += 1;
                let fresh4 = i;
                i += 1;
                *pageno.as_mut_ptr().offset(fresh4 as isize) =
                    *pageno.as_mut_ptr().offset(fresh3 as isize);
            }
            *pageno.as_mut_ptr().offset(i as isize) = '\0' as i32 as libc::c_char;
        }
    }
    fputs(setpage_open.as_mut_ptr(), ind_fp);
    fputs(pageno.as_mut_ptr(), ind_fp);
    fputs(setpage_close.as_mut_ptr(), ind_fp);
    ind_lc += setpagelen;
}
