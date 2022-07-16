# async-book 例子, 理解 Rust 异步的执行过程
## 执行
`cargo run --example test`

## 调用链分析

1. spawn()
    将 future 包装成 task 发送到 channel.
2. executor.run()
    1. 从 channel 获得 task, 获得 task 包裹的 future, 调用 future.poll(context)
    2. context 参数从哪来?
        1. 从 task 构建 Waker (该 Waker 的 wake() 函数的实现是将当前 task 发到 channel)
        2. 将 waker 包在 context 里
3. future.poll(context) (poll 函数一般不用手动写, 由编译器自动生成)
    - 执行业务逻辑, 判断是否 ready.
    - 从 context 里获得 waker, 将 waker 挂在 shared_state 上

**接下来, 只要有人能够在业务逻辑执行完的时候触发 waker.wake() 函数即可!!!**

标准的实现一般是有一个 reactor 使用 epoll/kqueue 监听事件, 来帮我们触发 wake.
在这个简单的例子里我们通过创建一个线程来触发 wake 函数. 所以在我们的 TimerFuture 的 构造函数里创建线程, 该线程在指定的延时后触发挂在 shared_state 上的 waker.wake().

## 时序图
![](./docs/seq.png)

## from
https://rust-lang.github.io/async-book/