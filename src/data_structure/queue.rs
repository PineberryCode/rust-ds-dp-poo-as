/*
 * "Some" is the type of variable accepted by "Option".
 * Note: Option is a enum 
*/
#[derive(Clone, Debug)]
pub struct PizzaBox {
    pub pizza: String,
    pub next: Option<Box<PizzaBox>>
}

pub struct Queue {
    pub front: Option<Box<PizzaBox>>,
    pub rear: Option<Box<PizzaBox>>,
    pub length: u8
}

impl Queue {
    pub fn length(&self) -> u8 {
        self.length
    }
    
    fn is_empty(&self) -> bool {
        self.length == 0 
    }

    pub fn enqueue(&mut self, type_pizza: String) {
        let pizza_box = Box::new(PizzaBox {
            pizza: type_pizza,
            next: None
        });
        
        if self.is_empty() {
            self.front = Some(pizza_box.clone());
        } else {
            let mut current = &mut self.front;
            while let Some(node_next) = current {
                if node_next.next.is_none() {
                    node_next.next = Some(pizza_box.clone());
                    break;
                }
                current = &mut node_next.next;
            }
        }

        self.rear = Some(pizza_box);
        self.length += 1;
        println!("Front {:?}", self.front);
        println!("rear {:?}", self.rear);
        println!("Lenght02 {}", self.length());
    }

    pub fn show_front(&self) {
        if let Some(ref front) = &self.front {
            println!("Front: {}", front.pizza);
        } else {
            println!("front is empty")
        }
    }

    pub fn show_rear(&self) {
        if let Some(ref rear) = &self.rear {
            println!("Rear: {}", rear.pizza);
        } else {
            println!("rear is empty")
        }
    }

    pub fn dequeue(&mut self) {
        if self.is_empty() {
            eprintln!("Queue is empty!");
            return;
        }

        if let Some(node) = &mut self.front {
            self.front = node.next.take();
            println!("{:?}", self.front);
            self.length -= 1;
        }
    }

    pub fn peek(&self) {
        if self.is_empty() {
            eprintln!("Queue is empty");
        }

        let mut current = &self.front;
        while let Some(node) = current {
            println!("Element: {}", node.pizza);
            current = &node.next;
        }
    }

}