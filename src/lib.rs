use std::{
    collections::{HashMap, VecDeque},
    error::Error,
};
use log::info;

pub type PatternMap = HashMap<String, String>;

struct Node {
    children: [Option<Box<Node>>; 256],
    fail_link: Option<*mut Node>,
    label: Option<String>,
}

impl Node {
    fn new() -> Self {
        Node {
            children: std::array::from_fn(|_| None),
            fail_link: None,
            label: None,
        }
    }
}

pub struct LabelMaker {
    root: Box<Node>,
}

impl LabelMaker {
    fn new() -> Self {
        Self {
            root: Box::new(Node::new()),
        }
    }

    pub fn build(lookup: PatternMap) -> Self {
        let mut labeler = LabelMaker::new();
        for (pattern, label) in lookup {
            labeler.insert(pattern.as_str(), label.as_str()).unwrap_or_else(|e| {
                panic!("Failed to insert pattern: {}, label: {}. Error: {}", pattern, label, e)
            });
        }
        labeler.finalize();
        labeler
    }

    fn insert(&mut self, pattern: &str, label: &str) -> Result<(), Box<dyn Error>> {
        info!("Inserting pattern: {}, label: {}", pattern, label);
        let mut node = &mut *self.root;
        for &byte in pattern.as_bytes() {
            let index = byte as usize;
            if node.children[index].is_none() {
                node.children[index] = Some(Box::new(Node::new()));
            }
            node = node.children[index].as_mut().unwrap();
        }

        if let Some(existing_label) = &node.label {
            if existing_label != label {
                return Err(format!(
                    "Pattern '{}' is already labeled for: {}, conflicts with existing pattern '{}'",
                    pattern, label, existing_label
                )
                .into());
            } 
        }

        node.label = Some(label.to_string());

        Ok(())
    }

    pub fn categorize(&self, text: &str) -> Option<String> {
        let mut node = &*self.root;
        let mut longest_match_label: Option<String> = None;

        for &byte in text.as_bytes() {
            let index = byte as usize;

            while node.children[index].is_none() && node as *const _ != &*self.root {
                node = unsafe { &*node.fail_link.unwrap() };
            }

            if let Some(ref child) = node.children[index] {
                node = child;
            }

            if let Some(ref label) = node.label {
                longest_match_label = Some(label.clone());
            }
        }

        longest_match_label
    }

    fn finalize(&mut self) {
        let root_ptr: *mut Node = &mut *self.root;
        let mut queue = VecDeque::new();

        for i in 0..256 {
            if let Some(ref mut child) = self.root.children[i] {
                child.fail_link = Some(root_ptr);
                queue.push_back(child.as_mut() as *mut Node);
            }
        }

        while let Some(current) = queue.pop_front() {
            for i in 0..256 {
                unsafe {
                    if let Some(ref mut child) = (*current).children[i] {
                        let mut fail = (*current).fail_link.unwrap();

                        while fail != root_ptr && (*fail).children[i].is_none() {
                            if let Some(new_fail) = (*fail).fail_link {
                                fail = new_fail;
                            } else {
                                fail = root_ptr;
                                break;
                            }
                        }

                        if let Some(ref mut sibling) = (*fail).children[i] {
                            child.fail_link = Some(sibling.as_mut() as *mut Node);
                        } else {
                            child.fail_link = Some(root_ptr);
                        }

                        queue.push_back(child.as_mut() as *mut Node);
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod test {
    use test_log::test;
    use std::collections::HashMap;

    #[test]
    fn test_should_allow_happy_insert() {
        let categories: super::PatternMap = HashMap::from([
            ("Tyrannosaurus rex".to_string(), "Therapod".to_string()),
            ("Velociraptor".to_string(), "Therapod".to_string()),
            ("Brachiosaurus".to_string(), "Saurapod".to_string()),
            ("Patagotitan".to_string(), "Saurapod".to_string()),
        ]);
        let categorosaurus = super::LabelMaker::build(categories);

        let result = categorosaurus.categorize("Tyrannosaurus rex").unwrap();
        assert_eq!(result, "Therapod");
    }

    #[test]
    fn test_should_find_longest_pattern_and_return_label() {
        let categories: super::PatternMap = HashMap::from([
            ("triceratop".to_string(), "Single".to_string()),
            ("triceratops".to_string(), "Many".to_string()),
        ]);

        let categorosaurus = super::LabelMaker::build(categories);

        let text = "triceratops are a group of herbivorous ceratopsid dinosaurs";
        let result = categorosaurus.categorize(text).unwrap();
        assert_eq!(result, "Many");
    }
}
