#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
pub mod pardibaalrs {

    pub use bindings::pardibaal_DBM as PDBM;
    pub use bindings::pardibaal_bound_t as Bound;
    use bindings::pardibaal_bounds_table_t;

    mod bindings {
        include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    }
    pub fn zero(dim: u32) -> PDBM {
        unsafe {
            bindings::pardibaal_DBM_zero(dim)
        }
    }

    pub fn unconstrained(dim: u32) -> PDBM {
        unsafe {
            bindings::pardibaal_DBM_unconstrained(dim)
        }
    }

    pub fn satisfies(dbm: &PDBM, x: u32, y: u32, g: Bound) -> bool {
        unsafe {
            bindings::pardibaal_DBM_satisfies(dbm, x, y, g)
        }
    }

    pub fn subset(dbm1: &PDBM, dbm2: &PDBM) -> bool {
        let rel = unsafe { bindings::pardibaal_DBM_relation(dbm1, dbm2) };
        rel._equal || rel._subset
    }

    pub fn close(dbm: &mut PDBM) {
        unsafe {
            bindings::pardibaal_DBM_close(dbm)
        }
    }

    pub fn future(dbm: &mut PDBM) {
        unsafe {
            bindings::pardibaal_DBM_future(dbm)
        }
    }

    pub fn past(dbm: &mut PDBM) {
        unsafe {
            bindings::pardibaal_DBM_past(dbm)
        }
    }

    pub fn restrict(dbm: &mut PDBM, x: u32, y: u32, g: Bound) {
        unsafe {
            bindings::pardibaal_DBM_restrict(dbm, x, y, g)
        }
    }

    pub fn free(dbm: &mut PDBM, x: u32) {
        unsafe {
            bindings::pardibaal_DBM_free(dbm, x)
        }
    }

    pub fn assign(dbm: &mut PDBM, x: u32, v: i32) {
        unsafe {
            bindings::pardibaal_DBM_assign(dbm, x, v)
        }
    }

    pub fn copy(dbm: &mut PDBM, x: u32, y: u32) {
        unsafe {
            bindings::pardibaal_DBM_copy(dbm, x, y)
        }
    }

    pub fn shift(dbm: &mut PDBM, x: u32, v: i32) {
        unsafe {
            bindings::pardibaal_DBM_shift(dbm, x, v)
        }
    }

    pub fn new_bound(val: i32, strict: bool) -> Bound {
        unsafe { bindings::pardibaal_bound_t::new1(val, strict) }
    }

    impl Clone for pardibaal_bounds_table_t {
        fn clone(&self) -> Self {
            self.to_owned()
        }
    }

    impl Clone for PDBM {
        fn clone(&self) -> Self {
            self.to_owned()
        }
    }
}
#[cfg(test)]
mod test {
    use crate::pardibaalrs::*;
    #[test]
    fn smoke_test(){
    let dbm: PDBM = unconstrained(10 as u32);
    }
}
