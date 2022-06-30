pub use libc::{c_double, c_int, c_void};
pub use std::ptr::{null, null_mut};

extern "C" {
    fn CINTgto_norm(l: c_int, e: c_double) -> c_double;
}

pub fn gto_norm(l: i32, e: f64) -> f64 {
    unsafe { CINTgto_norm(l, e) }
}

pub struct CINToptimizer {
    opt: *const c_void,
    allocated: bool,
}

impl Drop for CINToptimizer {
    fn drop(&mut self) {
        extern "C" {
            fn CINTdel_optimizer(opt: *mut *const c_void);
        }

        if self.allocated {
            unsafe {
                CINTdel_optimizer(&mut self.opt as *mut _);
            }
        }
    }
}

#[macro_export]
macro_rules! cint_opt {
    ($f:ident) => {{
        extern "C" {
            fn $f(
                opt: *mut *const c_void,
                atm: *const c_int,
                natm: c_int,
                bas: *const c_int,
                nbas: c_int,
                env: *mut c_double,
            );
        }

        |atm: &[[i32; 6]], bas: &[[i32; 8]], env: &mut [f64]| {
            let mut opt = null();
            unsafe {
                $f(
                    &mut opt as *mut _,
                    atm.as_ptr() as *const c_int,
                    atm.len() as c_int,
                    bas.as_ptr() as *const c_int,
                    bas.len() as c_int,
                    env.as_mut_ptr(),
                );
            }
            CINToptimizer {
                opt,
                allocated: true,
            }
        }
    }};
}

#[allow(unused_macros)]
macro_rules! cint_func {
    ($f:ident, $n_shl:expr) => {{
        extern "C" {
            fn $f(
                buf: *mut c_double,
                dims: *const c_int,
                shls: *const c_int,
                atm: *const c_int,
                natm: c_int,
                bas: *const c_int,
                nbas: c_int,
                env: *mut c_double,
                opt: *const c_void,
                cache: *mut c_double,
            ) -> c_int;
        }

        |buf: &mut [f64],
         shls: [i32; $n_shl],
         atm: &[[i32; 6]],
         bas: &[[i32; 8]],
         env: &mut [f64],
         opt: &CINToptimizer| unsafe {
            $f(
                buf.as_mut_ptr(),
                null(),
                shls.as_ptr(),
                atm.as_ptr() as *const c_int,
                atm.len() as i32,
                bas.as_ptr() as *const c_int,
                bas.len() as i32,
                env.as_mut_ptr(),
                opt.opt,
                null_mut(),
            )
        }
    }};
}

#[allow(unused_macros)]
macro_rules! cint_noopt {
    ($f:ident, $n_shl:expr) => {{
        let func = cint_func!($f, $n_shl);

        move |buf: &mut [f64],
         shls: [i32; $n_shl],
         atm: &[[i32; 6]],
         bas: &[[i32; 8]],
         env: &mut [f64]| {
            func(
                buf,
                shls,
                atm,
                bas,
                env,
                &CINToptimizer {
                    opt: null(),
                    allocated: false,
                },
            )
        }
    }};
}

#[macro_export]
macro_rules! cint1e_opt {
    ($f:ident) => {
        cint_func!($f, 2)
    };
}

#[macro_export]
macro_rules! cint2e_opt {
    ($f:ident) => {
        cint_func!($f, 4)
    };
}

#[macro_export]
macro_rules! cint1e {
    ($f:ident) => {
        cint_noopt!($f, 2)
    };
}

