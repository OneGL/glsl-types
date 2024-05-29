use crate::console_log;
use crate::utils::get_shader_type::ShaderType;
use glsl::parser::Parse as _;
use glsl::syntax::{
  ShaderStage, SingleDeclaration, StorageQualifier, TypeQualifierSpec, TypeSpecifierNonArray,
};
use glsl::visitor::{Host, Visit, Visitor};

pub fn capitalize_first_letter(s: &str) -> String {
  s.chars().next().unwrap().to_uppercase().collect::<String>() + &s[1..]
}

#[derive(Clone, Debug)]
pub struct TypedVariable {
  pub identifier: String,
  pub type_label: TypeSpecifierNonArray,
}

#[derive(Clone, Debug)]
pub struct ShaderData {
  pub uniforms: Vec<TypedVariable>,
  pub ins: Vec<TypedVariable>,
  pub outs: Vec<TypedVariable>,
  pub shader_type: ShaderType,
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
                self.uniforms.push(TypedVariable {
                  identifier: name.as_str().to_string(),
                  type_label: declaration.ty.ty.ty.clone(),
                });
              }

              match self.shader_type {
                _ => {
                  if storage_qualifier == StorageQualifier::Out {
                    self.outs.push(TypedVariable {
                      identifier: name.as_str().to_string(),
                      type_label: declaration.ty.ty.ty.clone(),
                    });
                  }
                  if storage_qualifier == StorageQualifier::In {
                    self.ins.push(TypedVariable {
                      identifier: name.as_str().to_string(),
                      type_label: declaration.ty.ty.ty.clone(),
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

pub fn extract_shader_data(file: &String, shader_type: ShaderType) -> ShaderData {
  let stage: Result<glsl::syntax::TranslationUnit, glsl::parser::ParseError> =
    ShaderStage::parse(file);

  let mut shader_data = ShaderData {
    uniforms: Vec::new(),
    ins: Vec::new(),
    outs: Vec::new(),
    shader_type,
  };

  match stage {
    Ok(stage) => stage.visit(&mut shader_data),
    Err(e) => {
      console_log(&format!("Error parsing the shader: {}", e));
    }
  }

  return shader_data;
}
