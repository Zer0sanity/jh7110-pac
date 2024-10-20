#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    syscfg0: Syscfg0,
    syscfg1: Syscfg1,
    syscfg2: Syscfg2,
    test: [Test; 4],
}
impl RegisterBlock {
    #[doc = "0x00 - VOUT SYSCFG 0: dom_vout_sysconsaif_0"]
    #[inline(always)]
    pub const fn syscfg0(&self) -> &Syscfg0 {
        &self.syscfg0
    }
    #[doc = "0x04 - VOUT SYSCFG 1: dom_vout_sysconsaif_4"]
    #[inline(always)]
    pub const fn syscfg1(&self) -> &Syscfg1 {
        &self.syscfg1
    }
    #[doc = "0x08 - VOUT SYSCFG 2: dom_vout_sysconsaif_8"]
    #[inline(always)]
    pub const fn syscfg2(&self) -> &Syscfg2 {
        &self.syscfg2
    }
    #[doc = "0x0c..0x1c - VOUT SYSCFG 3-6: dom_vout_sysconsaif_12 - dom_vout_sysonsaif_24"]
    #[inline(always)]
    pub const fn test(&self, n: usize) -> &Test {
        &self.test[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c..0x1c - VOUT SYSCFG 3-6: dom_vout_sysconsaif_12 - dom_vout_sysonsaif_24"]
    #[inline(always)]
    pub fn test_iter(&self) -> impl Iterator<Item = &Test> {
        self.test.iter()
    }
}
#[doc = "syscfg0 (rw) register accessor: VOUT SYSCFG 0: dom_vout_sysconsaif_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg0`]
module"]
#[doc(alias = "syscfg0")]
pub type Syscfg0 = crate::Reg<syscfg0::Syscfg0Spec>;
#[doc = "VOUT SYSCFG 0: dom_vout_sysconsaif_0"]
pub mod syscfg0;
#[doc = "syscfg1 (rw) register accessor: VOUT SYSCFG 1: dom_vout_sysconsaif_4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg1`]
module"]
#[doc(alias = "syscfg1")]
pub type Syscfg1 = crate::Reg<syscfg1::Syscfg1Spec>;
#[doc = "VOUT SYSCFG 1: dom_vout_sysconsaif_4"]
pub mod syscfg1;
#[doc = "syscfg2 (rw) register accessor: VOUT SYSCFG 2: dom_vout_sysconsaif_8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg2`]
module"]
#[doc(alias = "syscfg2")]
pub type Syscfg2 = crate::Reg<syscfg2::Syscfg2Spec>;
#[doc = "VOUT SYSCFG 2: dom_vout_sysconsaif_8"]
pub mod syscfg2;
#[doc = "test (rw) register accessor: VOUT SYSCFG 3-6: dom_vout_sysconsaif_12 - dom_vout_sysonsaif_24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test`]
module"]
#[doc(alias = "test")]
pub type Test = crate::Reg<test::TestSpec>;
#[doc = "VOUT SYSCFG 3-6: dom_vout_sysconsaif_12 - dom_vout_sysonsaif_24"]
pub mod test;