#[macro_export]
macro_rules! cint2e {
    ($f:ident) => {
        cint_noopt!($f, 4)
    };
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_norm() {
        let norm = unsafe { CINTgto_norm(1, 1.0) };

        println!("Test1: {}", norm);
    }

    fn get_h2o_ccpvdz_params() -> ([[i32; 6]; 3], [[i32; 8]; 12], [f64; 106]) {
        let atm = [
            [8, 20, 0, 0, 0, 0],
            [1, 74, 0, 0, 0, 0],
            [1, 90, 0, 0, 0, 0],
        ];
        let bas = [
            [0, 0, 9, 1, 0, 24, 33, 0],
            [0, 0, 9, 1, 0, 42, 51, 0],
            [0, 0, 1, 1, 0, 60, 61, 0],
            [0, 1, 4, 1, 0, 62, 66, 0],
            [0, 1, 1, 1, 0, 70, 71, 0],
            [0, 2, 1, 1, 0, 72, 73, 0],
            [1, 0, 4, 1, 0, 78, 82, 0],
            [1, 0, 1, 1, 0, 86, 87, 0],
            [1, 1, 1, 1, 0, 88, 89, 0],
            [2, 0, 4, 1, 0, 94, 98, 0],
            [2, 0, 1, 1, 0, 102, 103, 0],
            [2, 1, 1, 1, 0, 104, 105, 0],
        ];
        let env = [
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            11720.0,
            1759.0,
            400.8,
            113.7,
            37.03,
            13.27,
            5.025,
            1.013,
            0.3023,
            2.020547580103431,
            3.7536323225593904,
            6.299893943574668,
            9.219272574580845,
            10.735242992903203,
            7.882113341271768,
            2.2975207101423387,
            0.03943441578671931,
            -0.002662589297873141,
            11720.0,
            1759.0,
            400.8,
            113.7,
            37.03,
            13.27,
            5.025,
            1.013,
            0.3023,
            -0.45533466593880145,
            -0.8666979201814461,
            -1.4183078400827118,
            -2.262240587098483,
            -2.689821926039761,
            -2.905578435263727,
            -0.9917126821529172,
            1.4218839085400548,
            0.5899504772381131,
            0.3023,
            1.0300152022720082,
            17.7,
            3.854,
            1.046,
            0.2753,
            4.556183329863166,
            3.6061558120480353,
            1.5699457292299095,
            0.26791746556315915,
            0.2753,
            0.5817577222014568,
            1.185,
            3.5118543773803417,
            1.8897261246257702,
            0.0,
            0.0,
            0.0,
            13.01,
            1.962,
            0.4446,
            0.122,
            0.34068923869698303,
            0.5778910580209711,
            0.6577403143783562,
            0.2614150690824873,
            0.122,
            0.5215367270818116,
            0.727,
            1.9584045348700287,
            0.0,
            1.8897261246257702,
            0.0,
            0.0,
            13.01,
            1.962,
            0.4446,
            0.122,
            0.34068923869698303,
            0.5778910580209711,
            0.6577403143783562,
            0.2614150690824873,
            0.122,
            0.5215367270818116,
            0.727,
            1.9584045348700287,
        ];

        (atm, bas, env)
    }

    #[test]
    fn test_overlap() {
        let ovlp_func = cint1e!(int1e_ovlp_sph);

        let mut buf = [0.0; 9];

        let (atm, bas, mut env) = get_h2o_ccpvdz_params();

        ovlp_func(&mut buf, [3, 8], &atm, &bas, &mut env);
        println!("overlap: {:?}\n", buf);

        let check = [
            -0.33116533479016613,
            0.0,
            0.0,
            -0.0,
            0.28759899604856765,
            0.0,
            -0.0,
            0.0,
            0.28759899604856765,
        ];

        for (a, b) in buf.iter().zip(check.iter()) {
            assert!((a - b).abs() < 1e-10);
        }
    }

    #[test]
    fn test_eri() {
        let eri_func = cint2e!(int2e_sph);

        let mut buf = [0.0; 15];

        let (atm, bas, mut env) = get_h2o_ccpvdz_params();

        eri_func(&mut buf, [1, 6, 11, 5], &atm, &bas, &mut env);

        let check = [
            0.08479537298282352,
            -0.015420694555831652,
            0.0,
            0.0,
            0.0,
            0.08253851715317655,
            -0.006719324031110618,
            0.03367549074917227,
            0.0,
            0.0,
            0.0,
            0.0060861207370332526,
            0.0015438892280071129,
            0.05281528750373969,
            0.0,
        ];

        for (a, b) in buf.iter().zip(check.iter()) {
            assert!((a - b).abs() < 1e-10);
        }
    }

    #[test]
    fn test_eri_opt() {
        let eri_func = cint2e_opt!(int2e_sph);
        let opt_func = cint_opt!(int2e_optimizer);

        let mut buf = [0.0; 15];

        let (atm, bas, mut env) = get_h2o_ccpvdz_params();

        let opt = opt_func(&atm, &bas, &mut env);

        eri_func(&mut buf, [1, 6, 11, 5], &atm, &bas, &mut env, &opt);

        let check = [
            0.08479537298282352,
            -0.015420694555831652,
            0.0,
            0.0,
            0.0,
            0.08253851715317655,
            -0.006719324031110618,
            0.03367549074917227,
            0.0,
            0.0,
            0.0,
            0.0060861207370332526,
            0.0015438892280071129,
            0.05281528750373969,
            0.0,
        ];

        for (a, b) in buf.iter().zip(check.iter()) {
            assert!((a - b).abs() < 1e-10);
        }
    }
}
