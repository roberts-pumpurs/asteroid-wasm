use web_sys::WebGlProgram;
use web_sys::WebGlShader;
use web_sys::{console, WebGlRenderingContext as GL};

pub fn console_log(to_log: &str) {
    let array = js_sys::Array::new();
    array.push(&"WASM:".into());
    array.push(&to_log.into());
    unsafe {
        console::log(&array);
    }
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn link_program(gl: &GL, vert_source: &str, frag_source: &str) -> Result<WebGlProgram, String> {
    console_log("Compilign shader");
    let vertex_shader = compile_shader(gl, GL::VERTEX_SHADER, vert_source).unwrap();
    console_log("Compilign shader");
    let fragment_shader = compile_shader(gl, GL::FRAGMENT_SHADER, frag_source).unwrap();
    let shader_program = gl
        .create_program()
        .ok_or_else(|| String::from("Error creating program"))?;
    gl.attach_shader(&shader_program, &vertex_shader);
    gl.attach_shader(&shader_program, &fragment_shader);
    gl.link_program(&shader_program);

    if !gl
        .get_program_parameter(&shader_program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        console_log("Unable to initialize shader program!");
        Err(gl
            .get_program_info_log(&shader_program)
            .unwrap_or_else(|| String::from("Unknown error occurred when creating program object")))
    } else {
        Ok(shader_program)
    }
}


fn compile_shader(context: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    // TODO LEFT OFF  "Initializing the shaders" https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial/Adding_2D_content_to_a_WebGL_context
    // This is the 'loadShader' function
    let shader: WebGlShader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);
    if context
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        console_log("Error occurred during compilation");
        // context.delete_shader(Some(&shader));
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}
