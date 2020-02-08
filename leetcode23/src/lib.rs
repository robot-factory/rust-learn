#[allow(dead_code)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

    pub fn from_vec(list: Vec<i32>) -> Option<Box<Self>> {
        let mut head = None;
        for val in list.iter().rev() {
            head = Some(Box::new(ListNode {
                val: *val,
                next: head,
            }));
        }
        head
    }
}
// pub fn generate_ListNode(v: Vec<i32>) -> Option<Box<ListNode>> {}
#[derive(Debug)]
struct Solution {}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut val_vec: Vec<i32> = lists
            .iter()
            .map(|ln| {
                let mut v: Vec<i32> = vec![];
                let mut n = ln;

                while let Some(bn) = n {
                    v.push(bn.val);
                    n = &bn.next;
                }
                v
            })
            .flatten()
            .collect();
        val_vec.sort_unstable();
        ListNode::from_vec(val_vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_a_node() {
        let node = ListNode::new(1);
        assert_eq!(node.val, 1);
    }

    #[test]
    fn new_list_from_vec() {
        let v = vec![1, 2, 3];
        let n = ListNode::from_vec(v);
        let n = n.unwrap();
        assert_eq!(n.val, 1);
        let n = n.next.unwrap();
        assert_eq!(n.val, 2);
    }
}
