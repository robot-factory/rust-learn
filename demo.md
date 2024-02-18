# 两个栈实现一个队列
push pop top empty
in queue  de

```js
const stack = new Stack()

class Queue {
  private stack1 = new Stack()
  private stack2 = new Stack()
  
  public in(v) {
    stack1.push(v)
  }

  public out() {
    if (stack2.empty()) {
      while (!stack1.empty()) {
        const el = stack1.pop()
        stack2.push(el)
      }
      return stack2.pop()
    } else {
      return stack2.pop()
    }
  }
}


```