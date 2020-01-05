pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {}



fn main() {
    let mut head = ListNode::new(0);
    let value = vec![1,2,3,4,5,6,7,8,9];
    let mut tail = Box::new(ListNode::new(value[1]));
    head.next = Some(tail);
    for i in value.iter() {
        tail = match tail.next {
            Some(x) => x,
            None => Box::new(ListNode::new(*i)),
        };
    };
    println!("second is {:?}", tail.val);
    let head = head.next;
    if let Some(x) = head {
        println!("head is {}", x.val);
    } else {
        println!("head is none.");
    }
}
