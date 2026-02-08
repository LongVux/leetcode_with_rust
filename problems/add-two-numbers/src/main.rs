#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut carry = 0;

        let mut head: Option<Box<ListNode>> = None;
        let mut tail = &mut head;

        while l1.is_some() || l2.is_some() {
            let mut sum = carry;
            let mut node;

            if let Some(mut n1) = l1.take() {
                sum += n1.val;
                l1 = n1.next.take();
                node = n1;

                if let Some(mut n2) = l2.take() {
                    sum += n2.val;
                    l2 = n2.next.take();
                }
            } else {
                let mut n2 = l2.take().unwrap();
                sum += n2.val;
                l2 = n2.next.take();
                node = n2;
            }

            carry = sum / 10;
            node.val = sum % 10;
            node.next = None;
            *tail = Some(node);
            tail = &mut tail.as_mut().unwrap().next;
        }

        if carry != 0 {
            *tail = Some(Box::new(ListNode::new(carry)));
        }

        head
    }
}

fn list_from_vec(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy;
    for &v in vals {
        tail.next = Some(Box::new(ListNode::new(v)));
        tail = tail.next.as_mut().unwrap();
    }
    dummy.next
}

fn list_to_vec(mut node: Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = Vec::new();
    while let Some(n) = node {
        out.push(n.val);
        node = n.next;
    }
    out
}

fn main() {
    let l1 = list_from_vec(&[2, 4, 3]);
    let l2 = list_from_vec(&[5, 6, 4]);
    let sum = Solution::add_two_numbers(l1, l2);
    let out = list_to_vec(sum);
    println!("result = {:?}", out);
    assert_eq!(out, vec![7, 0, 8]);

    let l1 = list_from_vec(&[0]);
    let l2 = list_from_vec(&[0]);
    let sum = Solution::add_two_numbers(l1, l2);
    let out = list_to_vec(sum);
    assert_eq!(out, vec![0]);
}
