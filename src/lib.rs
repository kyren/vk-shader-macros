use proc_macro_hack::proc_macro_hack;

/// Compile a GLSL source file into a binary SPIR-V constant
///
/// ```
/// use vk_shader_macros::include_glsl;
/// const VERT: &[u32] = include_glsl!("example.vert");
/// ```
///
/// Due to limitations of proc macros, paths are resolved relative to the crate root.
///
/// # Options
///
/// Compile options may be specified as additional arguments. Supported options include:
/// - `kind: <kind>` - Specify shader kind. Valid kinds are the same as the recognized file
///    extensions: `vert`, `frag`, `comp`, `geom`, `tesc`, and `tese`. If omitted, kind is inferred
///    from the file's extension, or a pragma in the source.
/// - `version: <version>` - Specify GLSL version. If omitted, version must be specified in the
///    source with `#version`
/// - `strip` - Omit debug info (set as default by enabling the `strip` feature)
/// - `debug` - Force debug info, even with the `strip` feature enabled
/// - `define: <name> ["value"]` - Define the preprocessor macro `<name>` as `value`
#[proc_macro_hack]
pub use vk_shader_macros_impl::include_glsl;

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    const TEST: &[u32] = include_glsl!("example.vert", version: 450);
}
