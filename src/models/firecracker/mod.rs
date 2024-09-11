use std::time::Duration;

use anyhow::{Ok, Result};
use firepilot::{
    builder::{
        drive::DriveBuilder, executor::FirecrackerExecutorBuilder, kernel::KernelBuilder, Builder,
        Configuration,
    },
    machine::Machine,
};
use tokio::time::sleep;

pub async fn init_vm() -> Result<()> {
    // to do - add the paths to .env
    // These are deployment env specific
    let kernel_image_path = "/home/sravan/src/notebook/linux/vmlinux-5.10.223".to_string();
    let rootfs_path = "/home/sravan/src/notebook/linux/ubuntu-22.04.ext4".to_string();
    let executer_path = "/home/sravan/src/notebook/linux/executer/".to_string();
    let firecracker_bin_location = "/usr/bin/firecracker".to_string();

    println!("Starting a VM");
    let kernel = KernelBuilder::new()
        .with_kernel_image_path(kernel_image_path)
        .with_boot_args("reboot=k panic=1 pci=off".to_string())
        .try_build()
        .unwrap();
    println!("vmlinux.bin loaded");

    let drive = DriveBuilder::new()
        .with_drive_id("rootfs".to_string())
        .with_path_on_host(rootfs_path.into())
        .as_read_only()
        .as_root_device()
        .try_build()
        .unwrap();
    println!("rootfs loaded");

    let executor = FirecrackerExecutorBuilder::new()
        .with_chroot(executer_path)
        .with_exec_binary(firecracker_bin_location.into())
        .try_build()
        .unwrap();
    println!("executer setup");

    let config = Configuration::new("simple_vm".to_string())
        .with_kernel(kernel)
        .with_executor(executor)
        .with_drive(drive);
    println!("Configuration finised");
    let mut machine = Machine::new();
    machine.create(config).await.unwrap();

    println!("Boot micro vm");
    machine.start().await.expect("Could not start VM");
    println!("Waiting a few seconds, the VM is started at this point");
    sleep(Duration::from_secs(5)).await;
    machine.stop().await.unwrap();
    println!("Shutting down the VM");
    machine.kill().await.unwrap();

    Ok(())
}
