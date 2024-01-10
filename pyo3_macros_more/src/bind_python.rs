//! Macros that bind common Python functionalities to Rust functions.
//!
//! Note: [deep import] results in a Python module with path enclosed in `[` and `]`
//! to be imported within a single call. This is necessary for certain modules that
//! are exposed only as their full path (commonly due to FFI bindings). For example,
//! `import omni.isaac.kit` works but `from omni.isaac import kit` might fail.

/// Bind a Python callable to a Rust function.
#[macro_export]
macro_rules! bind_python_callable {
    // [deep import, with GIL arg] Callable without arguments: `[mod.submod.**].**.callable(py: Python) => fn()`
    { $(#[$meta:meta])* [$module:expr]$(.$callable:ident)+() => $vis:vis fn $fn_name:ident$(<$lf_fn:lifetime>)?($gil:ident: $(::pyo3::)?Python$(<$lf_python:lifetime>)?) } => {
        $(#[$meta])*
        $vis fn $fn_name$(<$lf_fn>)?($gil: ::pyo3::Python$(<$lf_python>)?) -> ::pyo3::PyResult<()> {
            let module: &::pyo3::PyAny = ::pyo3::types::PyModule::import($gil, ::pyo3::intern!($gil, stringify!($module)))?;
            let callable = module$(.getattr(::pyo3::intern!($gil, stringify!($callable)))?)+;
            callable.call0()?;
            Ok(())
        }
    };
    // [deep import, with GIL arg] Callable without arguments: `[mod.submod.**].**.callable(py: Python) => fn() -> Result<value>`
    { $(#[$meta:meta])* [$module:expr]$(.$callable:ident)+() => $vis:vis fn $fn_name:ident$(<$lf_fn:lifetime>)?($gil:ident: $(::pyo3::)?Python$(<$lf_python:lifetime>)?) -> $(::pyo3::Py)?Result<$value:ty> } => {
        $(#[$meta])*
        $vis fn $fn_name$(<$lf_fn>)?($gil: ::pyo3::Python$(<$lf_python>)?) -> ::pyo3::PyResult<$value> {
            let module: &::pyo3::PyAny = ::pyo3::types::PyModule::import($gil, ::pyo3::intern!($gil, stringify!($module)))?;
            let callable = module$(.getattr(::pyo3::intern!($gil, stringify!($callable)))?)+;
            callable.call0()?.extract()
        }
    };
    // [with GIL arg] Callable with keyword arguments: `mod.**.callable() => fn(py: Python, arg: type, ...)
    { $(#[$meta:meta])* $module:ident$(.$callable:ident)+() => $vis:vis fn $fn_name:ident$(<$lf_fn:lifetime>)?($gil:ident: $(::pyo3::)?Python$(<$lf_python:lifetime>)?$(, $arg:ident: $arg_type:ty)+) } => {
        $crate::bind_python_callable! {
            $(#[$meta])*
            [$module]$(.$callable)+() => $vis fn $fn_name$(<$lf_fn>)?($gil: ::pyo3::Python$(<$lf_python>)?$(, $arg: $arg_type)+)
        }
    };
    // [deep import, with GIL arg] Callable with keyword arguments: `[mod.submod.**].**.callable() => fn(arg: type, ...)
    { $(#[$meta:meta])* [$module:expr]$(.$callable:ident)+() => $vis:vis fn $fn_name:ident$(<$lf_fn:lifetime>)?($gil:ident: $(::pyo3::)?Python$(<$lf_python:lifetime>)?$(, $arg:ident: $arg_type:ty)+) } => {
        $(#[$meta])*
        $vis fn $fn_name$(<$lf_fn>)?($gil: ::pyo3::Python$(<$lf_python>)?$(, $arg: $arg_type)*) -> ::pyo3::PyResult<()> {
            let module: &::pyo3::PyAny = ::pyo3::types::PyModule::import($gil, ::pyo3::intern!($gil, stringify!($module)))?;
            let callable = module$(.getattr(::pyo3::intern!($gil, stringify!($callable)))?)+;
            let kwargs = ::pyo3::types::PyDict::new($gil);
            $(
                if stringify!($arg).starts_with("r#") {
                    kwargs.set_item(stringify!($arg).trim_start_matches("r#"), $arg)?;
                }
                else {
                    kwargs.set_item(::pyo3::intern!($gil, stringify!($arg)), $arg)?;
                }
            )+
            callable.call((), Some(kwargs))?;
            Ok(())
        }
    };
    // [with GIL arg] Callable with keyword arguments: `mod.**.callable() => fn(py: Python, arg: type, ...) -> Result<value>`
    { $(#[$meta:meta])* $module:ident$(.$callable:ident)+() => $vis:vis fn $fn_name:ident$(<$lf_fn:lifetime>)?($gil:ident: $(::pyo3::)?Python$(<$lf_python:lifetime>)?$(, $arg:ident: $arg_type:ty)+) -> $(::pyo3::Py)?Result<$value:ty> } => {
        $crate::bind_python_callable! {
            $(#[$meta])*
            [$module]$(.$callable)+() => $vis fn $fn_name$(<$lf_fn>)?($gil: ::pyo3::Python$(<$lf_python>)?$(, $arg: $arg_type)+) -> Result<$value>
        }
    };
    // [deep import, with GIL arg] Callable with keyword arguments: `[mod.submod.**].**.callable() => fn(arg: type, ...) -> Result<value>`
    { $(#[$meta:meta])* [$module:expr]$(.$callable:ident)+() => $vis:vis fn $fn_name:ident$(<$lf_fn:lifetime>)?($gil:ident: $(::pyo3::)?Python$(<$lf_python:lifetime>)?$(, $arg:ident: $arg_type:ty)+) -> $(::pyo3::Py)?Result<$value:ty> } => {
        $(#[$meta])*
        $vis fn $fn_name$(<$lf_fn>)?($gil: ::pyo3::Python$(<$lf_python>)?$(, $arg: $arg_type)*) -> ::pyo3::PyResult<$value> {
            let module: &::pyo3::PyAny = ::pyo3::types::PyModule::import($gil, ::pyo3::intern!($gil, stringify!($module)))?;
            let callable = module$(.getattr(::pyo3::intern!($gil, stringify!($callable)))?)+;
            let kwargs = ::pyo3::types::PyDict::new($gil);
            $(
                if stringify!($arg).starts_with("r#") {
                    kwargs.set_item(stringify!($arg).trim_start_matches("r#"), $arg)?;
                }
                else {
                    kwargs.set_item(::pyo3::intern!($gil, stringify!($arg)), $arg)?;
                }
            )+
            callable.call((), Some(kwargs))?.extract()
        }
    };
    // Callable without arguments: `mod.**.callable() => fn()`
    { $(#[$meta:meta])* $module:ident$(.$callable:ident)+() => $vis:vis fn $fn_name:ident$(<$lf_fn:lifetime>)?($($gil:ident: $(::pyo3::)?Python$(<$lf_python:lifetime>)?)?) } => {
        $crate::bind_python_callable! {
            $(#[$meta])*
            [$module]$(.$callable)+() => $vis fn $fn_name$(<$lf_fn>)?($($gil: ::pyo3::Python$(<$lf_python>)?)?)
        }
    };
    // Callable without arguments: `mod.**.callable() => fn() -> Result<value>`
    { $(#[$meta:meta])* $module:ident$(.$callable:ident)+() => $vis:vis fn $fn_name:ident$(<$lf_fn:lifetime>)?($($gil:ident: $(::pyo3::)?Python$(<$lf_python:lifetime>)?)?) -> $(::pyo3::Py)?Result<$value:ty> } => {
        $crate::bind_python_callable! {
            $(#[$meta])*
            [$module]$(.$callable)+() => $vis fn $fn_name$(<$lf_fn>)?($($gil: ::pyo3::Python$(<$lf_python>)?)?) -> Result<$value>
        }
    };
    // [deep import] Callable without arguments: `[mod.submod.**].**.callable() => fn()`
    { $(#[$meta:meta])* [$module:expr]$(.$callable:ident)+() => $vis:vis fn $fn_name:ident() } => {
        $(#[$meta])*
        $vis fn $fn_name() -> ::pyo3::PyResult<()> {
            ::pyo3::Python::with_gil(|py| {
                let module: &::pyo3::PyAny = ::pyo3::types::PyModule::import(py, ::pyo3::intern!(py, stringify!($module)))?;
                let callable = module$(.getattr(::pyo3::intern!(py, stringify!($callable)))?)+;
                callable.call0()?;
                Ok(())
            })
        }
    };
    // [deep import] Callable without arguments: `[mod.submod.**].**.callable() => fn() -> Result<value>`
    { $(#[$meta:meta])* [$module:expr]$(.$callable:ident)+() => $vis:vis fn $fn_name:ident() -> $(::pyo3::Py)?Result<$value:ty> } => {
        $(#[$meta])*
        $vis fn $fn_name() -> ::pyo3::PyResult<$value> {
            ::pyo3::Python::with_gil(|py| {
                let module: &::pyo3::PyAny = ::pyo3::types::PyModule::import(py, ::pyo3::intern!(py, stringify!($module)))?;
                let callable = module$(.getattr(::pyo3::intern!(py, stringify!($callable)))?)+;
                callable.call0()?.extract()
            })
        }
    };
    // Callable with keyword arguments: `mod.**.callable() => fn(arg: type, ...)
    { $(#[$meta:meta])* $module:ident$(.$callable:ident)+() => $vis:vis fn $fn_name:ident($($arg:ident: $arg_type:ty),+) } => {
        $crate::bind_python_callable! {
            $(#[$meta])*
            [$module]$(.$callable)+() => $vis fn $fn_name($($arg: $arg_type),+)
        }
    };
    // Callable with keyword arguments: `mod.**.callable() => fn(arg: type, ...) -> Result<value>`
    { $(#[$meta:meta])* $module:ident$(.$callable:ident)+() => $vis:vis fn $fn_name:ident($($arg:ident: $arg_type:ty),+) -> $(::pyo3::Py)?Result<$value:ty> } => {
        $crate::bind_python_callable! {
            $(#[$meta])*
            [$module]$(.$callable)+() => $vis fn $fn_name($($arg: $arg_type),+) -> Result<$value>
        }
    };
    // [deep import] Callable with keyword arguments: `[mod.submod.**].**.callable() => fn(arg: type, ...)
    { $(#[$meta:meta])* [$module:expr]$(.$callable:ident)+() => $vis:vis fn $fn_name:ident($($arg:ident: $arg_type:ty),*) } => {
        $(#[$meta])*
        $vis fn $fn_name($($arg: $arg_type),+) -> ::pyo3::PyResult<()> {
            ::pyo3::Python::with_gil(|py| {
                let module: &::pyo3::PyAny = ::pyo3::types::PyModule::import(py, ::pyo3::intern!(py, stringify!($module)))?;
                let callable = module$(.getattr(::pyo3::intern!(py, stringify!($callable)))?)+;
                let kwargs = ::pyo3::types::PyDict::new(py);
                $(
                    if stringify!($arg).starts_with("r#") {
                        kwargs.set_item(stringify!($arg).trim_start_matches("r#"), $arg)?;
                    }
                    else {
                        kwargs.set_item(::pyo3::intern!(py, stringify!($arg)), $arg)?;
                    }
                )+
                callable.call((), Some(kwargs))?;
                Ok(())
            })
        }
    };
    // [deep import] Callable with keyword arguments: `[mod.submod.**].**.callable() => fn(arg: type, ...) -> Result<value>`
    { $(#[$meta:meta])* [$module:expr]$(.$callable:ident)+() => $vis:vis fn $fn_name:ident($($arg:ident: $arg_type:ty),*) -> $(::pyo3::Py)?Result<$value:ty> } => {
        $(#[$meta])*
        $vis fn $fn_name($($arg: $arg_type),+) -> ::pyo3::PyResult<$value> {
            ::pyo3::Python::with_gil(|py| {
                let module: &::pyo3::PyAny = ::pyo3::types::PyModule::import(py, ::pyo3::intern!(py, stringify!($module)))?;
                let callable = module$(.getattr(::pyo3::intern!(py, stringify!($callable)))?)+;
                let kwargs = ::pyo3::types::PyDict::new(py);
                $(
                    if stringify!($arg).starts_with("r#") {
                        kwargs.set_item(stringify!($arg).trim_start_matches("r#"), $arg)?;
                    }
                    else {
                        kwargs.set_item(::pyo3::intern!(py, stringify!($arg)), $arg)?;
                    }
                )+
                callable.call((), Some(kwargs))?.extract()
            })
        }
    };
}

/// Bind a Python callable of `self` to a Rust function.
#[macro_export]
macro_rules! bind_python_self_callable {
    // [with GIL arg, &mut self] Callable without arguments: `mod.**.callable(py: Python) => fn()`
    { $(#[$meta:meta])* $module:ident$(.$callable:ident)+() => $vis:vis fn $fn_name:ident$(<$lf_fn:lifetime>)?(&$($lf_self:lifetime)?mut self, $gil:ident: $(::pyo3::)?Python$(<$lf_python:lifetime>)?) } => {
        $(#[$meta])*
        $vis fn $fn_name$(<$lf_fn>)?(&$($lf_self)?mut self, $gil: ::pyo3::Python$(<$lf_python>)?) -> ::pyo3::PyResult<()> {
            let callable = self.as_ref($gil)$(.getattr(::pyo3::intern!($gil, stringify!($callable)))?)+;
            callable.call0()?;
            Ok(())
        }
    };
    // [with GIL arg, &mut self] Callable without arguments: `mod.**.callable(py: Python) => fn() -> Result<value>`
    { $(#[$meta:meta])* $module:ident$(.$callable:ident)+() => $vis:vis fn $fn_name:ident$(<$lf_fn:lifetime>)?(&$($lf_self:lifetime)?mut self, $gil:ident: $(::pyo3::)?Python$(<$lf_python:lifetime>)?) -> $(::pyo3::Py)?Result<$value:ty> } => {
        $(#[$meta])*
        $vis fn $fn_name$(<$lf_fn>)?(&$($lf_self)?mut self, $gil: ::pyo3::Python$(<$lf_python>)?) -> ::pyo3::PyResult<$value> {
            let callable = self.as_ref($gil)$(.getattr(::pyo3::intern!($gil, stringify!($callable)))?)+;
            callable.call0()?.extract()
        }
    };
    // [with GIL arg, &mut self] Callable with keyword arguments: `mod.**.callable() => fn(arg: type, ...)
    { $(#[$meta:meta])* $module:ident$(.$callable:ident)+() => $vis:vis fn $fn_name:ident$(<$lf_fn:lifetime>)?(&$($lf_self:lifetime)?mut self, $gil:ident: $(::pyo3::)?Python$(<$lf_python:lifetime>)?$(, $arg:ident: $arg_type:ty)+) } => {
        $(#[$meta])*
        $vis fn $fn_name$(<$lf_fn>)?(&$($lf_self)?mut self, $gil: ::pyo3::Python$(<$lf_python>)?$(, $arg: $arg_type)*) -> ::pyo3::PyResult<()> {
            let callable = self.as_ref($gil)$(.getattr(::pyo3::intern!($gil, stringify!($callable)))?)+;
            let kwargs = ::pyo3::types::PyDict::new($gil);
            $(
                if stringify!($arg).starts_with("r#") {
                    kwargs.set_item(stringify!($arg).trim_start_matches("r#"), $arg)?;
                }
                else {
                    kwargs.set_item(::pyo3::intern!($gil, stringify!($arg)), $arg)?;
                }
            )+
            callable.call((), Some(kwargs))?;
            Ok(())
        }
    };
    // [with GIL arg, &mut self] Callable with keyword arguments: `mod.**.callable() => fn(arg: type, ...) -> Result<value>`
    { $(#[$meta:meta])* $module:ident$(.$callable:ident)+() => $vis:vis fn $fn_name:ident$(<$lf_fn:lifetime>)?(&$($lf_self:lifetime)?mut self, $gil:ident: $(::pyo3::)?Python$(<$lf_python:lifetime>)?$(, $arg:ident: $arg_type:ty)+) -> $(::pyo3::Py)?Result<$value:ty> } => {
        $(#[$meta])*
        $vis fn $fn_name$(<$lf_fn>)?(&$($lf_self)?mut self, $gil: ::pyo3::Python$(<$lf_python>)?$(, $arg: $arg_type)*) -> ::pyo3::PyResult<$value> {
            let callable = self.as_ref($gil)$(.getattr(::pyo3::intern!($gil, stringify!($callable)))?)+;
            let kwargs = ::pyo3::types::PyDict::new($gil);
            $(
                if stringify!($arg).starts_with("r#") {
                    kwargs.set_item(stringify!($arg).trim_start_matches("r#"), $arg)?;
                }
                else {
                    kwargs.set_item(::pyo3::intern!($gil, stringify!($arg)), $arg)?;
                }
            )+
            callable.call((), Some(kwargs))?.extract()
        }
    };
    // [&mut self] Callable without arguments: `mod.**.callable() => fn()`
    { $(#[$meta:meta])* $module:ident$(.$callable:ident)+() => $vis:vis fn $fn_name:ident(&mut self) } => {
        $(#[$meta])*
        $vis fn $fn_name(&mut self) -> ::pyo3::PyResult<()> {
            ::pyo3::Python::with_gil(|py| {
                let callable = self.as_ref(py)$(.getattr(::pyo3::intern!(py, stringify!($callable)))?)+;
                callable.call0()?;
                Ok(())
            })
        }
    };
    // [&mut self] Callable without arguments: `mod.**.callable() => fn() -> Result<value>`
    { $(#[$meta:meta])* $module:ident$(.$callable:ident)+() => $vis:vis fn $fn_name:ident(&mut self) -> $(::pyo3::Py)?Result<$value:ty> } => {
        $(#[$meta])*
        $vis fn $fn_name(&mut self) -> ::pyo3::PyResult<$value> {
            ::pyo3::Python::with_gil(|py| {
                let callable = self.as_ref(py)$(.getattr(::pyo3::intern!(py, stringify!($callable)))?)+;
                callable.call0()?.extract()
            })
        }
    };
    // [&mut self] Callable with keyword arguments: `mod.**.callable() => fn(arg: type, ...)`
    { $(#[$meta:meta])* $module:ident$(.$callable:ident)+() => $vis:vis fn $fn_name:ident(&mut self, $($arg:ident: $arg_type:ty),*) } => {
        $(#[$meta])*
        $vis fn $fn_name(&mut self, $($arg: $arg_type),+) -> ::pyo3::PyResult<()> {
            ::pyo3::Python::with_gil(|py| {
                let callable = self.as_ref(py)$(.getattr(::pyo3::intern!(py, stringify!($callable)))?)+;
                let kwargs = ::pyo3::types::PyDict::new(py);
                $(
                    if stringify!($arg).starts_with("r#") {
                        kwargs.set_item(stringify!($arg).trim_start_matches("r#"), $arg)?;
                    }
                    else {
                        kwargs.set_item(::pyo3::intern!(py, stringify!($arg)), $arg)?;
                    }
                )+
                callable.call((), Some(kwargs))?;
                Ok(())
            })
        }
    };
    // [&mut self] Callable with keyword arguments: `mod.**.callable() => fn(arg: type, ...) -> Result<value>`
    { $(#[$meta:meta])* $module:ident$(.$callable:ident)+() => $vis:vis fn $fn_name:ident(&mut self, $($arg:ident: $arg_type:ty),*) -> $(::pyo3::Py)?Result<$value:ty> } => {
        $(#[$meta])*
        $vis fn $fn_name(&mut self, $($arg: $arg_type),+) -> ::pyo3::PyResult<$value> {
            ::pyo3::Python::with_gil(|py| {
                let callable = self.as_ref(py)$(.getattr(::pyo3::intern!(py, stringify!($callable)))?)+;
                let kwargs = ::pyo3::types::PyDict::new(py);
                $(
                    if stringify!($arg).starts_with("r#") {
                        kwargs.set_item(stringify!($arg).trim_start_matches("r#"), $arg)?;
                    }
                    else {
                        kwargs.set_item(::pyo3::intern!(py, stringify!($arg)), $arg)?;
                    }
                )+
                callable.call((), Some(kwargs))?.extract()
            })
        }
    };
    // [with GIL arg] Callable without arguments: `mod.**.callable(py: Python) => fn()`
    { $(#[$meta:meta])* $module:ident$(.$callable:ident)+() => $vis:vis fn $fn_name:ident$(<$lf_fn:lifetime>)?(&$($lf_self:lifetime)?self, $gil:ident: $(::pyo3::)?Python$(<$lf_python:lifetime>)?) } => {
        $(#[$meta])*
        $vis fn $fn_name$(<$lf_fn>)?(&$($lf_self)?self, $gil: ::pyo3::Python$(<$lf_python>)?) -> ::pyo3::PyResult<()> {
            let callable = self.as_ref($gil)$(.getattr(::pyo3::intern!($gil, stringify!($callable)))?)+;
            callable.call0()?;
            Ok(())
        }
    };
    // [with GIL arg] Callable without arguments: `mod.**.callable(py: Python) => fn() -> Result<value>`
    { $(#[$meta:meta])* $module:ident$(.$callable:ident)+() => $vis:vis fn $fn_name:ident$(<$lf_fn:lifetime>)?(&$($lf_self:lifetime)?self, $gil:ident: $(::pyo3::)?Python$(<$lf_python:lifetime>)?) -> $(::pyo3::Py)?Result<$value:ty> } => {
        $(#[$meta])*
        $vis fn $fn_name$(<$lf_fn>)?(&$($lf_self)?self, $gil: ::pyo3::Python$(<$lf_python>)?) -> ::pyo3::PyResult<$value> {
            let callable = self.as_ref($gil)$(.getattr(::pyo3::intern!($gil, stringify!($callable)))?)+;
            callable.call0()?.extract()
        }
    };
    // [with GIL arg] Callable with keyword arguments: `mod.**.callable() => fn(arg: type, ...)
    { $(#[$meta:meta])* $module:ident$(.$callable:ident)+() => $vis:vis fn $fn_name:ident$(<$lf_fn:lifetime>)?(&$($lf_self:lifetime)?self, $gil:ident: $(::pyo3::)?Python$(<$lf_python:lifetime>)?$(, $arg:ident: $arg_type:ty)+) } => {
        $(#[$meta])*
        $vis fn $fn_name$(<$lf_fn>)?(&$($lf_self)?self, $gil: ::pyo3::Python$(<$lf_python>)?$(, $arg: $arg_type)*) -> ::pyo3::PyResult<()> {
            let callable = self.as_ref($gil)$(.getattr(::pyo3::intern!($gil, stringify!($callable)))?)+;
            let kwargs = ::pyo3::types::PyDict::new($gil);
            $(
                if stringify!($arg).starts_with("r#") {
                    kwargs.set_item(stringify!($arg).trim_start_matches("r#"), $arg)?;
                }
                else {
                    kwargs.set_item(::pyo3::intern!($gil, stringify!($arg)), $arg)?;
                }
            )+
            callable.call((), Some(kwargs))?;
            Ok(())
        }
    };
    // [with GIL arg] Callable with keyword arguments: `mod.**.callable() => fn(arg: type, ...) -> Result<value>`
    { $(#[$meta:meta])* $module:ident$(.$callable:ident)+() => $vis:vis fn $fn_name:ident$(<$lf_fn:lifetime>)?(&$($lf_self:lifetime)?self, $gil:ident: $(::pyo3::)?Python$(<$lf_python:lifetime>)?$(, $arg:ident: $arg_type:ty)+) -> $(::pyo3::Py)?Result<$value:ty> } => {
        $(#[$meta])*
        $vis fn $fn_name$(<$lf_fn>)?(&$($lf_self)?self, $gil: ::pyo3::Python$(<$lf_python>)?$(, $arg: $arg_type)*) -> ::pyo3::PyResult<$value> {
            let callable = self.as_ref($gil)$(.getattr(::pyo3::intern!($gil, stringify!($callable)))?)+;
            let kwargs = ::pyo3::types::PyDict::new($gil);
            $(
                if stringify!($arg).starts_with("r#") {
                    kwargs.set_item(stringify!($arg).trim_start_matches("r#"), $arg)?;
                }
                else {
                    kwargs.set_item(::pyo3::intern!($gil, stringify!($arg)), $arg)?;
                }
            )+
            callable.call((), Some(kwargs))?.extract()
        }
    };
    // Callable without arguments: `mod.**.callable() => fn()`
    { $(#[$meta:meta])* $module:ident$(.$callable:ident)+() => $vis:vis fn $fn_name:ident(&self) } => {
        $(#[$meta])*
        $vis fn $fn_name(&self) -> ::pyo3::PyResult<()> {
            ::pyo3::Python::with_gil(|py| {
                let callable = self.as_ref(py)$(.getattr(::pyo3::intern!(py, stringify!($callable)))?)+;
                callable.call0()?;
                Ok(())
            })
        }
    };
    // Callable without arguments: `mod.**.callable() => fn() -> Result<value>`
    { $(#[$meta:meta])* $module:ident$(.$callable:ident)+() => $vis:vis fn $fn_name:ident(&self) -> $(::pyo3::Py)?Result<$value:ty> } => {
        $(#[$meta])*
        $vis fn $fn_name(&self) -> ::pyo3::PyResult<$value> {
            ::pyo3::Python::with_gil(|py| {
                let callable = self.as_ref(py)$(.getattr(::pyo3::intern!(py, stringify!($callable)))?)+;
                callable.call0()?.extract()
            })
        }
    };
    // Callable with keyword arguments: `mod.**.callable() => fn(arg: type, ...)`
    { $(#[$meta:meta])* $module:ident$(.$callable:ident)+() => $vis:vis fn $fn_name:ident(&self, $($arg:ident: $arg_type:ty),*) } => {
        $(#[$meta])*
        $vis fn $fn_name(&self, $($arg: $arg_type),+) -> ::pyo3::PyResult<()> {
            ::pyo3::Python::with_gil(|py| {
                let callable = self.as_ref(py)$(.getattr(::pyo3::intern!(py, stringify!($callable)))?)+;
                let kwargs = ::pyo3::types::PyDict::new(py);
                $(
                    if stringify!($arg).starts_with("r#") {
                        kwargs.set_item(stringify!($arg).trim_start_matches("r#"), $arg)?;
                    }
                    else {
                        kwargs.set_item(::pyo3::intern!(py, stringify!($arg)), $arg)?;
                    }
                )+
                callable.call((), Some(kwargs))?;
                Ok(())
            })
        }
    };
    // Callable with keyword arguments: `mod.**.callable() => fn(arg: type, ...) -> Result<value>`
    { $(#[$meta:meta])* $module:ident$(.$callable:ident)+() => $vis:vis fn $fn_name:ident(&self, $($arg:ident: $arg_type:ty),*) -> $(::pyo3::Py)?Result<$value:ty> } => {
        $(#[$meta])*
        $vis fn $fn_name(&self, $($arg: $arg_type),+) -> ::pyo3::PyResult<$value> {
            ::pyo3::Python::with_gil(|py| {
                let callable = self.as_ref(py)$(.getattr(::pyo3::intern!(py, stringify!($callable)))?)+;
                let kwargs = ::pyo3::types::PyDict::new(py);
                $(
                    if stringify!($arg).starts_with("r#") {
                        kwargs.set_item(stringify!($arg).trim_start_matches("r#"), $arg)?;
                    }
                    else {
                        kwargs.set_item(::pyo3::intern!(py, stringify!($arg)), $arg)?;
                    }
                )+
                callable.call((), Some(kwargs))?.extract()
            })
        }
    };
}

/// Write a test for the existence of Python getter.
#[macro_export]
macro_rules! test_bind_python_getter {
    // Getter `mod.**.attr`
    { $(#[$meta:meta])* $module:ident$(.$attr:ident)+ } => {
        $crate::test_bind_python_getter! {
            $(#[$meta])*
            [$module]$(.$attr)+
        }
    };
    // [deep import] Getter `[mod.submod.**].**.attr`
    { $(#[$meta:meta])* [$module:expr]$(.$attr:ident)+ } => {
        $(#[$meta])*
        #[cfg(test)]
        fn test_$fn_name() {
            ::pyo3::Python::with_gil(|py| {
                let mod_or_attr: &::pyo3::PyAny = ::pyo3::types::PyModule::import(py, ::pyo3::intern!(py, stringify!($module))).unwrap();
                $(
                assert!(mod_or_attr.hasattr(::pyo3::intern!(py, stringify!($attr))).unwrap())
                let mod_or_attr = mod_or_attr.getattr(::pyo3::intern!(py, stringify!($attr))).unwrap();
                )+
            });
        }
    };
}

/// Bind a Python getter to a Rust function.
#[macro_export]
macro_rules! bind_python_getter {
    // [with GIL arg] Getter `mod.**.attr => fn(py: ::pyo3::Python) -> Result<value>`
    { $(#[$meta:meta])* $module:ident$(.$attr:ident)+ => $vis:vis fn $fn_name:ident$(<$lf_fn:lifetime>)?($gil:ident: $(::pyo3::)?Python$(<$lf_python:lifetime>)?) -> $(::pyo3::Py)?Result<$value:ty> } => {
        $crate::bind_python_getter! {
            $(#[$meta])*
            [$module]$(.$attr)+ => $vis fn $fn_name$(<$lf_fn>)?($gil: ::pyo3::Python$(<$lf_python>)?) -> Result<$value>
        }
    };
    // [deep import, with GIL arg] Getter `[mod.submod.**].**.attr => fn(py: ::pyo3::Python) -> Result<value>`
    { $(#[$meta:meta])* [$module:expr]$(.$attr:ident)+ => $vis:vis fn $fn_name:ident$(<$lf_fn:lifetime>)?($gil:ident: $(::pyo3::)?Python$(<$lf_python:lifetime>)?) -> $(::pyo3::Py)?Result<$value:ty> } => {
        $(#[$meta])*
        $vis fn $fn_name$(<$lf_fn>)?($gil: ::pyo3::Python$(<$lf_python>)?) -> ::pyo3::PyResult<$value> {
            let module: &::pyo3::PyAny = ::pyo3::types::PyModule::import($gil, ::pyo3::intern!($gil, stringify!($module)))?;
            let attr = module$(.getattr(::pyo3::intern!($gil, stringify!($attr)))?)+;
            attr.extract()
        }
    };
    // Getter `mod.**.attr => fn() -> Result<value>`
    { $(#[$meta:meta])* $module:ident$(.$attr:ident)+ => $vis:vis fn $fn_name:ident$(<$lf_fn:lifetime>)?() -> $(::pyo3::Py)?Result<$value:ty> } => {
        $crate::bind_python_getter! {
            $(#[$meta])*
            [$module]$(.$attr)+ => $vis fn $fn_name$(<$lf_fn>)?() -> Result<$value>
        }
    };
    // [deep import] Getter `[mod.submod.**].**.attr => fn() -> Result<value>`
    { $(#[$meta:meta])* [$module:expr]$(.$attr:ident)+ => $vis:vis fn $fn_name:ident() -> $(::pyo3::Py)?Result<$value:ty> } => {
        $(#[$meta])*
        $vis fn $fn_name() -> ::pyo3::PyResult<$value> {
            ::pyo3::Python::with_gil(|py| {
                let module: &::pyo3::PyAny = ::pyo3::types::PyModule::import(py, ::pyo3::intern!(py, stringify!($module)))?;
                let attr = module$(.getattr(::pyo3::intern!(py, stringify!($attr)))?)+;
                attr.extract()
            })
        }
    };
}

/// Bind a Python getter of `self` to a Rust function.
#[macro_export]
macro_rules! bind_python_self_getter {
    // [with GIL arg] Self getter `self.**.attr => fn(&self, py: ::pyo3::Python) -> Result<value>`
    { $(#[$meta:meta])* self$(.$attr:ident)+ => $vis:vis fn $fn_name:ident$(<$lf_fn:lifetime>)?(&$($lf_self:lifetime)?self, $gil:ident: $(::pyo3::)?Python$(<$lf_python:lifetime>)?) -> $(::pyo3::Py)?Result<$value:ty> } => {
        $(#[$meta])*
        $vis fn $fn_name$(<$lf_fn>)?(&$($lf_self)?self, $gil: ::pyo3::Python$(<$lf_python>)?) -> ::pyo3::PyResult<$value> {
            let attr = self.as_ref($gil)$(.getattr(::pyo3::intern!($gil, stringify!($attr)))?)+;
            attr.extract()
        }
    };
    // Self getter `self.**.attr => fn(&self) -> Result<value>`
    { $(#[$meta:meta])* self$(.$attr:ident)+ => $vis:vis fn $fn_name:ident(&self) -> $(::pyo3::Py)?Result<$value:ty> } => {
        $(#[$meta])*
        $vis fn $fn_name$(<$lf_fn>)?(&$($lf_self)?self) -> ::pyo3::PyResult<$value> {
            ::pyo3::Python::with_gil(|py| {
                let attr = self.as_ref(py)$(.getattr(::pyo3::intern!(py, stringify!($attr)))?)+;
                attr.extract()
            })
        }
    };
}

/// Bind a Python setter to a Rust function.
#[macro_export]
macro_rules! bind_python_setter {
    // Setter: `mod.attr = fn(py: Python, value: type)`
    { $(#[$meta:meta])* $module:ident.$attr:ident = $($macro_tail:tt)+ } => {
        $crate::bind_python_self_setter! { @inner end $(#[$meta])* [$module] $attr = $($macro_tail)+ }
    };
    // [with GIL arg] Setter: `mod.**.attr = fn(py: Python, value: type)`
    { $(#[$meta:meta])* $module:ident$(.$attr:ident)+ = $vis:vis fn $fn_name:ident$(<$lf_fn:lifetime>)?($gil:ident: $(::pyo3::)?Python$(<$lf_python:lifetime>)?, $value:ident: $value_type:ty) } => {
        $crate::bind_python_setter! { @inner +last $(#[$meta])* $module [ $(.$attr)+ & $(.$attr)+ ] = $vis fn $fn_name$(<$lf_fn>)?($gil: ::pyo3::Python$(<$lf_python>)?, $value: $value_type) }
    };
    // [deep import, with GIL arg] Setter: `[mod.submod.**].attr = fn(py: Python, value: type)`
    { $(#[$meta:meta])* [$module:expr].$attr:ident = $vis:vis fn $fn_name:ident$(<$lf_fn:lifetime>)?($gil:ident: $(::pyo3::)?Python$(<$lf_python:lifetime>)?, $value:ident: $value_type:ty) } => {
        $crate::bind_python_setter! { @inner end $(#[$meta])* [$module] $attr = $vis fn $fn_name$(<$lf_fn>)?($gil: ::pyo3::Python$(<$lf_python>)?, $value: $value_type) }
    };
    // Setter: `mod.**.attr => fn(value: type)`
    { $(#[$meta:meta])* $module:ident$(.$attr:ident)+ = $vis:vis fn $fn_name:ident($value:ident: $value_type:ty) } => {
        $crate::bind_python_setter! { @inner +last $(#[$meta])* $module [ $(.$attr)+ & $(.$attr)+ ] = $vis fn $fn_name($value: $value_type) }
    };
    // [deep import] Setter: `[mod.submod.**].attr = fn(value: type)`
    { $(#[$meta:meta])* [$module:expr].$attr:ident = $vis:vis fn $fn_name:ident($value:ident: $value_type:ty) } => {
        $crate::bind_python_setter! { @inner end $(#[$meta])* [$module] $attr = $vis fn $fn_name($value: $value_type) }
    };
    // Everything below are inner matches that enable consistent syntax for the macro above
    { @inner +last $(#[$meta:meta])* $module:ident [ $(.$attr:ident)+ & .$first1:ident$(.$rest1:ident)+ ] = $($macro_tail:tt)+ } => {
        $crate::bind_python_setter! { @inner +last $(#[$meta])* $module [ $(.$attr)+ & $(.$rest1)+ ] = $($macro_tail)+ }
    };
    { @inner +last $(#[$meta:meta])* $module:ident [ $(.$attr:ident)+ & .$first1:ident ] = $($macro_tail:tt)+ } => {
        $crate::bind_python_setter! { @inner -last $(#[$meta])* $module [ $(.$attr)+ & .$first1 ] = $($macro_tail)+ }
    };
    { @inner -last $(#[$meta:meta])* $module:ident [ .$first:ident & .$last1:ident ] = $($macro_tail:tt)+ } => {
        $crate::bind_python_setter! { @inner end $(#[$meta])* [$module].$first $last1 = $($macro_tail)+ }
    };
    { @inner -last $(#[$meta:meta])* $module:ident [ .$first:ident$(.$rest:ident)+ & .$last1:ident ] = $($macro_tail:tt)+ } => {
        $crate::bind_python_setter! { @inner -last $(#[$meta])* $module [ .$first | $(.$rest)+ & .$last1 ] = $($macro_tail)+ }
    };
    { @inner -last $(#[$meta:meta])* $module:ident [ $(.$extracted:ident)+ | .$first:ident$(.$rest:ident)+ & .$last1:ident ] = $($macro_tail:tt)+ } => {
        $crate::bind_python_setter! { @inner -last $(#[$meta])* $module [ $(.$extracted)+.$first | $(.$rest)+ & .$last1 ] = $($macro_tail)+ }
    };
    { @inner -last $(#[$meta:meta])* $module:ident [ $(.$extracted:ident)+ | .$_:ident & .$last1:ident ] = $($macro_tail:tt)+ } => {
        $crate::bind_python_setter! { @inner end $(#[$meta])* [$module]$(.$extracted)+ $last1 = $($macro_tail)+ }
    };
    { @inner end $(#[$meta:meta])* [$module:expr]$(.$submodule:ident)* $attr:ident = $vis:vis fn $fn_name:ident$(<$lf_fn:lifetime>)?($gil:ident: $(::pyo3::)?Python$(<$lf_python:lifetime>)?, $value:ident: $value_type:ty) } => {
        $(#[$meta])*
        $vis fn $fn_name$(<$lf_fn>)?($gil: ::pyo3::Python$(<$lf_python>)?, $value: $value_type) -> ::pyo3::PyResult<()> {
            let module: &::pyo3::PyAny = ::pyo3::types::PyModule::import($gil, ::pyo3::intern!($gil, stringify!($module)))?;
            let submodule = module$(.getattr(::pyo3::intern!($gil, stringify!($submodule)))?)*;
            submodule.setattr(::pyo3::intern!($gil, stringify!($attr)), $value)?;
            Ok(())
        }
    };
    { @inner end $(#[$meta:meta])* [$module:expr]$(.$submodule:ident)* $attr:ident = $vis:vis fn $fn_name:ident($value:ident: $value_type:ty) } => {
        $(#[$meta])*
        $vis fn $fn_name($value: $value_type) -> ::pyo3::PyResult<()> {
            ::pyo3::Python::with_gil(|py| {
                let module: &::pyo3::PyAny = ::pyo3::types::PyModule::import(py, ::pyo3::intern!(py, stringify!($module)))?;
                let submodule = module$(.getattr(::pyo3::intern!(py, stringify!($submodule)))?)*;
                submodule.setattr(::pyo3::intern!(py, stringify!($attr)), $value)?;
                Ok(())
            })
        }
    };
}

/// Bind a Python setter of `self` to a Rust function.
#[macro_export]
macro_rules! bind_python_self_setter {
    // Setter: `self.attr = fn(py: Python, value: type)`
    { $(#[$meta:meta])* self.$attr:ident = $($macro_tail:tt)+ } => {
        $crate::bind_python_self_setter! { @inner end $(#[$meta])* self $attr = $($macro_tail)+ }
    };
    // [with GIL arg] Setter: `self.**.attr = fn(py: Python, value: type)`
    { $(#[$meta:meta])* self$(.$attr:ident)+ = $vis:vis fn $fn_name:ident$(<$lf_fn:lifetime>)?(&$($lf_self:lifetime)?mut self, $gil:ident: $(::pyo3::)?Python$(<$lf_python:lifetime>)?, $value:ident: $value_type:ty) } => {
        $crate::bind_python_self_setter! { @inner +last $(#[$meta])* self$(.$attr)+ & $(.$attr)+ = $vis fn $fn_name$(<$lf_fn>)?(&$($lf_self)?mut self, $gil: ::pyo3::Python$(<$lf_python>)?, $value: $value_type) }
    };
    // Setter: `self.**.attr = fn(value: type)`
    { $(#[$meta:meta])* self$(.$attr:ident)+ = $vis:vis fn $fn_name:ident$(<$lf_fn:lifetime>)?(&$($lf_self:lifetime)?mut self, $value:ident: $value_type:ty) } => {
        $crate::bind_python_self_setter! { @inner +last $(#[$meta])* self$(.$attr)+ & $(.$attr)+ = $vis fn $fn_name$(<$lf_fn>)?(&$($lf_self)?mut self, $value: $value_type) }
    };
    // Everything below are inner matches that enable consistent syntax for the macro above
    { @inner +last $(#[$meta:meta])* self$(.$attr:ident)+ & .$first1:ident$(.$rest1:ident)+ = $($macro_tail:tt)+ } => {
        $crate::bind_python_self_setter! { @inner +last $(#[$meta])* self$(.$attr)+ & $(.$rest1)+ = $($macro_tail)+ }
    };
    { @inner +last $(#[$meta:meta])* self$(.$attr:ident)+ & .$first1:ident = $($macro_tail:tt)+ } => {
        $crate::bind_python_self_setter! { @inner -last $(#[$meta])* self$(.$attr)+ & .$first1 = $($macro_tail)+ }
    };
    { @inner -last $(#[$meta:meta])* self.$first:ident & .$last1:ident = $($macro_tail:tt)+ } => {
        $crate::bind_python_self_setter! { @inner end $(#[$meta])* self.$first $last1 = $($macro_tail)+ }
    };
    { @inner -last $(#[$meta:meta])* self.$first:ident$(.$rest:ident)+ & .$last1:ident = $($macro_tail:tt)+ } => {
        $crate::bind_python_self_setter! { @inner -last $(#[$meta])* self.$first | $(.$rest)+ & .$last1 = $($macro_tail)+ }
    };
    { @inner -last $(#[$meta:meta])* self$(.$extracted:ident)+ | .$first:ident$(.$rest:ident)+ & .$last1:ident = $($macro_tail:tt)+ } => {
        $crate::bind_python_self_setter! { @inner -last $(#[$meta])* self$(.$extracted)+.$first | $(.$rest)+ & .$last1 = $($macro_tail)+ }
    };
    { @inner -last $(#[$meta:meta])* self$(.$extracted:ident)+ | .$_:ident & .$last1:ident = $($macro_tail:tt)+ } => {
        $crate::bind_python_self_setter! { @inner end $(#[$meta])* self$(.$extracted)+ $last1 = $($macro_tail)+ }
    };
    { @inner end $(#[$meta:meta])* self$(.$submodule:ident)* $attr:ident = $vis:vis fn $fn_name:ident$(<$lf_fn:lifetime>)?(&$($lf_self:lifetime)?mut self, $gil:ident: $(::pyo3::)?Python$(<$lf_python:lifetime>)?, $value:ident: $value_type:ty) } => {
        $(#[$meta])*
        $vis fn $fn_name$(<$lf_fn>)?(&$($lf_self)?mut self, $gil: ::pyo3::Python$(<$lf_python>)?, $value: $value_type) -> ::pyo3::PyResult<()> {
            let submodule = self.as_ref($gil)$(.getattr(::pyo3::intern!($gil, stringify!($submodule)))?)*;
            submodule.setattr(::pyo3::intern!($gil, stringify!($attr)), $value)?;
            Ok(())
        }
    };
    { @inner end $(#[$meta:meta])* self$(.$submodule:ident)* $attr:ident = $vis:vis fn $fn_name:ident(&mut self, $value:ident: $value_type:ty) } => {
        $(#[$meta])*
        $vis fn $fn_name(&mut self, $value: $value_type) -> ::pyo3::PyResult<()> {
            ::pyo3::Python::with_gil(|py| {
                let submodule = self.as_ref(py)$(.getattr(::pyo3::intern!(py, stringify!($submodule)))?)*;
                submodule.setattr(::pyo3::intern!(py, stringify!($attr)), $value)?;
                Ok(())
            })
        }
    };
}

/// Wrapper for inner Python bindings (with `py: pyo3::Python` as the first argument)
/// into a Rust function that can be called without the `py` argument while
/// returning values wrapper around `pyo3::Py`.
#[macro_export]
macro_rules! python_wrap_with_gil {
    // With empty return type
    { $(#[$meta:meta])* $fn_name_source:path as $vis:vis $fn_name_target:ident($($arg:ident: $arg_type:ty),*) } => {
        $(#[$meta])*
        $vis fn $fn_name_target($($arg: $arg_type),*) -> ::pyo3::PyResult<()> {
            ::pyo3::Python::with_gil(|py| {
                $fn_name_source(py $(,$arg)*)?;
                Ok(())
            })
        }
    };
    // With Vec in the return type
    { $(#[$meta:meta])* $fn_name_source:path as $vis:vis $fn_name_target:ident($($arg:ident: $arg_type:ty),*) -> $(::pyo3::Py)?Result<Vec<$value:ty>> } => {
        $(#[$meta])*
        $vis fn $fn_name_target($($arg: $arg_type),*) -> ::pyo3::PyResult<Vec<$value>> {
            ::pyo3::Python::with_gil(|py| {
                Ok($fn_name_source(py $(,$arg)*)?.into_iter().map(|x| x.into()).collect())
            })
        }
    };
    // With non-iterable return type
    { $(#[$meta:meta])* $fn_name_source:path as $vis:vis $fn_name_target:ident($($arg:ident: $arg_type:ty),*) -> $(::pyo3::Py)?Result<$value:ty> } => {
        $(#[$meta])*
        $vis fn $fn_name_target($($arg: $arg_type),*) -> ::pyo3::PyResult<$value> {
            ::pyo3::Python::with_gil(|py| {
                Ok($fn_name_source(py $(,$arg)*)?.into())
            })
        }
    };
    // Combination of multiple python_wrap_with_gil! macros
    { $($macro_tail:tt)* } => {
        $( $crate::python_wrap_with_gil! $macro_tail )*
    };
    [ $($macro_tail:tt)* ] => {
        $( $crate::python_wrap_with_gil! $macro_tail )*
    };
}

/// Macro that binds common Python functionalities to Rust functions.
/// It is a combination of [`bind_python_callable!`], [`bind_python_getter!`], [`bind_python_setter!`] and [`python_wrap_with_gil!`].
#[macro_export]
macro_rules! bind_python {
    // Self callable (must be before other callable)
    { $(#[$meta:meta])* self$(.$callable:ident)+() => $($macro_tail:tt)+  } => {
        $crate::bind_python_self_callable! {
            $(#[$meta])*
            self$(.$callable)+() => $($macro_tail)+
        }
    };
    // Callable
    { $(#[$meta:meta])* $module:ident$(.$callable:ident)+() => $($macro_tail:tt)+ } => {
        $crate::bind_python_callable! {
            $(#[$meta])*
            $module$(.$callable)+() => $($macro_tail)+
        }
    };
    // [deep import] Callable
    { $(#[$meta:meta])* [$module:expr]$(.$callable:ident)+() => $($macro_tail:tt)+ } => {
        $crate::bind_python_callable! {
            $(#[$meta])*
            [$module]$(.$callable)+() => $($macro_tail)+
        }
    };
    // Self getter (must be before other getter)
    { $(#[$meta:meta])* self$(.$attr:ident)+ => $($macro_tail:tt)+ } => {
        $crate::bind_python_self_getter! {
            $(#[$meta])*
            self$(.$attr)+ => $($macro_tail)+
        }
    };
    // Getter
    { $(#[$meta:meta])* $module:ident$(.$attr:ident)+ => $($macro_tail:tt)+ } => {
        $crate::bind_python_getter! {
            $(#[$meta])*
            $module$(.$attr)+ => $($macro_tail)+
        }
    };
    // [deep import] Getter
    { $(#[$meta:meta])* [$module:expr]$(.$attr:ident)+ => $($macro_tail:tt)+ } => {
        $crate::bind_python_getter! {
            $(#[$meta])*
            [$module]$(.$attr)+ => $($macro_tail)+
        }
    };
    // Self setter (must be before other setter)
    { $(#[$meta:meta])* self$(.$attr:ident)+ = $($macro_tail:tt)+ } => {
        $crate::bind_python_self_setter! {
            $(#[$meta])*
            self$(.$attr)+ = $($macro_tail)+
        }
    };
    // Setter
    { $(#[$meta:meta])* $module:ident$(.$attr:ident)+ = $($macro_tail:tt)+ } => {
        $crate::bind_python_setter! {
            $(#[$meta])*
            $module$(.$attr)+ = $($macro_tail)+
        }
    };
    // [deep import] Setter
    { $(#[$meta:meta])* [$module:expr].$attr:ident = $($macro_tail:tt)+ } => {
        $crate::bind_python_setter! {
            $(#[$meta])*
            [$module].$attr = $($macro_tail)+
        }
    };
    // Wrapping of inner Python bindings
    { $(#[$meta:meta])* $fn_name_source:path as $($macro_tail:tt)+ } => {
        $crate::python_wrap_with_gil! {
            $(#[$meta])*
            $fn_name_source as $($macro_tail)+
        }
    };
    // Combination of multiple bind_python! macros
    { $($macro:tt)* } => {
        $( $crate::bind_python! $macro )*
    };
    [ $($macro:tt)* ] => {
        $( $crate::bind_python! $macro )*
    };
}
