use mini_executor::executor;
use mini_executor::timer_future;
use std::time::Duration;

fn main() {
    let (executor, spawner) = executor::new_executor_and_spawner();

    // Spawn a task to print before and after waiting on a timer.
    // async 函数会生成一个嵌套 future, 包裹住 TimerFuture
    spawner.spawn(async {
        println!("howdy!");
        // Wait for our timer future to complete after two seconds.
        timer_future::TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });

    // Drop the spawner so that our executor knows it is finished and won't
    // receive more incoming tasks to run.
    drop(spawner);

    // Run the executor until the task queue is empty.
    // This will print "howdy!", pause, and then print "done!".
    executor.run();
}