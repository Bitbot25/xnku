//! The XNK multiprocess scheduler
//! XNK will start this program in the userspace to manage multiprocessing.

use std::collections::VecDeque;

struct Service {
    // The elf binary for the service
    file: (),
}

struct Sched {
    services: Vec<Service>,
}

struct Process {}

fn main() {
    let tpms = xnk::tickrate();
    let mut queue: VecDeque<Process> = VecDeque::new();
    let aid: xnk::Aid = xnk::settimer(tpms * 200, || {
        let current = queue.pop_front();
        match current {
            Some(current) => {
                queue.push_back(current);
                xnk::TimerStat::Continue
            }
            None => xnk::TimerStat::Cancel,
        }
    });
    xnk::aidyield(aid); // block until async task is complete
    loop {}
}
