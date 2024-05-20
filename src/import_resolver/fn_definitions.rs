use super::fn_name_manager::FunctionNameManager;
use glsl::syntax::{Identifier, TranslationUnit};
use glsl::visitor::{HostMut, Visit, VisitorMut};
use std::path::PathBuf;

pub fn rename_functions_to_avoid_collisions(
  ast: &mut TranslationUnit,
  fn_name_manager: FunctionNameManager,
  file_path: &PathBuf,
) -> FunctionNameManager {
  let mut visitor = FnDefinitionVisitor {
    file_path: file_path.clone(),
    fn_name_manager,
  };

  ast.visit_mut(&mut visitor);

  return visitor.fn_name_manager.clone();
}

struct FnDefinitionVisitor {
  file_path: PathBuf,
  fn_name_manager: FunctionNameManager,
}

impl VisitorMut for FnDefinitionVisitor {
  fn visit_function_definition(
    &mut self,
    function: &mut glsl::syntax::FunctionDefinition,
  ) -> Visit {
    let mut fn_name = function.prototype.name.to_string();
    fn_name = self.fn_name_manager.get_fn_name(&fn_name, &self.file_path);
    function.prototype.name = Identifier::new(fn_name).unwrap();

    Visit::Parent
  }
}
