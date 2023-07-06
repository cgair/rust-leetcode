// 问题描述:
// 链表找环并证明?
// 证明: 设设链表共有 a + b 个节点, 其中链表头部到链表入口有 
//       a 个节点 (不计链表入口节点); 链表环有 b 个节点
//       我们有快慢指针 f, s. 已知 f 的速度是 s 的两倍,
//       df = 2 * ds    (1)
//       第一次相遇时, f 比 s 多走了 n 个环的长度
//       df = ds + nb   (2)
//       式 (1) - (2) 得 ds = nb, df = 2nb (n = 1, 2, 3, ...)
//       从 head 走到环入口需要走 a + nb, 而 s 已经走了 nb 步,
//       那么 s 再走 a 步就是入环点了.
//       如何知道 s 刚好走了 a 步? 从 head 开始, 和 s 指针一起走, 相遇时刚好就是 a 步.
#include "listnode.h"
#include <vector>

ListNode* detect_cycle(ListNode* head) {
    ListNode *slow = head, *fast = head;
    while (fast != nullptr && fast->next != nullptr) {
        slow = slow->next;
        fast = fast->next->next;

        if (slow == fast) break;
    }

    if (fast == nullptr || fast->next == nullptr) return nullptr;

    fast = head;

    while (slow != fast) {
        slow = slow->next;
        fast = fast->next;
    }

    return slow;
}

int main()
{
    return 0;
}