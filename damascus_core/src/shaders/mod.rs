pub fn ray_march_shader() -> String {
    let mut shader_source: String = "".to_string();
    for line in include_str!("./ray_march.wgsl").split("\n") {
        if line.starts_with("#include") {
            let include_file = line.split("#include").collect::<Vec<&str>>()[1]
                .trim()
                .split("\"")
                .collect::<Vec<&str>>()[1];

            shader_source += include_str!(include_file);
        } else {
            shader_source += &(line.to_owned() + "\n");
        }
    }
    return shader_source;
}
