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
    static mut init_page: i32;
    static mut merge_page: i32;
    static mut even_odd: i32;
    static mut verbose: i32;
    static mut german_sort: i32;
    static mut idx_ropen: libc::c_char;
    static mut idx_rclose: libc::c_char;
    static mut preamble: [libc::c_char; 1024];
    static mut postamble: [libc::c_char; 1024];
    static mut setpage_open: [libc::c_char; 1024];
    static mut setpage_close: [libc::c_char; 1024];
    static mut group_skip: [libc::c_char; 1024];
    static mut headings_flag: i32;
    static mut heading_pre: [libc::c_char; 1024];
    static mut heading_suf: [libc::c_char; 1024];
    static mut symhead_pos: [libc::c_char; 1024];
    static mut symhead_neg: [libc::c_char; 1024];
    static mut numhead_pos: [libc::c_char; 1024];
    static mut numhead_neg: [libc::c_char; 1024];
    static mut prelen: i32;
    static mut postlen: i32;
    static mut skiplen: i32;
    static mut headprelen: i32;
    static mut headsuflen: i32;
    static mut setpagelen: i32;
    static mut item_r: [[libc::c_char; 1024]; 3];
    static mut item_u: [[libc::c_char; 1024]; 3];
    static mut item_x: [[libc::c_char; 1024]; 3];
    static mut ilen_r: [i32; 3];
    static mut ilen_u: [i32; 3];
    static mut ilen_x: [i32; 3];
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
    static mut linemax: i32;
    static mut indent_space: [libc::c_char; 1024];
    static mut indent_length: i32;
    static mut ind_fp: *mut FILE;
    static mut ilg_fp: *mut FILE;
    static mut ind_fn: *mut libc::c_char;
    static mut pageno: [libc::c_char; 0];
    static mut idx_key: *mut FIELD_PTR;
    static mut idx_dot: i32;
    static mut idx_gt: i32;
    static mut idx_dc: i32;
    fn strtoint(str: *mut libc::c_char) -> i32;
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
    pub group: i32,
    pub lpg: [libc::c_char; 16],
    pub npg: [i16; 10],
    pub count: i16,
    pub type_0: i16,
    pub encap: *mut libc::c_char,
    pub fn_0: *mut libc::c_char,
    pub lc: i32,
}
pub type FIELD_PTR = *mut KFIELD;
static mut curr: FIELD_PTR = 0 as *const KFIELD as FIELD_PTR;
static mut prev: FIELD_PTR = 0 as *const KFIELD as FIELD_PTR;
static mut begin: FIELD_PTR = 0 as *const KFIELD as FIELD_PTR;
static mut end: FIELD_PTR = 0 as *const KFIELD as FIELD_PTR;
static mut range_ptr: FIELD_PTR = 0 as *const KFIELD as *mut KFIELD;
static mut level: i32 = 0;
static mut prev_level: i32 = 0;
static mut encap: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut prev_encap: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut in_range: i32 = 0;
static mut encap_range: i32 = 0;
static mut buff: [libc::c_char; 2048] = [0; 2048];
static mut line: [libc::c_char; 2048] = [0; 2048];
static mut ind_lc: i32 = 0;
static mut ind_ec: i32 = 0;
static mut ind_indent: i32 = 0;
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
    idx_dc = 0;
    n = 0;
    while n < idx_gt {
        if (**idx_key.offset(n as isize)).type_0 as i32 != 9999 && make_entry(n) != 0 {
            idx_dot = 1;
            let fresh0 = idx_dc;
            idx_dc += 1;
            if fresh0 == 0 {
                if verbose != 0 {
                    fprintf(stderr(), b".\0" as *const u8 as *const libc::c_char);
                }
                fprintf(ilg_fp, b".\0" as *const u8 as *const libc::c_char);
            }
            if idx_dc == 1000 {
                idx_dc = 0;
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
            idx_dot = 0;
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
            idx_ropen as i32,
        );
        ind_ec += 1;
    }
    prev = curr;
    flush_line(1);
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
unsafe extern "C" fn make_entry(mut n: i32) -> i32 {
    let mut let_0 = 0;
    prev = curr;
    curr = *idx_key.offset(n as isize);
    if *(*curr).encap as i32 == idx_ropen as i32 || *(*curr).encap as i32 == idx_rclose as i32 {
        encap = &mut *((*curr).encap).offset(1) as *mut libc::c_char;
    } else {
        encap = (*curr).encap;
    }
    if n == 0 {
        level = 0;
        prev_level = level;
        let_0 = *(*curr).sf[0] as i32;
        put_header(let_0);
        make_item(b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    } else {
        prev_level = level;
        level = 0;
        while level < 3 {
            if strcmp((*curr).sf[level as usize], (*prev).sf[level as usize]) != 0
                || strcmp((*curr).af[level as usize], (*prev).af[level as usize]) != 0
            {
                break;
            }
            level += 1;
            level;
        }
        if level < 3 {
            new_entry();
        } else {
            old_entry();
        }
    }
    if *(*curr).encap as i32 == idx_ropen as i32 {
        if in_range != 0 {
            if idx_dot != 0 {
                fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                idx_dot = 0;
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
                idx_ropen as i32,
            );
            ind_ec += 1;
        } else {
            in_range = 1;
            range_ptr = curr;
        }
    } else if *(*curr).encap as i32 == idx_rclose as i32 {
        if in_range != 0 {
            in_range = 0;
            if strcmp(
                &mut *((*curr).encap).offset(1),
                b"\0" as *const u8 as *const libc::c_char,
            ) != 0
                && strcmp(prev_encap, &mut *((*curr).encap).offset(1)) != 0
            {
                if idx_dot != 0 {
                    fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                    idx_dot = 0;
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
                idx_dot = 0;
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
                idx_rclose as i32,
            );
            ind_ec += 1;
        }
    } else if *(*curr).encap as i32 != '\0' as i32
        && strcmp((*curr).encap, prev_encap) != 0
        && in_range != 0
    {
        if idx_dot != 0 {
            fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
            idx_dot = 0;
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
        if *(*curr).af[level as usize] as i32 == '\0' as i32 {
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
        if *(*curr).af[level as usize] as i32 == '\0' as i32 {
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
    while i < 3 && *(*curr).sf[i as usize] as i32 != '\0' as i32 {
        fputs(line.as_mut_ptr(), ind_fp);
        if *(*curr).af[i as usize] as i32 == '\0' as i32 {
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
    ind_indent = 0;
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
            idx_dot = 0;
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
            idx_ropen as i32,
        );
        ind_ec += 1;

        in_range = 0;
        curr = ptr;
    }
    flush_line(1);
    if (*curr).group != -2 && (*curr).group != (*prev).group && (*prev).group == -1
        || (*curr).group == -2 && {
            let_0 = if *(*__ctype_b_loc())
                .offset(*((*curr).sf[0]).offset(0) as libc::c_uchar as i32 as isize)
                as i32
                & _ISupper as i32 as libc::c_ushort as i32
                != 0
            {
                tolower(*((*curr).sf[0]).offset(0) as libc::c_uchar as i32) as libc::c_uchar as i32
            } else {
                *((*curr).sf[0]).offset(0) as libc::c_uchar as i32
            };
            let_0 as libc::c_uchar as i32
                != (if *(*__ctype_b_loc())
                    .offset(*((*prev).sf[0]).offset(0) as libc::c_uchar as i32 as isize)
                    as i32
                    & _ISupper as i32 as libc::c_ushort as i32
                    != 0
                {
                    tolower(*((*prev).sf[0]).offset(0) as libc::c_uchar as i32) as libc::c_uchar
                        as i32
                } else {
                    *((*prev).sf[0]).offset(0) as libc::c_uchar as i32
                })
        }
        || german_sort != 0 && (*curr).group != -2 && (*prev).group == -2
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
    if (*prev).type_0 as i32 == (*curr).type_0 as i32
        && diff != -1
        && (diff == 0 && !prev_encap.is_null() && strcmp(encap, prev_encap) == 0
            || merge_page != 0
                && diff == 1
                && !prev_encap.is_null()
                && strcmp(encap, prev_encap) == 0
            || in_range != 0)
    {
        end = curr;
        if in_range != 0
            && *(*curr).encap as i32 != '\0' as i32
            && *(*curr).encap as i32 != idx_rclose as i32
            && strcmp((*curr).encap, prev_encap) != 0
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
            wrap_line(0);
        }
        if in_range != 0 {
            encap_range = 1;
        }
    } else {
        flush_line(0);
        if diff == 0 && (*prev).type_0 as i32 == (*curr).type_0 as i32 {
            if idx_dot != 0 {
                fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                idx_dot = 0;
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
        } else if in_range != 0 && (*prev).type_0 as i32 != (*curr).type_0 as i32 {
            if idx_dot != 0 {
                fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                idx_dot = 0;
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
        } else if in_range != 0 && diff == -1 {
            if idx_dot != 0 {
                fprintf(ilg_fp, b"\n\0" as *const u8 as *const libc::c_char);
                idx_dot = 0;
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
unsafe extern "C" fn page_diff(mut a: FIELD_PTR, mut b: FIELD_PTR) -> i32 {
    let mut i = 0;
    if (*a).count as i32 != (*b).count as i32 {
        return -1;
    }
    i = 0_i16;
    while (i as i32) < (*a).count as i32 - 1 {
        if (*a).npg[i as usize] as i32 != (*b).npg[i as usize] as i32 {
            return -1;
        }
        i += 1;
    }
    (*b).npg[((*b).count as i32 - 1) as usize] as i32
        - (*a).npg[((*a).count as i32 - 1) as usize] as i32
}
unsafe extern "C" fn put_header(mut let_0: i32) {
    if headings_flag != 0 {
        fputs(heading_pre.as_mut_ptr(), ind_fp);
        ind_lc += headprelen;
        match (*curr).group {
            -1 => {
                if headings_flag > 0 {
                    fputs(symhead_pos.as_mut_ptr(), ind_fp);
                } else {
                    fputs(symhead_neg.as_mut_ptr(), ind_fp);
                }
            }
            -2 => {
                if headings_flag > 0 {
                    let_0 = if *(*__ctype_b_loc()).offset(let_0 as libc::c_uchar as i32 as isize)
                        as i32
                        & _ISupper as i32 as libc::c_ushort as i32
                        != 0
                    {
                        let_0 as libc::c_uchar as i32
                    } else {
                        toupper(let_0 as libc::c_uchar as i32) as libc::c_uchar as i32
                    };
                    fputc(let_0, ind_fp);
                } else {
                    let_0 = if *(*__ctype_b_loc()).offset(let_0 as libc::c_uchar as i32 as isize)
                        as i32
                        & _ISupper as i32 as libc::c_ushort as i32
                        != 0
                    {
                        tolower(let_0 as libc::c_uchar as i32) as libc::c_uchar as i32
                    } else {
                        let_0 as libc::c_uchar as i32
                    };
                    fputc(let_0, ind_fp);
                }
            }
            _ => {
                if headings_flag > 0 {
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
unsafe extern "C" fn flush_line(mut print: i32) {
    let mut tmp = [0; 2048];
    if page_diff(begin, end) != 0 {
        if encap_range != 0
            || page_diff(begin, prev)
                > (if *suffix_2p.as_mut_ptr() as i32 != 0 {
                    0
                } else {
                    1
                })
        {
            let mut diff = page_diff(begin, end);
            if diff == 1 && *suffix_2p.as_mut_ptr() as i32 != 0 {
                sprintf(
                    buff.as_mut_ptr(),
                    b"%s%s\0" as *const u8 as *const libc::c_char,
                    ((*begin).lpg).as_mut_ptr(),
                    suffix_2p.as_mut_ptr(),
                );
            } else if diff == 2 && *suffix_3p.as_mut_ptr() as i32 != 0 {
                sprintf(
                    buff.as_mut_ptr(),
                    b"%s%s\0" as *const u8 as *const libc::c_char,
                    ((*begin).lpg).as_mut_ptr(),
                    suffix_3p.as_mut_ptr(),
                );
            } else if diff >= 2 && *suffix_mp.as_mut_ptr() as i32 != 0 {
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
            encap_range = 0;
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
        encap_range = 0;
        strcpy(buff.as_mut_ptr(), ((*begin).lpg).as_mut_ptr());
    }
    if *prev_encap as i32 != '\0' as i32 {
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
unsafe extern "C" fn wrap_line(mut print: i32) {
    let mut len = 0;
    len = (strlen(line.as_mut_ptr()))
        .wrapping_add(strlen(buff.as_mut_ptr()))
        .wrapping_add(ind_indent.try_into().unwrap()) as i32;
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
    let mut i = 0;
    let mut j = 0;
    let mut page = 0;
    if even_odd >= 0 {
        loop {
            let fresh1 = i;
            i += 1;
            if *pageno.as_mut_ptr().offset(fresh1 as isize) as i32 == '\0' as i32 {
                break;
            }
        }
        i -= 1;
        j = i;
        loop {
            i -= 1;
            if !(*(*__ctype_b_loc()).offset(*pageno.as_mut_ptr().offset(i as isize) as i32 as isize)
                as i32
                & _ISdigit as i32 as libc::c_ushort as i32
                != 0
                && i > 0)
            {
                break;
            }
        }
        if *(*__ctype_b_loc()).offset(*pageno.as_mut_ptr().offset(i as isize) as i32 as isize)
            as i32
            & _ISdigit as i32 as libc::c_ushort as i32
            == 0
        {
            i += 1;
        }
        page = strtoint(&mut *pageno.as_mut_ptr().offset(i as isize)) + 1;
        if even_odd == 1 && page % 2 == 0 || even_odd == 2 && page % 2 == 1 {
            page += 1;
            page;
        }
        *pageno.as_mut_ptr().offset((j + 1) as isize) = '\0' as i32 as libc::c_char;
        while page >= 10 {
            let fresh2 = j;
            j -= 1;
            *pageno.as_mut_ptr().offset(fresh2 as isize) = (page % 10 + 48) as libc::c_char;
            page /= 10;
        }
        *pageno.as_mut_ptr().offset(j as isize) = (page + 48) as libc::c_char;
        if i < j {
            while *pageno.as_mut_ptr().offset(j as isize) as i32 != '\0' as i32 {
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
