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
    static mut idx_quote: libc::c_char;
    static mut head: NODE_PTR;
    fn gen_ind();
    fn scan_idx();
    fn scan_sty();
    fn sort_idx();
}
pub type _IO_lock_t = ();
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
pub type NODE_PTR = *mut KNODE;
#[no_mangle]
pub static mut letter_ordering: libc::c_int = 0;
#[no_mangle]
pub static mut compress_blanks: libc::c_int = 0;
#[no_mangle]
pub static mut merge_page: libc::c_int = 1;
#[no_mangle]
pub static mut init_page: libc::c_int = 0;
#[no_mangle]
pub static mut even_odd: libc::c_int = -(1);
#[no_mangle]
pub static mut verbose: bool = true;
#[no_mangle]
pub static mut german_sort: libc::c_int = 0;
#[no_mangle]
pub static mut fn_no: libc::c_int = -(1);
#[no_mangle]
pub static mut idx_dot: libc::c_int = 1;
#[no_mangle]
pub static mut idx_tt: libc::c_int = 0;
#[no_mangle]
pub static mut idx_et: libc::c_int = 0;
#[no_mangle]
pub static mut idx_gt: libc::c_int = 0;
#[no_mangle]
pub static mut idx_key: *mut FIELD_PTR = 0 as *const FIELD_PTR as *mut FIELD_PTR;
#[no_mangle]
pub static mut log_fp: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut sty_fp: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut idx_fp: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut ind_fp: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut ilg_fp: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut pgm_fn: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut sty_fn: [libc::c_char; 72] = [0; 72];
#[no_mangle]
pub static mut idx_fn: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut ind: [libc::c_char; 256] = [0; 256];
#[no_mangle]
pub static mut ind_fn: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut ilg: [libc::c_char; 256] = [0; 256];
#[no_mangle]
pub static mut ilg_fn: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut pageno: [libc::c_char; 16] = [0; 16];
static mut log_fn: [libc::c_char; 256] = [0; 256];
static mut base: [libc::c_char; 256] = [0; 256];
static mut need_version: libc::c_int = 1;
#[no_mangle]
pub unsafe extern "C" fn main(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut fns = [std::ptr::null_mut::<libc::c_char>(); 1024];
    let mut ap = std::ptr::null_mut::<libc::c_char>();
    let mut use_stdin = false;
    let mut sty_given = false;
    let mut ind_given = false;
    let mut ilg_given = false;
    let mut log_given = false;
    pgm_fn = strrchr(*argv, '/' as i32);
    if pgm_fn.is_null() {
        pgm_fn = *argv;
    } else {
        pgm_fn = pgm_fn.offset(1);
    }
    loop {
        argc -= 1;
        if argc <= 0 {
            break;
        }
        argv = argv.offset(1);
        if **argv as libc::c_int == '-' as i32 {
            if *(*argv).offset(1) as libc::c_int == '\0' as i32 {
                break;
            }
            *argv = (*argv).offset(1);
            ap = *argv;
            while *ap as libc::c_int != '\0' as i32 {
                match *ap as libc::c_int {
                    105 => {
                        use_stdin = true;
                    }
                    108 => {
                        letter_ordering = 1;
                    }
                    114 => {
                        merge_page = 0;
                    }
                    113 => {
                        verbose = false;
                    }
                    99 => {
                        compress_blanks = 1;
                    }
                    115 => {
                        argc -= 1;

                        if argc <= 0 {
                            fprintf(
                                stderr(),
                                b"Expected -s <stylefile>\n\0" as *const u8 as *const libc::c_char,
                                b"\0" as *const u8 as *const libc::c_char,
                            );
                            fprintf(
                                stderr(),
                                b"Usage: %s [-ilqrcg] [-s sty] [-o ind] [-t log] [-p num] [idx0 idx1 ...]\n\0"
                                    as *const u8 as *const libc::c_char,
                                pgm_fn,
                            );
                            exit(1);
                        }
                        argv = argv.offset(1);
                        open_sty(*argv);
                        sty_given = true;
                    }
                    111 => {
                        argc -= 1;

                        if argc <= 0 {
                            fprintf(
                                stderr(),
                                b"Expected -o <ind>\n\0" as *const u8 as *const libc::c_char,
                                b"\0" as *const u8 as *const libc::c_char,
                            );
                            fprintf(
                                stderr(),
                                b"Usage: %s [-ilqrcg] [-s sty] [-o ind] [-t log] [-p num] [idx0 idx1 ...]\n\0"
                                    as *const u8 as *const libc::c_char,
                                pgm_fn,
                            );
                            exit(1);
                        }
                        argv = argv.offset(1);
                        ind_fn = *argv;
                        ind_given = true;
                    }
                    116 => {
                        argc -= 1;

                        if argc <= 0 {
                            fprintf(
                                stderr(),
                                b"Expected -t <logfile>\n\0" as *const u8 as *const libc::c_char,
                                b"\0" as *const u8 as *const libc::c_char,
                            );
                            fprintf(
                                stderr(),
                                b"Usage: %s [-ilqrcg] [-s sty] [-o ind] [-t log] [-p num] [idx0 idx1 ...]\n\0"
                                    as *const u8 as *const libc::c_char,
                                pgm_fn,
                            );
                            exit(1);
                        }
                        argv = argv.offset(1);
                        ilg_fn = *argv;
                        ilg_given = true;
                    }
                    112 => {
                        argc -= 1;

                        if argc <= 0 {
                            fprintf(
                                stderr(),
                                b"Expected -p <num>\n\0" as *const u8 as *const libc::c_char,
                                b"\0" as *const u8 as *const libc::c_char,
                            );
                            fprintf(
                                stderr(),
                                b"Usage: %s [-ilqrcg] [-s sty] [-o ind] [-t log] [-p num] [idx0 idx1 ...]\n\0"
                                    as *const u8 as *const libc::c_char,
                                pgm_fn,
                            );
                            exit(1);
                        }
                        argv = argv.offset(1);
                        strcpy(pageno.as_mut_ptr(), *argv);
                        init_page = 1;
                        if strcmp(
                            pageno.as_mut_ptr(),
                            b"even\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            log_given = true;
                            even_odd = 2 as libc::c_int;
                        } else if strcmp(
                            pageno.as_mut_ptr(),
                            b"odd\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            log_given = true;
                            even_odd = 1;
                        } else if strcmp(
                            pageno.as_mut_ptr(),
                            b"any\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            log_given = true;
                            even_odd = 0;
                        }
                    }
                    103 => {
                        german_sort = 1;
                    }
                    _ => {
                        fprintf(
                            stderr(),
                            b"Unknown option -%c.\n\0" as *const u8 as *const libc::c_char,
                            *ap as libc::c_int,
                        );
                        fprintf(
                            stderr(),
                            b"Usage: %s [-ilqrcg] [-s sty] [-o ind] [-t log] [-p num] [idx0 idx1 ...]\n\0"
                                as *const u8 as *const libc::c_char,
                            pgm_fn,
                        );
                        exit(1);
                    }
                }
                ap = ap.offset(1);
            
            }
        } else if fn_no < 1024 as libc::c_int {
            check_idx(*argv, 0);
            fn_no += 1;
            fns[fn_no as usize] = *argv;
        } else {
            fprintf(
                stderr(),
                b"Too many input files (max %d).\n\0" as *const u8 as *const libc::c_char,
                1024 as libc::c_int,
            );
            fprintf(
                stderr(),
                b"Usage: %s [-ilqrcg] [-s sty] [-o ind] [-t log] [-p num] [idx0 idx1 ...]\n\0"
                    as *const u8 as *const libc::c_char,
                pgm_fn,
            );
            exit(1);
        }
    }
    if fn_no == 0 && !sty_given {
        let mut tmp = [0; 261];
        sprintf(
            tmp.as_mut_ptr(),
            b"%s%s\0" as *const u8 as *const libc::c_char,
            base.as_mut_ptr(),
            b".mst\0" as *const u8 as *const libc::c_char,
        );
        if 0 == access(tmp.as_mut_ptr(), 4 as libc::c_int) {
            open_sty(tmp.as_mut_ptr());
            sty_given = true;
        }
    }
    process_idx(
        fns.as_mut_ptr(),
        use_stdin,
        sty_given,
        ind_given,
        ilg_given,
        log_given,
    );
    idx_gt = idx_tt - idx_et;
    if fn_no > 0 {
        if verbose {
            fprintf(
                stderr(),
                b"Overall %d files read (%d entries accepted, %d rejected).\n\0" as *const u8
                    as *const libc::c_char,
                fn_no + 1,
                idx_gt,
                idx_et,
            );
        }
        fprintf(
            ilg_fp,
            b"Overall %d files read (%d entries accepted, %d rejected).\n\0" as *const u8
                as *const libc::c_char,
            fn_no + 1,
            idx_gt,
            idx_et,
        );
    }
    if idx_gt > 0 {
        prepare_idx();
        sort_idx();
        gen_ind();
        if verbose {
            fprintf(
                stderr(),
                b"Output written in %s.\n\0" as *const u8 as *const libc::c_char,
                ind_fn,
            );
        }
        fprintf(
            ilg_fp,
            b"Output written in %s.\n\0" as *const u8 as *const libc::c_char,
            ind_fn,
        );
    } else {
        if verbose {
            fprintf(
                stderr(),
                b"Nothing written in %s.\n\0" as *const u8 as *const libc::c_char,
                ind_fn,
            );
        }
        fprintf(
            ilg_fp,
            b"Nothing written in %s.\n\0" as *const u8 as *const libc::c_char,
            ind_fn,
        );
    }
    if verbose {
        fprintf(
            stderr(),
            b"Transcript written in %s.\n\0" as *const u8 as *const libc::c_char,
            ilg_fn,
        );
    }
    fprintf(
        ilg_fp,
        b"Transcript written in %s.\n\0" as *const u8 as *const libc::c_char,
        ilg_fn,
    );
    fclose(ind_fp);
    fclose(ilg_fp);
    exit(0);
}
unsafe extern "C" fn prepare_idx() {
    let mut ptr = head;
    let mut i = 0;
    if head.is_null() {
        fprintf(
            stderr(),
            b"No valid index entries collected.\n\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr(),
            b"Usage: %s [-ilqrcg] [-s sty] [-o ind] [-t log] [-p num] [idx0 idx1 ...]\n\0"
                as *const u8 as *const libc::c_char,
            pgm_fn,
        );
        exit(1);
    }
    idx_key = calloc(
        idx_gt.try_into().unwrap(),
        ::core::mem::size_of::<FIELD_PTR>(),
    ) as *mut FIELD_PTR;
    if idx_key.is_null() {
        fprintf(
            stderr(),
            b"Not enough core...abort.\n\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr(),
            b"Usage: %s [-ilqrcg] [-s sty] [-o ind] [-t log] [-p num] [idx0 idx1 ...]\n\0"
                as *const u8 as *const libc::c_char,
            pgm_fn,
        );
        exit(1);
    }
    i = 0;
    while i < idx_gt {
        let fresh0 = &mut (*idx_key.offset(i as isize));
        *fresh0 = &mut (*ptr).data;
        ptr = (*ptr).next;
        i += 1;
    }
}
unsafe extern "C" fn process_idx(
    mut fn_0: *mut *mut libc::c_char,
    mut use_stdin: bool,
    mut sty_given: bool,
    mut ind_given: bool,
    mut ilg_given: bool,
    mut log_given: bool,
) {
    let mut i = 0;
    if fn_no == -(1) {
        use_stdin = true;
    } else {
        check_all(*fn_0.offset(0), ind_given, ilg_given, log_given);
        if verbose {
            fprintf(
                stderr(),
                b"This is %s, \0" as *const u8 as *const libc::c_char,
                pgm_fn,
            );
        }
        fprintf(
            ilg_fp,
            b"This is %s, \0" as *const u8 as *const libc::c_char,
            pgm_fn,
        );
        if verbose {
            fprintf(
                stderr(),
                b"%s.\n\0" as *const u8 as *const libc::c_char,
                b"portable version 2.12 [26-May-1993]\0" as *const u8 as *const libc::c_char,
            );
        }
        fprintf(
            ilg_fp,
            b"%s.\n\0" as *const u8 as *const libc::c_char,
            b"portable version 2.12 [26-May-1993]\0" as *const u8 as *const libc::c_char,
        );
        need_version = 0;
        if sty_given {
            scan_sty();
        }
        if german_sort != 0 && idx_quote as libc::c_int == '"' as i32 {
            fprintf(
                stderr(),
                b"Option -g invalid, quote character must be different from '%c'.\n\0" as *const u8
                    as *const libc::c_char,
                '"' as i32,
            );
            fprintf(
                stderr(),
                b"Usage: %s [-ilqrcg] [-s sty] [-o ind] [-t log] [-p num] [idx0 idx1 ...]\n\0"
                    as *const u8 as *const libc::c_char,
                pgm_fn,
            );
            exit(1);
        }
        scan_idx();
        ind_given = true;
        ilg_given = true;
        i = 1;
        while i <= fn_no {
            check_idx(*fn_0.offset(i as isize), 1);
            scan_idx();
            i += 1;
        }
    }
    if use_stdin {
        idx_fn = b"stdin\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        idx_fp = stdin();
        if ind_given {
            if ind_fp.is_null() && {
                ind_fp = fopen(ind_fn, b"w\0" as *const u8 as *const libc::c_char);
                ind_fp.is_null()
            } {
                fprintf(
                    stderr(),
                    b"Can't create output index file %s.\n\0" as *const u8 as *const libc::c_char,
                    ind_fn,
                );
                fprintf(
                    stderr(),
                    b"Usage: %s [-ilqrcg] [-s sty] [-o ind] [-t log] [-p num] [idx0 idx1 ...]\n\0"
                        as *const u8 as *const libc::c_char,
                    pgm_fn,
                );
                exit(1);
            }
        } else {
            ind_fn = b"stdout\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            ind_fp = stdout();
        }
        if ilg_given {
            if ilg_fp.is_null() && {
                ilg_fp = fopen(ilg_fn, b"w\0" as *const u8 as *const libc::c_char);
                ilg_fp.is_null()
            } {
                fprintf(
                    stderr(),
                    b"Can't create transcript file %s.\n\0" as *const u8 as *const libc::c_char,
                    ilg_fn,
                );
                fprintf(
                    stderr(),
                    b"Usage: %s [-ilqrcg] [-s sty] [-o ind] [-t log] [-p num] [idx0 idx1 ...]\n\0"
                        as *const u8 as *const libc::c_char,
                    pgm_fn,
                );
                exit(1);
            }
        } else {
            ilg_fn = b"stderr\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            ilg_fp = stderr();
        }
        if fn_no == -(1) && sty_given {
            scan_sty();
        }
        if german_sort != 0 && idx_quote as libc::c_int == '"' as i32 {
            fprintf(
                stderr(),
                b"Option -g ignored, quote character must be different from '%c'.\n\0" as *const u8
                    as *const libc::c_char,
                '"' as i32,
            );
            fprintf(
                stderr(),
                b"Usage: %s [-ilqrcg] [-s sty] [-o ind] [-t log] [-p num] [idx0 idx1 ...]\n\0"
                    as *const u8 as *const libc::c_char,
                pgm_fn,
            );
            exit(1);
        }
        if need_version != 0 {
            if verbose {
                fprintf(
                    stderr(),
                    b"This is %s, \0" as *const u8 as *const libc::c_char,
                    pgm_fn,
                );
            }
            fprintf(
                ilg_fp,
                b"This is %s, \0" as *const u8 as *const libc::c_char,
                pgm_fn,
            );
            if verbose {
                fprintf(
                    stderr(),
                    b"%s.\n\0" as *const u8 as *const libc::c_char,
                    b"portable version 2.12 [26-May-1993]\0" as *const u8 as *const libc::c_char,
                );
            }
            fprintf(
                ilg_fp,
                b"%s.\n\0" as *const u8 as *const libc::c_char,
                b"portable version 2.12 [26-May-1993]\0" as *const u8 as *const libc::c_char,
            );
            need_version = 0;
        }
        scan_idx();
        fn_no += 1;
    }
}
unsafe extern "C" fn check_idx(mut fn_0: *mut libc::c_char, mut open_fn: libc::c_int) {
    let mut ptr = fn_0;
    let mut ext = std::ptr::null_mut::<libc::c_char>();
    let mut with_ext = 0;
    let mut i = 0;
    ext = strrchr(fn_0, '.' as i32);
    if !ext.is_null() && ext != fn_0 && *ext.offset(1) as libc::c_int != '/' as i32 {
        with_ext = 1;
        while ptr != ext && i < 256 as libc::c_int {
            let fresh1 = ptr;
            ptr = ptr.offset(1);
            let fresh2 = i;
            i += 1;
            base[fresh2 as usize] = *fresh1;
        }
    } else {
        while *ptr as libc::c_int != '\0' as i32 && i < 256 as libc::c_int {
            let fresh3 = ptr;
            ptr = ptr.offset(1);
            let fresh4 = i;
            i += 1;
            base[fresh4 as usize] = *fresh3;
        }
    }
    if i < 256 as libc::c_int {
        base[i as usize] = '\0' as i32 as libc::c_char;
    } else {
        fprintf(
            stderr(),
            b"Index file name %s too long (max %d).\n\0" as *const u8 as *const libc::c_char,
            base.as_mut_ptr(),
            256 as libc::c_int,
        );
        fprintf(
            stderr(),
            b"Usage: %s [-ilqrcg] [-s sty] [-o ind] [-t log] [-p num] [idx0 idx1 ...]\n\0"
                as *const u8 as *const libc::c_char,
            pgm_fn,
        );
        exit(1);
    }
    idx_fn = fn_0;
    if open_fn != 0 && {
        idx_fp = fopen(idx_fn, b"r\0" as *const u8 as *const libc::c_char);
        idx_fp.is_null()
    } || open_fn == 0 && access(idx_fn, 4 as libc::c_int) != 0
    {
        if with_ext != 0 {
            fprintf(
                stderr(),
                b"Input index file %s not found.\n\0" as *const u8 as *const libc::c_char,
                idx_fn,
            );
            fprintf(
                stderr(),
                b"Usage: %s [-ilqrcg] [-s sty] [-o ind] [-t log] [-p num] [idx0 idx1 ...]\n\0"
                    as *const u8 as *const libc::c_char,
                pgm_fn,
            );
            exit(1);
        } else {
            idx_fn = malloc(256) as *mut libc::c_char;
            if idx_fn.is_null() {
                fprintf(
                    stderr(),
                    b"Not enough core...abort.\n\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                );
                fprintf(
                    stderr(),
                    b"Usage: %s [-ilqrcg] [-s sty] [-o ind] [-t log] [-p num] [idx0 idx1 ...]\n\0"
                        as *const u8 as *const libc::c_char,
                    pgm_fn,
                );
                exit(1);
            }
            sprintf(
                idx_fn,
                b"%s%s\0" as *const u8 as *const libc::c_char,
                base.as_mut_ptr(),
                b".idx\0" as *const u8 as *const libc::c_char,
            );
            if open_fn != 0 && {
                idx_fp = fopen(idx_fn, b"r\0" as *const u8 as *const libc::c_char);
                idx_fp.is_null()
            } || open_fn == 0 && access(idx_fn, 4 as libc::c_int) != 0
            {
                fprintf(
                    stderr(),
                    b"Couldn't find input index file %s nor %s.\n\0" as *const u8
                        as *const libc::c_char,
                    base.as_mut_ptr(),
                    idx_fn,
                );
                fprintf(
                    stderr(),
                    b"Usage: %s [-ilqrcg] [-s sty] [-o ind] [-t log] [-p num] [idx0 idx1 ...]\n\0"
                        as *const u8 as *const libc::c_char,
                    pgm_fn,
                );
                exit(1);
            }
        }
    }
}
unsafe extern "C" fn check_all(
    mut fn_0: *mut libc::c_char,
    mut ind_given: bool,
    mut ilg_given: bool,
    mut log_given: bool,
) {
    check_idx(fn_0, 1);
    if !ind_given {
        sprintf(
            ind.as_mut_ptr(),
            b"%s%s\0" as *const u8 as *const libc::c_char,
            base.as_mut_ptr(),
            b".ind\0" as *const u8 as *const libc::c_char,
        );
        ind_fn = ind.as_mut_ptr();
    }
    ind_fp = fopen(ind_fn, b"w\0" as *const u8 as *const libc::c_char);
    if ind_fp.is_null() {
        fprintf(
            stderr(),
            b"Can't create output index file %s.\n\0" as *const u8 as *const libc::c_char,
            ind_fn,
        );
        fprintf(
            stderr(),
            b"Usage: %s [-ilqrcg] [-s sty] [-o ind] [-t log] [-p num] [idx0 idx1 ...]\n\0"
                as *const u8 as *const libc::c_char,
            pgm_fn,
        );
        exit(1);
    }
    if !ilg_given {
        sprintf(
            ilg.as_mut_ptr(),
            b"%s%s\0" as *const u8 as *const libc::c_char,
            base.as_mut_ptr(),
            b".ilg\0" as *const u8 as *const libc::c_char,
        );
        ilg_fn = ilg.as_mut_ptr();
    }
    ilg_fp = fopen(ilg_fn, b"w\0" as *const u8 as *const libc::c_char);
    if ilg_fp.is_null() {
        fprintf(
            stderr(),
            b"Can't create transcript file %s.\n\0" as *const u8 as *const libc::c_char,
            ilg_fn,
        );
        fprintf(
            stderr(),
            b"Usage: %s [-ilqrcg] [-s sty] [-o ind] [-t log] [-p num] [idx0 idx1 ...]\n\0"
                as *const u8 as *const libc::c_char,
            pgm_fn,
        );
        exit(1);
    }
    if log_given {
        sprintf(
            log_fn.as_mut_ptr(),
            b"%s%s\0" as *const u8 as *const libc::c_char,
            base.as_mut_ptr(),
            b".log\0" as *const u8 as *const libc::c_char,
        );
        log_fp = fopen(
            log_fn.as_mut_ptr(),
            b"r\0" as *const u8 as *const libc::c_char,
        );
        if log_fp.is_null() {
            fprintf(
                stderr(),
                b"Source log file %s not found.\n\0" as *const u8 as *const libc::c_char,
                log_fn.as_mut_ptr(),
            );
            fprintf(
                stderr(),
                b"Usage: %s [-ilqrcg] [-s sty] [-o ind] [-t log] [-p num] [idx0 idx1 ...]\n\0"
                    as *const u8 as *const libc::c_char,
                pgm_fn,
            );
            exit(1);
        } else {
            find_pageno();
            fclose(log_fp);
        }
    }
}
unsafe extern "C" fn find_pageno() {
    let mut i = 0;
    let mut p = 0;
    let mut c = 0;
    fseek(log_fp, -(1), 2 as libc::c_int);
    p = fgetc(log_fp);
    fseek(log_fp, -(2), 1);
    loop {
        c = p;
        p = fgetc(log_fp);
        if p == '[' as i32
            && *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            || fseek(log_fp, -(2), 1) != 0
        {
            break;
        }
    }
    if p == '[' as i32 {
        loop {
            c = fgetc(log_fp);
            if c != ' ' as i32 {
                break;
            }
        }
        loop {
            let fresh5 = i;
            i += 1;
            pageno[fresh5 as usize] = c as libc::c_char;
            c = fgetc(log_fp);
            if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                == 0
            {
                break;
            }
        }
        pageno[i as usize] = '\0' as i32 as libc::c_char;
    } else {
        fprintf(
            ilg_fp,
            b"Couldn't find any page number in %s...ignored\n\0" as *const u8
                as *const libc::c_char,
            log_fn.as_mut_ptr(),
        );
        init_page = 0;
    };
}
unsafe extern "C" fn open_sty(mut fn_0: *mut libc::c_char) {
    let mut path = std::ptr::null_mut::<libc::c_char>();
    let mut ptr = std::ptr::null_mut::<libc::c_char>();
    let mut i = 0;
    let mut len = 0;
    path = getenv(b"INDEXSTYLE\0" as *const u8 as *const libc::c_char);
    if path.is_null() {
        strcpy(sty_fn.as_mut_ptr(), fn_0);
        sty_fp = fopen(
            sty_fn.as_mut_ptr(),
            b"r\0" as *const u8 as *const libc::c_char,
        );
    } else {
        len = (1024usize).wrapping_sub(strlen(fn_0)).wrapping_sub(1) as libc::c_int;
        while *path as libc::c_int != '\0' as i32 {
            ptr = strchr(path, ':' as i32);
            i = 0;
            if ptr.is_null() {
                let mut j = strlen(path) as libc::c_int;
                while i < j {
                    let fresh6 = path;
                    path = path.offset(1);
                    let fresh7 = i;
                    i += 1;
                    sty_fn[fresh7 as usize] = *fresh6;
                }
            } else {
                while path != ptr && i < len {
                    let fresh8 = path;
                    path = path.offset(1);
                    let fresh9 = i;
                    i += 1;
                    sty_fn[fresh9 as usize] = *fresh8;
                }
            }
            if i == len {
                fprintf(
                    stderr(),
                    b"Path %s too long (max %d).\n\0" as *const u8 as *const libc::c_char,
                    sty_fn.as_mut_ptr(),
                    1024 as libc::c_int,
                );
                fprintf(
                    stderr(),
                    b"Usage: %s [-ilqrcg] [-s sty] [-o ind] [-t log] [-p num] [idx0 idx1 ...]\n\0"
                        as *const u8 as *const libc::c_char,
                    pgm_fn,
                );
                exit(1);
            } else {
                if sty_fn[(i - 1) as usize] as libc::c_int != ']' as i32 {
                    let fresh10 = i;
                    i += 1;
                    sty_fn[fresh10 as usize] = '/' as i32 as libc::c_char;
                }
                sty_fn[i as usize] = '\0' as i32 as libc::c_char;
                strcat(sty_fn.as_mut_ptr(), fn_0);
                sty_fp = fopen(
                    sty_fn.as_mut_ptr(),
                    b"r\0" as *const u8 as *const libc::c_char,
                );
                if !sty_fp.is_null() {
                    break;
                }
                path = path.offset(1);
                path;
            }
        }
    }
    if sty_fp.is_null() {
        fprintf(
            stderr(),
            b"Index style file %s not found.\n\0" as *const u8 as *const libc::c_char,
            fn_0,
        );
        fprintf(
            stderr(),
            b"Usage: %s [-ilqrcg] [-s sty] [-o ind] [-t log] [-p num] [idx0 idx1 ...]\n\0"
                as *const u8 as *const libc::c_char,
            pgm_fn,
        );
        exit(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn strtoint(mut str: *mut libc::c_char) -> libc::c_int {
    let mut val = 0;
    while *str as libc::c_int != '\0' as i32 {
        val = 10 * val + *str as libc::c_int - 48 as libc::c_int;
        str = str.offset(1);
        str;
    }
    val
}
