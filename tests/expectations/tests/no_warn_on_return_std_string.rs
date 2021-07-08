#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub mod std {
        #[allow(unused_imports)]
        use self::super::super::root;
        pub type string = [u64; 4usize];
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct StdStringReturnedByValue {
        pub _address: u8,
    }
    #[test]
    fn bindgen_test_layout_StdStringReturnedByValue() {
        assert_eq!(
            ::std::mem::size_of::<StdStringReturnedByValue>(),
            1usize,
            concat!("Size of: ", stringify!(StdStringReturnedByValue))
        );
        assert_eq!(
            ::std::mem::align_of::<StdStringReturnedByValue>(),
            1usize,
            concat!("Alignment of ", stringify!(StdStringReturnedByValue))
        );
    }
    extern "C" {
        #[link_name = "\u{1}_ZN24StdStringReturnedByValue5valueEv"]
        pub fn StdStringReturnedByValue_value(
            this: *mut root::StdStringReturnedByValue,
        ) -> root::std::string;
    }
    impl StdStringReturnedByValue {
        #[inline]
        pub unsafe fn value(&mut self) -> root::std::string {
            StdStringReturnedByValue_value(self)
        }
    }
}
