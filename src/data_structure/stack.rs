pub struct Node {
    pub element: u8,
    pub next: Option<Box<Node>>
}

pub struct Stack {
    pub top: Option<Box<Node>>,
    pub length: u8
}

impl Stack {
    pub fn length(&self) -> u8 {
        self.length
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }   

    pub fn push(&mut self, data: u8) {
        let temp = Box::new(Node {
            element: data,
            next: self.top.take()
        });
        self.top = Some(temp);
        self.length += 1;
    }

    pub fn pop(&mut self) {
        if self.is_empty() {
            eprintln!("Error: Stack is empty!")

        }

        if let Some(top_boxed) = self.top.take() {
            let top_node = *top_boxed;
            self.top = top_node.next;
            self.length -=1;
        }
    }

    pub fn show_top(&self) {
        if let Some(ref top) = &self.top {
            println!("{}", top.element);
        } else {
            println!("Stack is empty")
        }
    }

    pub fn peek(&self) {
        let mut current = &self.top;

        while let Some(node) = current {
            println!("Element: {}", node.element);
            current = &node.next;
        }
    }

}