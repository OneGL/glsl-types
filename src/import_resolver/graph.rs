use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
pub struct Graph {
  pub adjacency_list: HashMap<PathBuf, Vec<PathBuf>>,
}

impl Graph {
  pub fn new() -> Self {
    Self {
      adjacency_list: HashMap::new(),
    }
  }

  pub fn add_edge(&mut self, node: PathBuf, edge: PathBuf) {
    self
      .adjacency_list
      .entry(node)
      .or_insert_with(Vec::new)
      .push(edge);
  }

  pub fn get_neighbors(&self, node: &PathBuf) -> Option<&Vec<PathBuf>> {
    self.adjacency_list.get(node)
  }

  fn dfs<'a>(
    &'a self,
    node: &'a PathBuf,
    visited: &mut HashMap<&'a PathBuf, bool>,
    recursive_stack: &mut HashMap<&'a PathBuf, bool>,
  ) -> bool {
    if recursive_stack.contains_key(node) {
      return true;
    }

    if visited.contains_key(node) {
      return false;
    }

    visited.insert(node, true);
    recursive_stack.insert(node, true);

    if let Some(neighbors) = self.get_neighbors(node) {
      for neighbor in neighbors {
        if self.dfs(neighbor, visited, recursive_stack) {
          return true;
        }
      }
    }

    recursive_stack.remove(node);

    return false;
  }

  pub fn has_cycle(&self) -> bool {
    let mut visited = HashMap::new();
    let mut recursive_stack = HashMap::new();

    for node in self.adjacency_list.keys() {
      if self.dfs(&node, &mut visited, &mut recursive_stack) {
        return true;
      }
    }

    return false;
  }
}
