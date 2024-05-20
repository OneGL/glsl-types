use std::collections::HashMap;
use std::path::PathBuf;

use super::fn_name_manager::FunctionNameManager;
use glsl::syntax::{Expr, FunIdentifier, Identifier, TranslationUnit};
use glsl::visitor::{HostMut, Visit, VisitorMut};

pub fn rename_imported_function_calls(
  ast: &mut TranslationUnit,
  fn_name_manager: FunctionNameManager,
  imports: HashMap<String, PathBuf>,
  file_path: PathBuf,
) -> FunctionNameManager {
  let mut visitor = FnCallVisitor {
    fn_name_manager,
    imports,
    file_path,
  };
  ast.visit_mut(&mut visitor);

  return visitor.fn_name_manager.clone();
}

struct FnCallVisitor {
  fn_name_manager: FunctionNameManager,
  imports: HashMap<String, PathBuf>,
  file_path: PathBuf,
}

impl FnCallVisitor {
  fn visit_function_call(&mut self, call: &mut FunIdentifier, args: &mut Vec<Expr>) {
    match call {
      FunIdentifier::Expr(expr) => {
        if let Expr::Dot(box_expr, imported_fn) = expr.as_ref() {
          if let Expr::Variable(import_identifier) = box_expr.as_ref() {
            let import_path = self.imports.get(import_identifier.as_str()).unwrap();
            let fn_name = self
              .fn_name_manager
              .get_fn_name(imported_fn.as_str(), import_path);
            let identifier = Identifier::new(fn_name).unwrap();
            *call = FunIdentifier::Identifier(identifier);
          }
        }
      }
      FunIdentifier::Identifier(identifier) => {
        let fn_name = self
          .fn_name_manager
          .get_fn_name(identifier.as_str(), &self.file_path);
        *identifier = Identifier::new(fn_name).unwrap();
      }
    }

    for arg in args {
      if let Expr::FunCall(call, args) = arg {
        self.visit_function_call(call, args);
      }
    }
  }
}

impl VisitorMut for FnCallVisitor {
  fn visit_expr(&mut self, expr: &mut Expr) -> Visit {
    if let Expr::FunCall(call, args) = expr {
      self.visit_function_call(call, args);
    }

    Visit::Parent
  }
}
