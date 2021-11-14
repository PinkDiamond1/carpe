extern crate systemstat;

use anyhow::{bail, Error};
use std::thread;
use std::time::Duration;
// use systemstat::{CPULoad, Platform};
use sysinfo::{ProcessExt, System, SystemExt, get_current_pid};

// pub fn get_cpu_stats() -> Result<CPULoad, Error> {
//   let sys = systemstat::System::new();
//   match sys.cpu_load_aggregate() {
//     Ok(cpu) => {
//       println!("\nMeasuring CPU load...");
//       thread::sleep(Duration::from_secs(1));
//       let cpu = cpu.done().unwrap();
//       println!(
//         "CPU load: {}% user, {}% nice, {}% system, {}% intr, {}% idle ",
//         cpu.user * 100.0,
//         cpu.nice * 100.0,
//         cpu.system * 100.0,
//         cpu.interrupt * 100.0,
//         cpu.idle * 100.0
//       );

//       Ok(cpu)
//     }
//     Err(x) => {
//       // let msg = format!("\nCPU load: error: {:?}", &x);
//       println!("\nCPU load: error: {}", &x);
//       bail!(&"cannot get CPU data");
//     }
//   }
// }

pub fn get_current_process_cpu() -> Result<f32, Error>{
  let s = System::new();
  match get_current_pid() {
    Ok(pid) => {
      println!("current pid: {}", pid);
      if let Some(process) = s.process(1337) {
        let cpu = process.cpu_usage();
          println!("{}%", &cpu);
          return Ok(cpu)
      }
    }
    Err(e) => {
      eprintln!("failed to get current pid: {}", e);
    }
  }
  bail!("ERROR: could not get CPU info for process");
}
