pub const TEAPOT_VERTEX_SHADER_SRC: &str = r#"
    #version 150

    in vec3 position;
    in vec3 normal;

    out vec3 v_position;
    out vec3 v_normal;

    uniform mat4 perspective;
    uniform mat4 view;
    uniform mat4 model;

    void main() {
        mat4 modelview = view * model;
        gl_Position = perspective * modelview * vec4(position, 1.0);

        v_position = gl_Position.xyz / gl_Position.w;
        v_normal = transpose(inverse(mat3(modelview))) * normal;
    }
"#;

pub const TEAPOT_FRAGMENT_SHADER_SRC: &str = r#"
    #version 140

    in vec3 v_position;
    in vec3 v_normal;

    out vec4 color;

    uniform vec3 light_dir;

    const vec3 diffuse_color = vec3(1.0, 1.0, 0.0);
    const vec3 ambient_color = vec3(0.2, 0.2, 0.0);
    const vec3 specular_color = vec3(1.0, 1.0, 1.0);

    void main() {
        float diffuse = max(dot(normalize(v_normal), -normalize(light_dir)), 0.0);

        vec3 camera_view_dir = normalize(-v_position);
        vec3 half_direction = normalize(-normalize(light_dir) + camera_view_dir);
        float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 32.0);

        color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
    }
"#;

pub const FLOOR_VERTEX_SHADER_SRC: &str = r#"
    #version 150

    in vec3 position;

    uniform mat4 perspective;
    uniform mat4 view;
    uniform mat4 model;

    void main() {
        mat4 modelview = view * model;
        gl_Position = perspective * modelview * vec4(position, 1.0);
    }
"#;

pub const FLOOR_FRAGMENT_SHADER_SRC: &str = r#"
    #version 140

    out vec4 color;

    const vec3 diffuse_color = vec3(0.3, 0.1, 0.1);

    void main() {
        color = vec4(diffuse_color, 1.0);
    }
"#;
