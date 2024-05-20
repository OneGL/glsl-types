use std::collections::HashMap;

#[derive(Debug)]
pub struct Graph {
  pub adjacency_list: HashMap<String, Vec<String>>,
}

impl Graph {
  pub fn new() -> Self {
    Self {
      adjacency_list: HashMap::new(),
    }
  }

  pub fn add_edge(&mut self, node: String, edge: String) {
    self
      .adjacency_list
      .entry(node)
      .or_insert_with(Vec::new)
      .push(edge);
  }

  pub fn get_neighbors(&self, node: &str) -> Option<&Vec<String>> {
    self.adjacency_list.get(node)
  }

  fn dfs(
    &self,
    node: &str,
    visited: &mut HashMap<String, bool>,
    recursive_stack: &mut HashMap<String, bool>,
  ) -> bool {
    if recursive_stack.contains_key(node) {
      return true;
    }

    if visited.contains_key(node) {
      return false;
    }

    visited.insert(node.to_string(), true);
    recursive_stack.insert(node.to_string(), true);

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
      if self.dfs(node, &mut visited, &mut recursive_stack) {
        return true;
      }
    }

    return false;
  }
}
