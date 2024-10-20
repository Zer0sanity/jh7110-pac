#[repr(C)]
#[doc = "Clock Video Decoder registers"]
#[doc(alias = "clk_vdec")]
pub struct ClkVdec {
    axi: Axi,
    wave511_axi: Wave511Axi,
    wave511_bpu: Wave511Bpu,
    wave511_vce: Wave511Vce,
    wave511_apb: Wave511Apb,
    jpg_arb: JpgArb,
    jpg_main: JpgMain,
    noc_axi: NocAxi,
}
impl ClkVdec {
    #[doc = "0x00 - Clock Video Decoder AXI"]
    #[inline(always)]
    pub const fn axi(&self) -> &Axi {
        &self.axi
    }
    #[doc = "0x04 - Clock WAVE511 AXI"]
    #[inline(always)]
    pub const fn wave511_axi(&self) -> &Wave511Axi {
        &self.wave511_axi
    }
    #[doc = "0x08 - Clock WAVE511 BPU"]
    #[inline(always)]
    pub const fn wave511_bpu(&self) -> &Wave511Bpu {
        &self.wave511_bpu
    }
    #[doc = "0x0c - Clock WAVE511 VCE"]
    #[inline(always)]
    pub const fn wave511_vce(&self) -> &Wave511Vce {
        &self.wave511_vce
    }
    #[doc = "0x10 - Clock WAVE511 APB"]
    #[inline(always)]
    pub const fn wave511_apb(&self) -> &Wave511Apb {
        &self.wave511_apb
    }
    #[doc = "0x14 - Clock Video Decoder JPG ARB"]
    #[inline(always)]
    pub const fn jpg_arb(&self) -> &JpgArb {
        &self.jpg_arb
    }
    #[doc = "0x18 - Clock Video Decoder JPG Main"]
    #[inline(always)]
    pub const fn jpg_main(&self) -> &JpgMain {
        &self.jpg_main
    }
    #[doc = "0x1c - Clock Video Decoder NOC AXI"]
    #[inline(always)]
    pub const fn noc_axi(&self) -> &NocAxi {
        &self.noc_axi
    }
}
#[doc = "axi (rw) register accessor: Clock Video Decoder AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi`]
module"]
#[doc(alias = "axi")]
pub type Axi = crate::Reg<axi::AxiSpec>;
#[doc = "Clock Video Decoder AXI"]
pub mod axi;
#[doc = "wave511_axi (rw) register accessor: Clock WAVE511 AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wave511_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wave511_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wave511_axi`]
module"]
#[doc(alias = "wave511_axi")]
pub type Wave511Axi = crate::Reg<wave511_axi::Wave511AxiSpec>;
#[doc = "Clock WAVE511 AXI"]
pub mod wave511_axi;
#[doc = "wave511_bpu (rw) register accessor: Clock WAVE511 BPU\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wave511_bpu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wave511_bpu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wave511_bpu`]
module"]
#[doc(alias = "wave511_bpu")]
pub type Wave511Bpu = crate::Reg<wave511_bpu::Wave511BpuSpec>;
#[doc = "Clock WAVE511 BPU"]
pub mod wave511_bpu;
#[doc = "wave511_vce (rw) register accessor: Clock WAVE511 VCE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wave511_vce::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wave511_vce::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wave511_vce`]
module"]
#[doc(alias = "wave511_vce")]
pub type Wave511Vce = crate::Reg<wave511_vce::Wave511VceSpec>;
#[doc = "Clock WAVE511 VCE"]
pub mod wave511_vce;
#[doc = "wave511_apb (rw) register accessor: Clock WAVE511 APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wave511_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wave511_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wave511_apb`]
module"]
#[doc(alias = "wave511_apb")]
pub type Wave511Apb = crate::Reg<wave511_apb::Wave511ApbSpec>;
#[doc = "Clock WAVE511 APB"]
pub mod wave511_apb;
#[doc = "jpg_arb (rw) register accessor: Clock Video Decoder JPG ARB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jpg_arb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jpg_arb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jpg_arb`]
module"]
#[doc(alias = "jpg_arb")]
pub type JpgArb = crate::Reg<jpg_arb::JpgArbSpec>;
#[doc = "Clock Video Decoder JPG ARB"]
pub mod jpg_arb;
#[doc = "jpg_main (rw) register accessor: Clock Video Decoder JPG Main\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jpg_main::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jpg_main::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jpg_main`]
module"]
#[doc(alias = "jpg_main")]
pub type JpgMain = crate::Reg<jpg_main::JpgMainSpec>;
#[doc = "Clock Video Decoder JPG Main"]
pub mod jpg_main;
#[doc = "noc_axi (rw) register accessor: Clock Video Decoder NOC AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`noc_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`noc_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@noc_axi`]
module"]
#[doc(alias = "noc_axi")]
pub type NocAxi = crate::Reg<noc_axi::NocAxiSpec>;
#[doc = "Clock Video Decoder NOC AXI"]
pub mod noc_axi;
