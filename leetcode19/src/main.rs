pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode {
        next: None,
        val
        }
    }
}

fn main() {
    println!("Hello, world!");
    let tear = Box::new(ListNode::new(1));
    let mut m = Box::new(ListNode::new(2));
    m.next = Some(tear);
    let mut head = Box::new(ListNode::new(3));
    head.next = Some(m);
    if let Some(node) =  {
        .
    }
    println!("{}", head.next.val)
    // let mut head = ListNode::new(32);
    // head.next = Some(Box::new(ListNode::new(48)));
}
