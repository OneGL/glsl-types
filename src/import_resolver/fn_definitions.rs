use super::fn_name_manager::FunctionNameManager;
use glsl::syntax::{FunctionDefinition, Identifier, StructSpecifier, TranslationUnit, TypeName};
use glsl::visitor::{HostMut, Visit, VisitorMut};
use std::path::PathBuf;

pub fn rename_functions_to_avoid_collisions(
  ast: &mut TranslationUnit,
  fn_name_manager: FunctionNameManager,
  file_path: &PathBuf,
) -> FnDefinitionVisitor {
  let mut visitor = FnDefinitionVisitor {
    file_path: file_path.clone(),
    fn_name_manager,
    fn_definitions: Vec::new(),
    struct_definitions: Vec::new(),
  };

  ast.visit_mut(&mut visitor);

  return visitor;
}

pub struct FnDefinitionVisitor {
  file_path: PathBuf,
  pub fn_name_manager: FunctionNameManager,
  pub fn_definitions: Vec<FunctionDefinition>,
  pub struct_definitions: Vec<StructSpecifier>,
}

impl VisitorMut for FnDefinitionVisitor {
  fn visit_function_definition(&mut self, function: &mut FunctionDefinition) -> Visit {
    let mut fn_name = function.prototype.name.to_string();
    fn_name = self.fn_name_manager.get_fn_name(&fn_name, &self.file_path);
    function.prototype.name = Identifier::new(fn_name).unwrap();

    self.fn_definitions.push(function.clone());

    Visit::Parent
  }

  fn visit_struct_specifier(&mut self, struct_specifier: &mut StructSpecifier) -> Visit {
    let mut struct_name = match &struct_specifier.name {
      Some(name) => name.to_string(),
      None => return Visit::Parent,
    };

    struct_name = self
      .fn_name_manager
      .get_fn_name(&struct_name, &self.file_path);

    struct_specifier.name = Some(TypeName::new(struct_name).unwrap());
    self.struct_definitions.push(struct_specifier.clone());

    Visit::Parent
  }
}
