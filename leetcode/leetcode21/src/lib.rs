pub struct Solution();

pub struct List {
    pub head: Option<Box<ListNode>>,
    pub tail: Option<Box<ListNode>>,
}

impl List {
    pub fn new() -> List {
        let new_node = ListNode::new(0);
        List {
            head: Some(Box::new(new_node.clone())),
            tail: Some(Box::new(new_node.clone())),
        }
    }

    pub fn push(&mut self, val: i32) {
        let new_node = ListNode::new(val);
        let mut tail = self.tail.take().unwrap();
        tail.next = Some(Box::new(new_node));
        self.tail = tail.next;
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
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
impl Solution {
    pub fn generate_list(tail: Option<Box<ListNode>>, mut data: Vec<i32>) -> Option<Box<ListNode>> {
        let mut result = tail;
        data.reverse();
        for el in data.iter() {
            result = Some(Box::new(ListNode {
                val: *el,
                next: result,
            }));
        }
        result
    }

    pub fn into_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        while head.is_some() {
            let node = head.unwrap();
            result.push(node.val);
            head = node.next;
        }
        result
    }

    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut stack: Vec<i32> = vec![];
        loop {
            if l1.is_none() && l2.is_some() {
                return Self::generate_list(l2, stack);
            }
            if l1.is_some() && l2.is_none() {
                return Self::generate_list(l1, stack);
            }
            if l1.is_none() && l2.is_none() {
                return Self::generate_list(None, stack);
            }
            let n1 = l1.unwrap();
            let n2 = l2.unwrap();
            if n1.val < n2.val {
                stack.push(n1.val);
                l1 = n1.next;
                l2 = Some(n2);
            } else {
                stack.push(n2.val);
                l1 = Some(n1);
                l2 = n2.next;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_vec() {
        let mut l1 = Some(Box::new(ListNode {val:1,next:Some(Box::new(ListNode::new(2)))}));
        assert_eq!(Solution::into_vec(l1),[1,2]);
    }

    #[test]
    fn generate_list() {
        let mut l1 = Solution::generate_list(None, vec![1,2,3]);
        assert_eq!(Solution::into_vec(l1),[1,2,3]);
    }

    #[test]
    fn push() {
        let mut list1 = Solution::generate_list(None, vec![1, 2, 4]);
        let mut list2 = Solution::generate_list(None, vec![1, 3, 4]);
        // list.push(1);
        let mut result = Solution::merge_two_lists(list1, list2);
        assert_eq!(Solution::into_vec(result), [1, 1, 2, 3, 4, 4]);
    }
}
