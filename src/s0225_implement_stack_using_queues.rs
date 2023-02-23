/**
 * [225] Implement Stack using Queues
 * 
 * Implement a last-in-first-out (LIFO) stack using only two queues. 
 * The implemented stack should support all the functions of a normal stack (push, top, pop, and empty).
 * 
 * Implement the MyStack class:
 * * void push(int x) Pushes element x to the top of the stack.
 * * int pop() Removes the element on the top of the stack and returns it.
 * * int top() Returns the element on the top of the stack.
 * 8 boolean empty() Returns true if the stack is empty, false otherwise.
 * 
 * Notes:
 * You must use only standard operations of a queue, which means that 
 * only push to back, peek/pop from front, size and is empty operations are valid.
 *
 * Depending on your language, the queue may not be supported natively. 
 * You may simulate a queue using a list or deque (double-ended queue) as long as you use only a queue's standard operations.
 *
 * Example 1:
 * 
 * 
 * Input
 * ["MyStack", "push", "push", "top", "pop", "empty"]
 * [[], [1], [2], [], [], []]
 * Output
 * [null, null, null, 2, 2, false]
 * 
 * Explanation
 * MyStack myStack = new MyStack();
 * myStack.push(1);
 * myStack.push(2);
 * myStack.top(); // return 2
 * myStack.pop(); // return 2
 * myStack.empty(); // return False
 * 
 */

 /**
  * Your MyStack object will be instantiated and called as such:
  * let obj = MyStack::new();
  * obj.push(x);
  * let ret_2: i32 = obj.pop();
  * let ret_3: i32 = obj.top();
  * let ret_4: bool = obj.empty();
  */

use std::collections::VecDeque;

struct MyStack {
    queue1: VecDeque<i32>,
    queue2: VecDeque<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
* If you need a mutable reference, change it to `&mut self` instead.
*/
impl MyStack {

    fn new() -> Self {
        Self { 
            queue1: VecDeque::new(), 
            queue2: VecDeque::new(),
        }
    }
    
    fn push(&mut self, x: i32) {

        // if self.queue1.is_empty() && self.queue2.is_empty() {
        //     self.queue1.push_back(x);
        // } else if !self.queue1.is_empty() {
        //     self.queue1.push_back(x);
        // } else {
        //     self.queue2.push_back(x);
        // }
    }
    
    fn pop(&mut self) -> i32 {

        if !self.queue1.is_empty() {
            let len = self.queue1.len();
            for _ in 0..len - 1 {
                self.queue2.push_back(self.queue1.pop_front().unwrap());
            }
            self.queue1.pop_front().unwrap()
        } else {
            let len = self.queue2.len();
            for _ in 0..len - 1 {
                self.queue1.push_back(self.queue2.pop_front().unwrap());
            }
            self.queue2.pop_front().unwrap()
        }
    }
    
    fn top(&mut self) -> i32 {
        let mut top;
        if !self.queue1.is_empty() {
            let len = self.queue1.len();
            for _ in 0..len - 1 {
                self.queue2.push_back(self.queue1.pop_front().unwrap());
            }
            top = self.queue1.pop_front().unwrap();
            self.queue2.push_back(top);
        } else {
            let len = self.queue2.len();
            for _ in 0..len - 1 {
                self.queue1.push_back(self.queue2.pop_front().unwrap());
            }
            top = self.queue2.pop_front().unwrap();
            self.queue1.push_back(top);
        }
        top
    }
    
    fn empty(&self) -> bool {
        self.queue1.is_empty() && self.queue2.is_empty()
    }
}

/*
use std::collections::VecDeque;

struct MyStack {
    q1: VecDeque<i32>,
    q2: VecDeque<i32>,
    out1: bool,
}

impl MyStack {
    fn new() -> Self {
        Self {
            q1: VecDeque::new(),
            q2: VecDeque::new(),
            out1: false,
        }
    }
    
    fn push(&mut self, x: i32) {
        let (q1, q2) = if !self.out1 {
            (&mut self.q1, &mut self.q2)
        } else {
            (&mut self.q2, &mut self.q1)
        };
        q1.push_back(x);
        while let Some(val) = q2.pop_front() {
            q1.push_back(val);
        }
        self.out1 = !self.out1;
    }
    
    fn pop(&mut self) -> i32 {
        if self.out1 {
            self.q1.pop_front().unwrap()
        } else {
            self.q2.pop_front().unwrap()
        }
    }
    
    fn top(&mut self) -> i32 {
        if self.out1 {
            *self.q1.front().unwrap()
        } else {
            *self.q2.front().unwrap()
        }
    }
    
    fn empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
    }
}
*/
