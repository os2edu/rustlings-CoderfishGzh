// threads2.rs
//执行`Rustlings Hint threads2`或使用``提示''手表子命令进行提示。
//在上次练习中构建，我们希望所有线程都能完成他们的工作，但是这次
//呈现的线程需要负责更新共享值：jobstatus.jobs_completed



use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::sync::Mutex;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
           // todo：您必须在更新共享值之前采取措施
           let mut status = status_shared.lock().unwrap();
           status.jobs_completed = status.jobs_completed + 1;
            
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
       // todo：打印jobstatus.jobs_completed的值。你注意到什么
        //在输出中有趣吗？您是否必须在所有手柄上“加入”？
        println!("jobs completed {:?}", status.lock().unwrap().jobs_completed);
    }
}
