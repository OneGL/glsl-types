use crate::uniforms::Uniform;
use glsl::parser::Parse as _;
use glsl::syntax::{ShaderStage, SingleDeclaration, StorageQualifier, TypeQualifierSpec};
use glsl::visitor::{Host, Visit, Visitor};

pub fn capitalize_first_letter(s: &str) -> String {
  s.chars().next().unwrap().to_uppercase().collect::<String>() + &s[1..]
}

struct UniformVisitor {
  uniforms: Vec<Uniform>,
}

impl Visitor for UniformVisitor {
  fn visit_single_declaration(&mut self, declaration: &SingleDeclaration) -> Visit {
    if let Some(name) = &declaration.name {
      if let Some(type_qualifier) = &declaration.ty.qualifier {
        type_qualifier
          .qualifiers
          .clone()
          .into_iter()
          .for_each(|qualifier| {
            if let TypeQualifierSpec::Storage(StorageQualifier::Uniform) = qualifier {
              self.uniforms.push(Uniform {
                name: name.as_str().to_string(),
                uniform_type: declaration.ty.ty.ty.clone(),
              });
            }
          });
      }
    }

    Visit::Parent
  }
}

pub fn extract_uniforms(file: &String) -> Vec<Uniform> {
  let stage = ShaderStage::parse(file);

  let mut uniform_visitor = UniformVisitor {
    uniforms: Vec::new(),
  };

  match stage {
    Ok(stage) => stage.visit(&mut uniform_visitor),
    Err(e) => {
      println!("Error parsing the shader: {}", e);
    }
  }

  return uniform_visitor.uniforms;
}
