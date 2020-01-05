pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

// impl ListNode {
//     fn new(val: i32) -> Self {
//         ListNode {
//         next: None,
//         val
//         }
//     }
// }

fn generate_head()->Option<Box<ListNode>> {
    let mut data = vec![1,2,3,4,5,6,7,8,9];
    let mut head = None;
    data.reverse();
    for i in data.iter() {
        head = Some(Box::new(ListNode {
            val: *i,
            next: head
        }));
    }
    // println!("{}",tail.unwrap().val);
    head
}

fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode {val:0, next: head}));
        let mut len =-1;
        {
            let mut p = head.as_ref();
            while p.is_some() {
                len +=1;
                p = p.unwrap().next.as_ref();
            }
        }
        len = len - n ;
        {
            let mut p = head.as_mut();
            for _ in 0..len {
                p = p.unwrap().next.as_mut();
            }
            let next = p.as_mut().unwrap().next.as_mut().unwrap().next.take();
            p.as_mut().unwrap().next = next; 
        }
        head.unwrap().next
}

fn main() {
    let mut head = generate_head();
    // println!("{}",head.unwrap().val);
    /* let n = 2;
    let mut p = head.as_ref();
    for _ in 0..n {
        p = p.unwrap().next.as_ref();
    }
    let mut p2 = head.as_mut();
    while p.is_some() {
        p = p.unwrap().next.as_ref();
        p2 = p2.unwrap().next.as_mut();
    }
    let next = p2.as_mut().unwrap().next.as_mut().unwrap().next.take();
    p2.as_mut().unwrap().next = next; */
    let n = 1;
    let mut len = 0;
    {
        let mut p = head.as_ref();
        while p.is_some() {
            len +=1;
            p = p.unwrap().next.as_ref();
        }
    }
    println!("{}", &len);
    len = len - n -1;
    {
        let mut p = head.as_mut();
        for _ in 0..len {
            p = p.unwrap().next.as_mut();
        }
        let next = p.as_mut().unwrap().next.as_mut().unwrap().next.take();
        p.as_mut().unwrap().next = next; 
    }
    

}
