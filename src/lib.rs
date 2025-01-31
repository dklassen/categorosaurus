use std::{
    collections::VecDeque,
    error::Error,
};

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
    _failure_links_built: bool,
}

impl LabelMaker {
    fn new() -> Self {
        Self {
            root: Box::new(Node::new()),
            _failure_links_built: false,
        }
    }

    pub fn insert(&mut self, pattern: &str, label: &str) -> Result<(), Box<dyn Error>> {
        if self._failure_links_built {
            return Err("Cannot insert after finalizing".into());
        }

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
        if !self._failure_links_built {
            panic!("Failure links not built yet. Call finalize() first.");
        }

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


    pub fn finalize(&mut self) {
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

        self._failure_links_built = true;
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_should_allow_happy_insert() {
        let mut labeler = super::LabelMaker::new();
        labeler.insert("Tyrannosaurus rex", "Therapod").unwrap();
        labeler.insert("Velociraptor", "Therapod").unwrap();
        labeler.insert("Brachiosaurus", "Saurapod").unwrap();
        labeler.insert("Patagotitan", "Saurapod").unwrap();
        labeler.finalize();
    }

    #[test]
    fn test_should_error_on_insert_after_finalize() {
        let mut labeler = super::LabelMaker::new();
        labeler.insert("Tyrannosaurus rex", "Therapod").unwrap();
        labeler.insert("Velociraptor", "Therapod").unwrap();
        labeler.insert("Brachiosaurus", "Saurapod").unwrap();
        labeler.insert("Patagotitan", "Saurapod").unwrap();
        labeler.finalize();

        let result = labeler.insert("Triceratops", "Ceratopsian");
        assert!(result.is_err());
    }

    #[test]
    #[should_panic]
    fn test_should_panic_if_try_to_categorize_before_finalizing() {
        let labeler = super::LabelMaker::new();
        labeler.categorize("Tyrannosaurus rex").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_should_detect_conflict_due_to_duplicate_labels_for_same_pattern() {
        let mut labeler = super::LabelMaker::new();
        let test_cases = vec![
            ("rex", "T"),
            ("rex", "Not-T")
        ];

        for (pattern, label) in test_cases {
            labeler.insert(pattern, label).unwrap();
        }
    }

    #[test]
    fn test_should_find_longest_pattern_and_return_label() {
        let mut labeler = super::LabelMaker::new();
        labeler.insert("triceratop", "Single").unwrap();
        labeler.insert("triceratops", "Many").unwrap();
        labeler.finalize();

        let text = "triceratops are a group of herbivorous ceratopsid dinosaurs";
        let result = labeler.categorize(text).unwrap();
        println!("Result: {}", result);

        assert_eq!(result, "Many");
    }
}

