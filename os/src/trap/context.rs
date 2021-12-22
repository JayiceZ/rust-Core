/* save context when trap*/

#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],   //registers
    pub sstatus: Sstatus,
    pub sepc: usize,
}