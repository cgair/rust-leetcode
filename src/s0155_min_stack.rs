/**
 * [155] Min Stack
 *
 *
 * Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.
 *
 *
 * push(x) -- Push element x onto stack.
 *
 *
 * pop() -- Removes the element on top of the stack.
 *
 *
 * top() -- Get the top element.
 *
 *
 * getMin() -- Retrieve the minimum element in the stack.
 *
 *
 *
 *
 * Example:
 *
 * MinStack minStack = new MinStack();
 * minStack.push(-2);
 * minStack.push(0);
 * minStack.push(-3);
 * minStack.getMin();   --> Returns -3.
 * minStack.pop();
 * minStack.top();      --> Returns 0.
 * minStack.getMin();   --> Returns -2.
 *
 *
 */

pub struct Solution {}

pub struct MinStack {
    pub stack: Vec<i32>,
    pub min: Vec<i32>   // min: i32
} 

impl MinStack {
    pub fn new() -> Self {
        stack: Vec::new(),
        min: Vec::new()
        // min: i32::MAX
    }

    pub fn push(&mut self, item: i32) {
        self.stack.push(item);
        if self.min.is_empty() {
            min.push(item);
        }else {
            let min_top = *self.min.last().unwrap();
            if item < min_top {
                self.min.push(item);
            }else {
                self.min.push(min_top);
            }
        }
    }

    pub fn pop(&mut self) {
        if self.stack.is_empty() {
            println!("[-] Stack is NULL!")
        } else {
            self.stack.pop();
            self.min.pop();
        }
    }

    pub fn top(&self) -> i32 {
        if let Some(element) = *self.stack.last() {
            return element;
        }else {
            println!("[-] Stack is NULL!")
        }
    }

    pub fn getMin() -> i32 {
        self.min
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_155() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3); // --> Returns -3.
        min_stack.pop();
        assert_eq!(min_stack.top(), 0); // --> Returns 0.
        assert_eq!(min_stack.get_min(), -2); // --> Returns -2.[]
    }
}