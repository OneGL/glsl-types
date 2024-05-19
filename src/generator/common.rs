use glsl::parser::Parse as _;
use glsl::syntax::{
  ShaderStage, SingleDeclaration, StorageQualifier, TypeQualifierSpec, TypeSpecifierNonArray,
};
use glsl::visitor::{Host, Visit, Visitor};

pub fn capitalize_first_letter(s: &str) -> String {
  s.chars().next().unwrap().to_uppercase().collect::<String>() + &s[1..]
}

const FRAGMENT_SHADER_EXTENSIONS: [&str; 2] = ["fs", "frag"];
const VERTEX_SHADER_EXTENSIONS: [&str; 2] = ["vs", "vert"];

#[derive(Clone, Debug)]
enum ShaderType {
  Fragment,
  Vertex,
}

fn get_shader_type(file_path: &std::path::PathBuf) -> ShaderType {
  let extension = file_path.extension().unwrap().to_str().unwrap();
  if FRAGMENT_SHADER_EXTENSIONS.contains(&extension) {
    return ShaderType::Fragment;
  } else if VERTEX_SHADER_EXTENSIONS.contains(&extension) {
    return ShaderType::Vertex;
  }

  panic!("Unknown shader type");
}
#[derive(Clone, Debug)]
pub struct Uniform {
  pub name: String,
  pub uniform_type: TypeSpecifierNonArray,
}

#[derive(Clone, Debug)]
pub struct Varying {
  pub name: String,
  pub varying_type: TypeSpecifierNonArray,
}

#[derive(Clone, Debug)]
pub struct ShaderData {
  pub uniforms: Vec<Uniform>,
  pub varyings: Vec<Varying>,
  shader_type: ShaderType,
}

impl Visitor for ShaderData {
  fn visit_single_declaration(&mut self, declaration: &SingleDeclaration) -> Visit {
    if let Some(name) = &declaration.name {
      if let Some(type_qualifier) = &declaration.ty.qualifier {
        type_qualifier
          .qualifiers
          .clone()
          .into_iter()
          .for_each(|qualifier| {
            if let TypeQualifierSpec::Storage(storage_qualifier) = qualifier {
              if storage_qualifier == StorageQualifier::Uniform {
                self.uniforms.push(Uniform {
                  name: name.as_str().to_string(),
                  uniform_type: declaration.ty.ty.ty.clone(),
                });
              }

              match self.shader_type {
                ShaderType::Fragment => {
                  if storage_qualifier == StorageQualifier::In {
                    self.varyings.push(Varying {
                      name: name.as_str().to_string(),
                      varying_type: declaration.ty.ty.ty.clone(),
                    });
                  }
                }
                ShaderType::Vertex => {
                  if storage_qualifier == StorageQualifier::Attribute {
                    self.varyings.push(Varying {
                      name: name.as_str().to_string(),
                      varying_type: declaration.ty.ty.ty.clone(),
                    });
                  }
                }
              }
            }
          });
      }
    }

    Visit::Parent
  }
}

pub fn extract_shader_data(file: &String, file_path: &std::path::PathBuf) -> ShaderData {
  let stage = ShaderStage::parse(file);

  let mut shader_data = ShaderData {
    uniforms: Vec::new(),
    varyings: Vec::new(),
    shader_type: get_shader_type(file_path),
  };

  match stage {
    Ok(stage) => stage.visit(&mut shader_data),
    Err(e) => {
      println!("Error parsing the shader: {}", e);
    }
  }

  return shader_data;
}
