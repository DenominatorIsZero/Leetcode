use crate::custom_error::LCError;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[tracing::instrument]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    #[tracing::instrument]
    fn from_i64(val: i64) -> Option<Box<ListNode>> {
        let mut digits = Vec::new();
        let mut n = val;
        while n > 9 {
            digits.push((n % 10) as i32);
            n /= 10;
        }
        digits.push(n as i32);
        ListNode::from_vec(digits)
    }

    #[tracing::instrument]
    pub fn from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
        Some(Box::new(Self {
            val: v[0],
            next: if v.len() > 1 {
                ListNode::from_vec(v[1..].to_vec())
            } else {
                None
            },
        }))
    }

    #[tracing::instrument]
    fn to_i64(&self) -> i64 {
        let mut sum = self.val as i64;
        let mut next = self.next.as_ref();
        let mut mult = 10;
        while next.is_some() {
            sum += next.as_ref().unwrap().val as i64 * mult;
            next = next.as_ref().unwrap().next.as_ref();
            mult *= 10;
        }
        sum
    }
}

#[tracing::instrument]
pub fn process(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> miette::Result<Option<Box<ListNode>>, LCError> {
    let mut n1 = l1.unwrap();
    let mut n2 = l2.unwrap();

    let mut v3 = n1.val + n2.val;
    let mut carry_over = v3 / 10;

    let mut head = Some(Box::new(ListNode::new(v3 % 10)));
    let mut n3 = head.as_mut().unwrap();

    while n1.next.is_some() || n2.next.is_some() {
        let v1 = if let Some(next) = n1.next.as_ref() {
            next.val
        } else {
            0
        };
        let v2 = if let Some(next) = n2.next.as_ref() {
            next.val
        } else {
            0
        };
        v3 = v1 + v2 + carry_over;
        carry_over = v3 / 10;

        n3.next = Some(Box::new(ListNode::new(v3 % 10)));
        n3 = n3.next.as_mut().unwrap();

        n1 = if let Some(next) = n1.next { next } else { n1 };
        n2 = if let Some(next) = n2.next { next } else { n2 };
    }
    if carry_over > 0 {
        n3.next = Some(Box::new(ListNode::new(carry_over)));
    }
    Ok(head)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[tracing::instrument]
    fn test_to_vec() -> miette::Result<()> {
        let l1 = ListNode::from_vec(vec![2, 4, 3]);
        assert_eq!(342, l1.unwrap().to_i64());
        Ok(())
    }

    #[rstest]
    #[case(
        ListNode::from_vec(vec![2, 4, 3]),
        ListNode::from_vec(vec![5,6,4]),
        ListNode::from_vec(vec![7,0,8]))]
    #[case(
        ListNode::from_vec(vec![9]),
        ListNode::from_vec(vec![1,9,9,9,9,9,9,9,9,9]),
        ListNode::from_vec(vec![0,0,0,0,0,0,0,0,0,0,1]))]
    #[tracing::instrument]
    fn test_process(
        #[case] l1: Option<Box<ListNode>>,
        #[case] l2: Option<Box<ListNode>>,
        #[case] expected: Option<Box<ListNode>>,
    ) -> miette::Result<()> {
        assert_eq!(expected, process(l1, l2)?);
        Ok(())
    }
}
