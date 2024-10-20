#[repr(C)]
#[doc = "Clock Video Encoder registers"]
#[doc(alias = "clk_venc")]
pub struct ClkVenc {
    axi: Axi,
    wave420l_axi: Wave420lAxi,
    wave420l_bpu: Wave420lBpu,
    wave420l_vce: Wave420lVce,
    wave420l_apb: Wave420lApb,
    noc_axi: NocAxi,
}
impl ClkVenc {
    #[doc = "0x00 - Clock Video Encoder AXI"]
    #[inline(always)]
    pub const fn axi(&self) -> &Axi {
        &self.axi
    }
    #[doc = "0x04 - Clock WAVE420L AXI"]
    #[inline(always)]
    pub const fn wave420l_axi(&self) -> &Wave420lAxi {
        &self.wave420l_axi
    }
    #[doc = "0x08 - Clock WAVE420L BPU"]
    #[inline(always)]
    pub const fn wave420l_bpu(&self) -> &Wave420lBpu {
        &self.wave420l_bpu
    }
    #[doc = "0x0c - Clock WAVE420L VCE"]
    #[inline(always)]
    pub const fn wave420l_vce(&self) -> &Wave420lVce {
        &self.wave420l_vce
    }
    #[doc = "0x10 - Clock WAVE420L APB"]
    #[inline(always)]
    pub const fn wave420l_apb(&self) -> &Wave420lApb {
        &self.wave420l_apb
    }
    #[doc = "0x14 - Clock Video Encoder NOC AXI"]
    #[inline(always)]
    pub const fn noc_axi(&self) -> &NocAxi {
        &self.noc_axi
    }
}
#[doc = "axi (rw) register accessor: Clock Video Encoder AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi`]
module"]
#[doc(alias = "axi")]
pub type Axi = crate::Reg<axi::AxiSpec>;
#[doc = "Clock Video Encoder AXI"]
pub mod axi;
#[doc = "wave420l_axi (rw) register accessor: Clock WAVE420L AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wave420l_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wave420l_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wave420l_axi`]
module"]
#[doc(alias = "wave420l_axi")]
pub type Wave420lAxi = crate::Reg<wave420l_axi::Wave420lAxiSpec>;
#[doc = "Clock WAVE420L AXI"]
pub mod wave420l_axi;
#[doc = "wave420l_bpu (rw) register accessor: Clock WAVE420L BPU\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wave420l_bpu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wave420l_bpu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wave420l_bpu`]
module"]
#[doc(alias = "wave420l_bpu")]
pub type Wave420lBpu = crate::Reg<wave420l_bpu::Wave420lBpuSpec>;
#[doc = "Clock WAVE420L BPU"]
pub mod wave420l_bpu;
#[doc = "wave420l_vce (rw) register accessor: Clock WAVE420L VCE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wave420l_vce::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wave420l_vce::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wave420l_vce`]
module"]
#[doc(alias = "wave420l_vce")]
pub type Wave420lVce = crate::Reg<wave420l_vce::Wave420lVceSpec>;
#[doc = "Clock WAVE420L VCE"]
pub mod wave420l_vce;
#[doc = "wave420l_apb (rw) register accessor: Clock WAVE420L APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wave420l_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wave420l_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wave420l_apb`]
module"]
#[doc(alias = "wave420l_apb")]
pub type Wave420lApb = crate::Reg<wave420l_apb::Wave420lApbSpec>;
#[doc = "Clock WAVE420L APB"]
pub mod wave420l_apb;
#[doc = "noc_axi (rw) register accessor: Clock Video Encoder NOC AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`noc_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`noc_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@noc_axi`]
module"]
#[doc(alias = "noc_axi")]
pub type NocAxi = crate::Reg<noc_axi::NocAxiSpec>;
#[doc = "Clock Video Encoder NOC AXI"]
pub mod noc_axi;
