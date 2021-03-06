//Definition for singly-linked list.
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
impl crate::solutions::Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // if let boxedhead = head {
        //     let test = boxedhead;
        // }
        return head;
    }
}
//1. how to un-wrap,
// 2. recursive sort
// 3. recursive merge