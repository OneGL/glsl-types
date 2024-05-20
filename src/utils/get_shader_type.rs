pub const FRAGMENT_SHADER_EXTENSIONS: [&str; 2] = ["frag", "fs"];
pub const VERTEX_SHADER_EXTENSIONS: [&str; 2] = ["vert", "vs"];

#[derive(Clone, Debug)]
pub enum ShaderType {
  Fragment,
  Vertex,
}

pub fn get_shader_type(file_path: &std::path::PathBuf) -> Option<ShaderType> {
  let extension = file_path.extension().unwrap().to_str().unwrap();

  if FRAGMENT_SHADER_EXTENSIONS.contains(&extension) {
    return Some(ShaderType::Fragment);
  } else if VERTEX_SHADER_EXTENSIONS.contains(&extension) {
    return Some(ShaderType::Vertex);
  }

  return None;
}
