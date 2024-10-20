#[repr(C)]
#[doc = "USB3 XHCI Capability registers."]
#[doc(alias = "cap")]
pub struct Cap {
    hc_capbase: HcCapbase,
    hcs_params1: HcsParams1,
    hcs_params2: HcsParams2,
    hcs_params3: HcsParams3,
    hcc_params: HccParams,
    db_off: DbOff,
    run_regs_off: RunRegsOff,
    hcc_params2: HccParams2,
}
impl Cap {
    #[doc = "0x00 - USB3 XHCI host controller capability base - defines the offset of the `op` register cluster."]
    #[inline(always)]
    pub const fn hc_capbase(&self) -> &HcCapbase {
        &self.hc_capbase
    }
    #[doc = "0x04 - USB3 XHCI host controller structural parameters 1."]
    #[inline(always)]
    pub const fn hcs_params1(&self) -> &HcsParams1 {
        &self.hcs_params1
    }
    #[doc = "0x08 - USB3 XHCI host controller structural parameters 2."]
    #[inline(always)]
    pub const fn hcs_params2(&self) -> &HcsParams2 {
        &self.hcs_params2
    }
    #[doc = "0x0c - USB3 XHCI host controller structural parameters 3."]
    #[inline(always)]
    pub const fn hcs_params3(&self) -> &HcsParams3 {
        &self.hcs_params3
    }
    #[doc = "0x10 - USB3 XHCI host controller capability parameters."]
    #[inline(always)]
    pub const fn hcc_params(&self) -> &HccParams {
        &self.hcc_params
    }
    #[doc = "0x14 - USB3 XHCI host controller doorbell array offset."]
    #[inline(always)]
    pub const fn db_off(&self) -> &DbOff {
        &self.db_off
    }
    #[doc = "0x18 - USB3 XHCI host controller run register cluster offset - runtime register space offset."]
    #[inline(always)]
    pub const fn run_regs_off(&self) -> &RunRegsOff {
        &self.run_regs_off
    }
    #[doc = "0x1c - USB3 XHCI host controller capabilities parameters - XHCI v1.1."]
    #[inline(always)]
    pub const fn hcc_params2(&self) -> &HccParams2 {
        &self.hcc_params2
    }
}
#[doc = "hc_capbase (r) register accessor: USB3 XHCI host controller capability base - defines the offset of the `op` register cluster.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_capbase::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc_capbase`]
module"]
#[doc(alias = "hc_capbase")]
pub type HcCapbase = crate::Reg<hc_capbase::HcCapbaseSpec>;
#[doc = "USB3 XHCI host controller capability base - defines the offset of the `op` register cluster."]
pub mod hc_capbase;
#[doc = "hcs_params1 (r) register accessor: USB3 XHCI host controller structural parameters 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcs_params1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcs_params1`]
module"]
#[doc(alias = "hcs_params1")]
pub type HcsParams1 = crate::Reg<hcs_params1::HcsParams1Spec>;
#[doc = "USB3 XHCI host controller structural parameters 1."]
pub mod hcs_params1;
#[doc = "hcs_params2 (r) register accessor: USB3 XHCI host controller structural parameters 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcs_params2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcs_params2`]
module"]
#[doc(alias = "hcs_params2")]
pub type HcsParams2 = crate::Reg<hcs_params2::HcsParams2Spec>;
#[doc = "USB3 XHCI host controller structural parameters 2."]
pub mod hcs_params2;
#[doc = "hcs_params3 (r) register accessor: USB3 XHCI host controller structural parameters 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcs_params3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcs_params3`]
module"]
#[doc(alias = "hcs_params3")]
pub type HcsParams3 = crate::Reg<hcs_params3::HcsParams3Spec>;
#[doc = "USB3 XHCI host controller structural parameters 3."]
pub mod hcs_params3;
#[doc = "hcc_params (r) register accessor: USB3 XHCI host controller capability parameters.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcc_params::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcc_params`]
module"]
#[doc(alias = "hcc_params")]
pub type HccParams = crate::Reg<hcc_params::HccParamsSpec>;
#[doc = "USB3 XHCI host controller capability parameters."]
pub mod hcc_params;
#[doc = "db_off (r) register accessor: USB3 XHCI host controller doorbell array offset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`db_off::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@db_off`]
module"]
#[doc(alias = "db_off")]
pub type DbOff = crate::Reg<db_off::DbOffSpec>;
#[doc = "USB3 XHCI host controller doorbell array offset."]
pub mod db_off;
#[doc = "run_regs_off (r) register accessor: USB3 XHCI host controller run register cluster offset - runtime register space offset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`run_regs_off::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@run_regs_off`]
module"]
#[doc(alias = "run_regs_off")]
pub type RunRegsOff = crate::Reg<run_regs_off::RunRegsOffSpec>;
#[doc = "USB3 XHCI host controller run register cluster offset - runtime register space offset."]
pub mod run_regs_off;
#[doc = "hcc_params2 (r) register accessor: USB3 XHCI host controller capabilities parameters - XHCI v1.1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcc_params2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcc_params2`]
module"]
#[doc(alias = "hcc_params2")]
pub type HccParams2 = crate::Reg<hcc_params2::HccParams2Spec>;
#[doc = "USB3 XHCI host controller capabilities parameters - XHCI v1.1."]
pub mod hcc_params2;
