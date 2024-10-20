#[repr(C)]
#[doc = "DesignWare DMAC Common registers"]
#[doc(alias = "common")]
pub struct Common {
    intclear: Intclear,
    intstatus_enable: IntstatusEnable,
    intsignal_enable: IntsignalEnable,
    intstatus: Intstatus,
}
impl Common {
    #[doc = "0x00..0x08 - DMAC Interrupt Clear register contains the DMAC interrupt clear settings."]
    #[inline(always)]
    pub const fn intclear(&self) -> &Intclear {
        &self.intclear
    }
    #[doc = "0x08..0x10 - DMAC Interrupt Status Enable register contains the DMAC interrupt status enable settings."]
    #[inline(always)]
    pub const fn intstatus_enable(&self) -> &IntstatusEnable {
        &self.intstatus_enable
    }
    #[doc = "0x10..0x18 - DMAC Interrupt Signal Enable register contains the DMAC interrupt signal enable settings."]
    #[inline(always)]
    pub const fn intsignal_enable(&self) -> &IntsignalEnable {
        &self.intsignal_enable
    }
    #[doc = "0x18..0x20 - DMAC Interrupt Status register contains the DMAC interrupt status."]
    #[inline(always)]
    pub const fn intstatus(&self) -> &Intstatus {
        &self.intstatus
    }
}
#[doc = "intclear (w) register accessor: DMAC Interrupt Clear register contains the DMAC interrupt clear settings.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intclear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intclear`]
module"]
#[doc(alias = "intclear")]
pub type Intclear = crate::Reg<intclear::IntclearSpec>;
#[doc = "DMAC Interrupt Clear register contains the DMAC interrupt clear settings."]
pub mod intclear;
#[doc = "intstatus_enable (rw) register accessor: DMAC Interrupt Status Enable register contains the DMAC interrupt status enable settings.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intstatus_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intstatus_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstatus_enable`]
module"]
#[doc(alias = "intstatus_enable")]
pub type IntstatusEnable = crate::Reg<intstatus_enable::IntstatusEnableSpec>;
#[doc = "DMAC Interrupt Status Enable register contains the DMAC interrupt status enable settings."]
pub mod intstatus_enable;
#[doc = "intsignal_enable (rw) register accessor: DMAC Interrupt Signal Enable register contains the DMAC interrupt signal enable settings.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intsignal_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intsignal_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsignal_enable`]
module"]
#[doc(alias = "intsignal_enable")]
pub type IntsignalEnable = crate::Reg<intsignal_enable::IntsignalEnableSpec>;
#[doc = "DMAC Interrupt Signal Enable register contains the DMAC interrupt signal enable settings."]
pub mod intsignal_enable;
#[doc = "intstatus (r) register accessor: DMAC Interrupt Status register contains the DMAC interrupt status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstatus`]
module"]
#[doc(alias = "intstatus")]
pub type Intstatus = crate::Reg<intstatus::IntstatusSpec>;
#[doc = "DMAC Interrupt Status register contains the DMAC interrupt status."]
pub mod intstatus;
