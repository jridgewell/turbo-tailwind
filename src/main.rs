use std::{env::current_dir, time::Duration};

use turbo_tasks::{util::FormatDuration, TurboTasks, UpdateInfo, Vc};
use turbo_tasks_memory::MemoryBackend;

#[tokio::main]
async fn main() {
    println!("make any edits you want to the README.md file");

    // Calling the lib's register function informs the turbo engine of all
    // macro-decorated values and functions in our crate. This boilerplate is
    // unfortunately necessary, but only needs to be done in the root of the
    // main function.
    turbo_tailwind::register();

    // Setup a "backend" for the Turbo Engine, which controls where the graph and
    // output caching is stored.
    let tt = TurboTasks::new(MemoryBackend::new(usize::MAX));

    // This is a "root task", which is simply the root of our call graph and the
    // entry point into the Turbo Engine.
    let task = tt.spawn_root_task(|| {
        Box::pin(async {
            let cwd = current_dir().unwrap().to_str().unwrap().to_string();
            turbo_tailwind::tailwind(cwd).await?;

            Ok(Vc::<()>::cell(()))
        })
    });

    // For our example, start the root task, which can continually be recomputed.
    let _ = tt.wait_task_completion(task, true);

    // The root task is reactive, so any changes (like a file save) will recompute
    // the task.
    loop {
        let UpdateInfo {
            duration, tasks, ..
        } = tt
            .get_or_wait_aggregated_update_info(Duration::from_millis(100))
            .await;
        println!("updated {} tasks in {}", tasks, FormatDuration(duration));
    }
}
