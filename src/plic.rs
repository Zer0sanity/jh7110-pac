#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    priority: [Priority; 136],
    _reserved1: [u8; 0x0ddc],
    pending: [Pending; 5],
    _reserved2: [u8; 0x0fec],
    enable: (),
    _reserved3: [u8; 0x001f_e000],
    threshold_claim: (),
}
impl RegisterBlock {
    #[doc = "0x04..0x224 - RISC-V PLIC Interrupt Source Priority: the priority value `0` is reserved to mean `never interrupt`, and interrupt priority increases with increasing integer values."]
    #[inline(always)]
    pub const fn priority(&self, n: usize) -> &Priority {
        &self.priority[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x224 - RISC-V PLIC Interrupt Source Priority: the priority value `0` is reserved to mean `never interrupt`, and interrupt priority increases with increasing integer values."]
    #[inline(always)]
    pub fn priority_iter(&self) -> impl Iterator<Item = &Priority> {
        self.priority.iter()
    }
    #[doc = "0x1000..0x1014 - RISC-V PLIC Pending: 32-bit register indicating if there is a pending interrupt. The bit index indicates the interrupt source, e.g. pending\\[0\\]\\[31\\]
is interrupt 31, pending\\[1\\]\\[0\\]
is interrupt 32"]
    #[inline(always)]
    pub const fn pending(&self, n: usize) -> &Pending {
        &self.pending[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1000..0x1014 - RISC-V PLIC Pending: 32-bit register indicating if there is a pending interrupt. The bit index indicates the interrupt source, e.g. pending\\[0\\]\\[31\\]
is interrupt 31, pending\\[1\\]\\[0\\]
is interrupt 32"]
    #[inline(always)]
    pub fn pending_iter(&self) -> impl Iterator<Item = &Pending> {
        self.pending.iter()
    }
    #[doc = "0x2000..0x2064 - PLIC interrupt enable registers (per-HART)"]
    #[inline(always)]
    pub const fn enable(&self, n: usize) -> &Enable {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(8192)
                .add(128 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2000..0x2064 - PLIC interrupt enable registers (per-HART)"]
    #[inline(always)]
    pub fn enable_iter(&self) -> impl Iterator<Item = &Enable> {
        (0..5).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(8192)
                .add(128 * n)
                .cast()
        })
    }
    #[doc = "0x200000..0x200028 - PLIC threshold and claim_complete registers"]
    #[inline(always)]
    pub const fn threshold_claim(&self, n: usize) -> &ThresholdClaim {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(2097152)
                .add(4096 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200000..0x200028 - PLIC threshold and claim_complete registers"]
    #[inline(always)]
    pub fn threshold_claim_iter(&self) -> impl Iterator<Item = &ThresholdClaim> {
        (0..5).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(2097152)
                .add(4096 * n)
                .cast()
        })
    }
}
#[doc = "priority (rw) register accessor: RISC-V PLIC Interrupt Source Priority: the priority value `0` is reserved to mean `never interrupt`, and interrupt priority increases with increasing integer values.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority`]
module"]
#[doc(alias = "priority")]
pub type Priority = crate::Reg<priority::PrioritySpec>;
#[doc = "RISC-V PLIC Interrupt Source Priority: the priority value `0` is reserved to mean `never interrupt`, and interrupt priority increases with increasing integer values."]
pub mod priority;
#[doc = "pending (rw) register accessor: RISC-V PLIC Pending: 32-bit register indicating if there is a pending interrupt. The bit index indicates the interrupt source, e.g. pending\\[0\\]\\[31\\]
is interrupt 31, pending\\[1\\]\\[0\\]
is interrupt 32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pending::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pending::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pending`]
module"]
#[doc(alias = "pending")]
pub type Pending = crate::Reg<pending::PendingSpec>;
#[doc = "RISC-V PLIC Pending: 32-bit register indicating if there is a pending interrupt. The bit index indicates the interrupt source, e.g. pending\\[0\\]\\[31\\]
is interrupt 31, pending\\[1\\]\\[0\\]
is interrupt 32"]
pub mod pending;
#[doc = "PLIC interrupt enable registers (per-HART)"]
pub use self::enable::Enable;
#[doc = r"Cluster"]
#[doc = "PLIC interrupt enable registers (per-HART)"]
pub mod enable;
#[doc = "PLIC threshold and claim_complete registers"]
pub use self::threshold_claim::ThresholdClaim;
#[doc = r"Cluster"]
#[doc = "PLIC threshold and claim_complete registers"]
pub mod threshold_claim;
