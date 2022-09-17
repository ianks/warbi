#[allow(clippy::all)]
pub mod ruby_value_resource {
    #[allow(unused_imports)]
    use wit_bindgen_host_wasmtime_rust::{anyhow, wasmtime};
    pub trait RubyValueResource: Sized {
        type RubyValue: std::fmt::Debug;
        type Error;
        fn error_to_trap(&mut self, err: Self::Error) -> wasmtime::Trap;
        fn ruby_value_from_str(&mut self, input: &str) -> Result<Self::RubyValue, Self::Error>;

        fn ruby_value_from_i64(&mut self, input: i64) -> Result<Self::RubyValue, Self::Error>;

        fn ruby_value_from_f64(&mut self, input: f64) -> Result<Self::RubyValue, Self::Error>;

        fn ruby_value_from_bool(&mut self, input: bool) -> Result<Self::RubyValue, Self::Error>;

        fn ruby_value_to_string(&mut self, self_: &Self::RubyValue) -> Result<String, Self::Error>;

        fn drop_ruby_value(&mut self, state: Self::RubyValue) {
            drop(state);
        }
    }

    pub struct RubyValueResourceTables<T: RubyValueResource> {
        pub(crate) ruby_value_table: crate::table::Table<T::RubyValue>,
    }
    impl<T: RubyValueResource> Default for RubyValueResourceTables<T> {
        fn default() -> Self {
            Self {
                ruby_value_table: Default::default(),
            }
        }
    }
    pub fn add_to_linker<T, U>(
        linker: &mut wasmtime::Linker<T>,
        get: impl Fn(&mut T) -> (&mut U, &mut RubyValueResourceTables<U>) + Send + Sync + Copy + 'static,
    ) -> anyhow::Result<()>
    where
        U: RubyValueResource,
    {
        use wit_bindgen_host_wasmtime_rust::rt::get_func;
        use wit_bindgen_host_wasmtime_rust::rt::get_memory;
        linker.func_wrap(
            "ruby-value-resource",
            "ruby-value::from-str: func(input: string) -> handle<ruby-value>",
            move |mut caller: wasmtime::Caller<'_, T>, arg0: i32, arg1: i32| {
                let memory = &get_memory(&mut caller, "memory")?;
                let (mem, data) = memory.data_and_store_mut(&mut caller);
                let mut _bc = wit_bindgen_host_wasmtime_rust::BorrowChecker::new(mem);
                let host = get(data);
                let (host, _tables) = host;
                let ptr0 = arg0;
                let len0 = arg1;
                let param0 = _bc.slice_str(ptr0, len0)?;
                let result = match host.ruby_value_from_str(param0) {
                    Ok(val) => val,
                    Err(e) => return Err(host.error_to_trap(e)),
                };
                Ok(_tables.ruby_value_table.insert(result) as i32)
            },
        )?;
        linker.func_wrap(
            "ruby-value-resource",
            "ruby-value::from-i64: func(input: s64) -> handle<ruby-value>",
            move |mut caller: wasmtime::Caller<'_, T>, arg0: i64| {
                let host = get(caller.data_mut());
                let (host, _tables) = host;
                let param0 = arg0;
                let result = match host.ruby_value_from_i64(param0) {
                    Ok(val) => val,
                    Err(e) => return Err(host.error_to_trap(e)),
                };
                Ok(_tables.ruby_value_table.insert(result) as i32)
            },
        )?;
        linker.func_wrap(
            "ruby-value-resource",
            "ruby-value::from-f64: func(input: float64) -> handle<ruby-value>",
            move |mut caller: wasmtime::Caller<'_, T>, arg0: f64| {
                let host = get(caller.data_mut());
                let (host, _tables) = host;
                let param0 = arg0;
                let result = match host.ruby_value_from_f64(param0) {
                    Ok(val) => val,
                    Err(e) => return Err(host.error_to_trap(e)),
                };
                Ok(_tables.ruby_value_table.insert(result) as i32)
            },
        )?;
        linker.func_wrap(
            "ruby-value-resource",
            "ruby-value::from-bool: func(input: bool) -> handle<ruby-value>",
            move |mut caller: wasmtime::Caller<'_, T>, arg0: i32| {
                let host = get(caller.data_mut());
                let (host, _tables) = host;
                let param0 = match arg0 {
                    0 => false,
                    1 => true,
                    _ => return Err(invalid_variant("bool")),
                };
                let result = match host.ruby_value_from_bool(param0) {
                    Ok(val) => val,
                    Err(e) => return Err(host.error_to_trap(e)),
                };
                Ok(_tables.ruby_value_table.insert(result) as i32)
            },
        )?;
        linker.func_wrap(
            "ruby-value-resource",
            "ruby-value::to-string: func(self: handle<ruby-value>) -> string",
            move |mut caller: wasmtime::Caller<'_, T>, arg0: i32, arg1: i32| {
                let func = get_func(&mut caller, "cabi_realloc")?;
                let func_cabi_realloc = func.typed::<(i32, i32, i32, i32), i32, _>(&caller)?;
                let memory = &get_memory(&mut caller, "memory")?;
                let host = get(caller.data_mut());
                let (host, _tables) = host;
                let param0 = _tables
                    .ruby_value_table
                    .get((arg0) as u32)
                    .ok_or_else(|| wasmtime::Trap::new("invalid handle index"))?;
                let result = match host.ruby_value_to_string(param0) {
                    Ok(val) => val,
                    Err(e) => return Err(host.error_to_trap(e)),
                };
                let vec0 = result;
                let ptr0 = func_cabi_realloc.call(&mut caller, (0, 0, 1, vec0.len() as i32))?;
                let (caller_memory, data) = memory.data_and_store_mut(&mut caller);
                let (_, _tables) = get(data);
                caller_memory.store_many(ptr0, vec0.as_bytes())?;
                caller_memory.store(
                    arg1 + 4,
                    wit_bindgen_host_wasmtime_rust::rt::as_i32(vec0.len() as i32),
                )?;
                caller_memory.store(arg1 + 0, wit_bindgen_host_wasmtime_rust::rt::as_i32(ptr0))?;
                Ok(())
            },
        )?;
        linker.func_wrap(
            "canonical_abi",
            "resource_drop_ruby-value",
            move |mut caller: wasmtime::Caller<'_, T>, handle: u32| {
                let (host, tables) = get(caller.data_mut());
                let handle = tables
                    .ruby_value_table
                    .remove(handle)
                    .map_err(|e| wasmtime::Trap::new(format!("failed to remove handle: {}", e)))?;
                host.drop_ruby_value(handle);
                Ok(())
            },
        )?;
        Ok(())
    }
    use wit_bindgen_host_wasmtime_rust::rt::invalid_variant;
    use wit_bindgen_host_wasmtime_rust::rt::RawMem;
}
