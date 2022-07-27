// the ribbit (pre-)init system
//
// part of aos
//
// used as the init system for the kernel and OS components
//
// this is NOT the actual user facing init system , which starts what the user wants
//
// a bit weird of a architecture , if your coming from linux , I know.
//
// 7/23/22
//
// ribbit.rs starts here:

// ribbit::croak is the entrypoint to ribbit
// the result code table is documented at return
use crate::println;

#[path = "ffs.rs"] mod ffs;
#[path = "dsh.rs"] mod dsh;

pub fn croak(){
    println!("info: ribbit started");
    let _filesystem = ffs::Ffs::new();
    println!("info: ribbit reports, 'inital (fake) filesystem started'");
    dsh::init();
    println!("dsh finsished , nothing to do");
    println!("preparing for shutdown");
    println!("commencing shutdown");
    println!("you may now pull the plug");
}
