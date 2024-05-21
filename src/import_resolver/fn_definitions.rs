use super::fn_name_manager::FunctionNameManager;
use glsl::syntax::{FunctionDefinition, Identifier, TranslationUnit};
use glsl::visitor::{HostMut, Visit, VisitorMut};
use std::path::PathBuf;

pub fn rename_functions_to_avoid_collisions(
  ast: &mut TranslationUnit,
  fn_name_manager: FunctionNameManager,
  file_path: &PathBuf,
) -> (FunctionNameManager, Vec<FunctionDefinition>) {
  let mut visitor = FnDefinitionVisitor {
    file_path: file_path.clone(),
    fn_name_manager,
    fn_definitions: Vec::new(),
  };

  ast.visit_mut(&mut visitor);

  return (visitor.fn_name_manager.clone(), visitor.fn_definitions);
}

struct FnDefinitionVisitor {
  file_path: PathBuf,
  fn_name_manager: FunctionNameManager,
  fn_definitions: Vec<FunctionDefinition>,
}

impl VisitorMut for FnDefinitionVisitor {
  fn visit_function_definition(&mut self, function: &mut FunctionDefinition) -> Visit {
    let mut fn_name = function.prototype.name.to_string();
    fn_name = self.fn_name_manager.get_fn_name(&fn_name, &self.file_path);
    function.prototype.name = Identifier::new(fn_name).unwrap();

    self.fn_definitions.push(function.clone());

    Visit::Parent
  }
}
