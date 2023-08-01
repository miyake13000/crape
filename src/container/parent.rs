pub mod io_connector;

use anyhow::{anyhow, Context, Result};
use io_connector::IoConnector;
use nix::libc::{grantpt, unlockpt};
use nix::sched::{self, CloneFlags};
use nix::unistd::Pid;
use std::fs::File;
use std::io::{stdin, stdout};
use std::os::unix::io::AsRawFd;

pub struct Initilizer;

impl Initilizer {
    pub fn setns(child_pid: Pid, clone_flags: CloneFlags) -> Result<()> {
        let raw_child_pid = child_pid.as_raw() as isize;

        if clone_flags.contains(CloneFlags::CLONE_NEWUSER) {
            let userns_filename = format!("/proc/{}/ns/user", raw_child_pid);
            let userns = File::open(&userns_filename)
                .with_context(|| format!("Failed to open '{}", userns_filename))?;
            let userns_fd = userns.as_raw_fd();
            sched::setns(userns_fd, CloneFlags::CLONE_NEWUSER)
                .context("Failed to setns to userns")?;
        }

        if clone_flags.contains(CloneFlags::CLONE_NEWNS) {
            let mntns_filename = format!("/proc/{}/ns/mnt", raw_child_pid);
            let mntns = File::open(&mntns_filename)
                .with_context(|| format!("Failed to open '{}", mntns_filename))?;
            let mntns_fd = mntns.as_raw_fd();
            sched::setns(mntns_fd, CloneFlags::CLONE_NEWNS).context("Failed to setns to mntns")?;
        }

        Ok(())
    }

    pub fn connect_tty() -> Result<IoConnector> {
        let pty_master_path = "/dev/pts/ptmx";
        let pty_master = nix::fcntl::open(
            pty_master_path,
            nix::fcntl::OFlag::O_RDWR,
            nix::sys::stat::Mode::all(),
        )
        .context("Child process has not connected tty yet")?;

        if unsafe { grantpt(pty_master) } < 0 {
            return Err(anyhow!("Failed to grantpt('{}')", pty_master_path));
        }
        if unsafe { unlockpt(pty_master) } < 0 {
            return Err(anyhow!("Failed to unlockpt('{}')", pty_master_path));
        }

        Ok(IoConnector::new(
            stdout().as_raw_fd(),
            stdin().as_raw_fd(),
            pty_master,
            pty_master,
        ))
    }
}
