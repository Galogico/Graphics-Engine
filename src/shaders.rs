
    pub const vertex_shader_src: &str = r#"
            #version 140

            in vec2 position;
            in vec2 tex_coords;
            out vec2 v_tex_coords;

            uniform mat4 matrix;

            void main() {
                v_tex_coords = tex_coords;
                gl_Position = matrix * vec4(position, 0.0, 1.0);
            }
        "#;

    pub const fragment_shader_src: &str = r#"
        #version 140
        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;