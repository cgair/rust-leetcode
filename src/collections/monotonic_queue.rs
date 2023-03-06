/**
 * [单调队列结构]
 * 其实就是一个「队列」, 只是使用了一点 trick, 使得队列中的元素全都是单调递增 (或递减) 的.
 * 
 * 类似的有「优先级队」, 专门用来动态寻找最值的, 创建一个大 (小) 顶堆, 就可以很快拿到最大 (小) 值了
 * 如果单纯地维护最值的话, 优先级队列很专业, 队头元素就是最值. 
 * 但优先级队列无法满足标准队列结构「先进先出」的时间顺序, 因为优先级队列底层利用二叉堆对元素进行动态排序, 
 * 元素的出队顺序是元素的大小顺序, 和入队的先后顺序完全没有关系.
 * 
 * 所以, 现在需要一种新的队列结构, **既能够维护队列元素「先进先出」的时间顺序**, 
 * 又能够正确维护队列中所有元素的最值, 这就是「单调队列」结构.
 * 
 * 「单调队列」这个数据结构主要用来辅助解决滑动窗口相关的问题
 */
use std::collections::VecDeque;

#[derive(Debug)]
pub struct MonotonicQueue<T> {
    inner: VecDeque<T>
}

impl<T> MonotonicQueue<T> {
    pub fn new() -> Self {
        MonotonicQueue { inner: VecDeque::new() }
    }
}

impl<T: Ord> MonotonicQueue<T> {
    /// 在尾部添加一个元素 n, 维护 [`MonotonicQueue`] 的单调性质
    pub fn push(&mut self, value: T) {
        while !self.inner.is_empty() && *self.inner.back().unwrap() < value {
            self.inner.pop_back();
        }
        self.inner.push_back(value);
    }

    /// 返回当前队列中的最大值
    pub fn max(&self) -> Option<&T> {
        self.inner.front()
    }

    /// 队头元素如果是 value, 删除它
    pub fn pop(&mut self, value: T) {
        if let Some(v) = self.inner.front() {
            if *v == value {
                self.inner.pop_front();
            }
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_pop() {
        let mut mq = MonotonicQueue::new();
        mq.push(5);
        mq.push(3);
        mq.push(2);
        mq.push(1);
        mq.push(4);

        mq.pop(2);
        assert_eq!(
            mq.max(),
            Some(&5)
        );
        mq.pop(5);
        assert_eq!(
            mq.max(),
            Some(&4)
        );
    }
}

#[test]
fn test_vecdeque_api() {
    let mut queue = VecDeque::new();
    queue.push_back(1);
    queue.push_back(2);
    queue.push_back(3);

    for num in queue.iter_mut() {
        *num = *num - 2;
    }
    println!("{}", queue.pop_front().unwrap());
    println!("{}", queue.pop_front().unwrap());
    println!("{}", queue.pop_front().unwrap());
}