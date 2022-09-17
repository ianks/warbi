#[allow(clippy::all)]
mod ruby_value_resource {
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct RubyValue(i32);
    impl RubyValue {
        pub unsafe fn from_raw(raw: i32) -> Self {
            Self(raw)
        }

        pub fn into_raw(self) -> i32 {
            let ret = self.0;
            core::mem::forget(self);
            return ret;
        }

        pub fn as_raw(&self) -> i32 {
            self.0
        }
    }
    impl Drop for RubyValue {
        fn drop(&mut self) {
            #[link(wasm_import_module = "canonical_abi")]
            extern "C" {
                #[link_name = "resource_drop_ruby-value"]
                fn close(fd: i32);
            }
            unsafe {
                close(self.0);
            }
        }
    }
    impl Clone for RubyValue {
        fn clone(&self) -> Self {
            #[link(wasm_import_module = "canonical_abi")]
            extern "C" {
                #[link_name = "resource_clone_ruby-value"]
                fn clone(val: i32) -> i32;
            }
            unsafe { Self(clone(self.0)) }
        }
    }
    impl RubyValue {
        pub fn from_str(input: &str) -> RubyValue {
            unsafe {
                let vec0 = input;
                let ptr0 = vec0.as_ptr() as i32;
                let len0 = vec0.len() as i32;
                #[link(wasm_import_module = "ruby-value-resource")]
                extern "C" {
                    #[cfg_attr(
                        target_arch = "wasm32",
                        link_name = "ruby-value::from-str: func(input: string) -> handle<ruby-value>"
                    )]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "ruby-value-resource_ruby-value::from-str: func(input: string) -> handle<ruby-value>"
                    )]
                    fn wit_import(_: i32, _: i32) -> i32;
                }
                let ret = wit_import(ptr0, len0);
                RubyValue(ret)
            }
        }
    }
    impl RubyValue {
        pub fn from_i64(input: i64) -> RubyValue {
            unsafe {
                #[link(wasm_import_module = "ruby-value-resource")]
                extern "C" {
                    #[cfg_attr(
                        target_arch = "wasm32",
                        link_name = "ruby-value::from-i64: func(input: s64) -> handle<ruby-value>"
                    )]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "ruby-value-resource_ruby-value::from-i64: func(input: s64) -> handle<ruby-value>"
                    )]
                    fn wit_import(_: i64) -> i32;
                }
                let ret = wit_import(wit_bindgen_guest_rust::rt::as_i64(input));
                RubyValue(ret)
            }
        }
    }
    impl RubyValue {
        pub fn from_f64(input: f64) -> RubyValue {
            unsafe {
                #[link(wasm_import_module = "ruby-value-resource")]
                extern "C" {
                    #[cfg_attr(
                        target_arch = "wasm32",
                        link_name = "ruby-value::from-f64: func(input: float64) -> handle<ruby-value>"
                    )]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "ruby-value-resource_ruby-value::from-f64: func(input: float64) -> handle<ruby-value>"
                    )]
                    fn wit_import(_: f64) -> i32;
                }
                let ret = wit_import(wit_bindgen_guest_rust::rt::as_f64(input));
                RubyValue(ret)
            }
        }
    }
    impl RubyValue {
        pub fn from_bool(input: bool) -> RubyValue {
            unsafe {
                #[link(wasm_import_module = "ruby-value-resource")]
                extern "C" {
                    #[cfg_attr(
                        target_arch = "wasm32",
                        link_name = "ruby-value::from-bool: func(input: bool) -> handle<ruby-value>"
                    )]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "ruby-value-resource_ruby-value::from-bool: func(input: bool) -> handle<ruby-value>"
                    )]
                    fn wit_import(_: i32) -> i32;
                }
                let ret = wit_import(match input {
                    true => 1,
                    false => 0,
                });
                RubyValue(ret)
            }
        }
    }
    impl RubyValue {
        pub fn to_string(&self) -> String {
            unsafe {
                let ptr0 = __RUBY_VALUE_RESOURCE_RET_AREA.0.as_mut_ptr() as i32;
                #[link(wasm_import_module = "ruby-value-resource")]
                extern "C" {
                    #[cfg_attr(
                        target_arch = "wasm32",
                        link_name = "ruby-value::to-string: func(self: handle<ruby-value>) -> string"
                    )]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "ruby-value-resource_ruby-value::to-string: func(self: handle<ruby-value>) -> string"
                    )]
                    fn wit_import(_: i32, _: i32);
                }
                wit_import(self.0, ptr0);
                let len1 = *((ptr0 + 4) as *const i32) as usize;
                String::from_utf8(Vec::from_raw_parts(
                    *((ptr0 + 0) as *const i32) as *mut _,
                    len1,
                    len1,
                ))
                .unwrap()
            }
        }
    }

    #[repr(align(4))]
    struct __RubyValueResourceRetArea([u8; 8]);
    static mut __RUBY_VALUE_RESOURCE_RET_AREA: __RubyValueResourceRetArea =
        __RubyValueResourceRetArea([0; 8]);
}
