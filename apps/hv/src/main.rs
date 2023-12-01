#![no_std]
#![no_main]

extern crate alloc;
#[macro_use]
extern crate libax;

use libax::{
    hv::{
        HyperCraftHalImpl, PerCpu, VM,
    },
    info,
};
use core::sync::atomic::{AtomicUsize, Ordering};
use libax::thread;


mod x64;

const NUM_VM: usize = 2;
static FINISHED_TASKS: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
fn main(hart_id: usize) {
    println!("Hello, hv!");
    println!("into main {}", hart_id);

    let mut p = PerCpu::<HyperCraftHalImpl>::new(hart_id);
    p.hardware_enable().unwrap();
    let vmcs_revision_id = p.get_vmcs_revision_id();

    for id in 0..NUM_VM {
        thread::spawn(move || {

            let gpm = x64::setup_gpm(id).unwrap();
            let mut vm = VM::<HyperCraftHalImpl>::new(id);
            let vcpu_id = vm.add_vcpu(vmcs_revision_id, x64::BIOS_ENTRY, gpm.nest_page_table_root()).unwrap();
            let vcpu = vm.get_vcpu(vcpu_id).unwrap();
            vcpu.run();


        });
    }
    p.hardware_disable().unwrap();

    return;

}