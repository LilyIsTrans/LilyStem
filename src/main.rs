#![no_std]
#![no_main]

#[uefi::entry]
fn main(
    handle: uefi::Handle,
    mut system_table: uefi::table::SystemTable<uefi::table::Boot>,
) -> uefi::Status {
    uefi::Status::SUCCESS
}
