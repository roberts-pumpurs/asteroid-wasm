pub const F_SHADER: &str = r#"
    varying lowp vec4 vColor;

    void main() {
      gl_FragColor = vColor;
    }
"#;
