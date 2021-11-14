extern crate systemstat;

use std::thread;

use anyhow::{bail, Error};
use systemstat::{CPULoad, Duration, Platform};
use sysinfo::{get_current_pid, ProcessExt, System, SystemExt};

pub fn get_cpu_stats() -> Result<CPULoad, Error> {
  let sys = systemstat::System::new();
  match sys.cpu_load_aggregate() {
    Ok(cpu) => {
      println!("\nMeasuring CPU load...");
      thread::sleep(Duration::from_secs(1));
      let cpu = cpu.done().unwrap();
      println!(
        "CPU load: {}% user, {}% nice, {}% system, {}% intr, {}% idle ",
        cpu.user * 100.0,
        cpu.nice * 100.0,
        cpu.system * 100.0,
        cpu.interrupt * 100.0,
        cpu.idle * 100.0
      );

      Ok(cpu)
    }
    Err(x) => {
      // let msg = format!("\nCPU load: error: {:?}", &x);
      println!("\nCPU load: error: {}", &x);
      bail!(&"cannot get CPU data");
    }
  }
}

pub fn get_current_process_cpu() -> Result<f32, Error> {
  let s = System::new();
  match get_current_pid() {
    Ok(pid) => {
      println!("current pid: {}", pid);
      match s.process(pid) {
        Some(p) => {
          let cpu = p.cpu_usage();
          println!("{}%", &cpu);
          Ok(cpu)
        }
        None => {
          let pid = s.process_by_name("carpe");
          match pid.first() {
            Some(p) => Ok(p.cpu_usage()),
            None => {
              let pid = s.process_by_name("app");
              match pid.first() {
                Some(p) => Ok(p.cpu_usage()),
                None => {
                  bail!("ERROR: could not load test cpu info for 'carpe' nor 'app'")
                }
              }
            }
          }
        }
      }
    }
    Err(e) => {
      bail!("failed to get current pid: {:?}", e)
    }
  }
}
