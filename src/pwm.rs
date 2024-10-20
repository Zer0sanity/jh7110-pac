#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cntr: Cntr,
    hrc: Hrc,
    lrc: Lrc,
    ctrl: Ctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - Opencores PTC PWM v1 CNTR is the actual counter register. It is incremented at every counter/timer clock cycle. Source clock is either system clock or ptc_ecgt eclk/gate input. Selection between both clocks is performed with the RPTC_CTRL\\[ECLK\\]. Active edge of external clock is selected with the RPTC_CTRL\\[NEC\\]. In order to count, RPTC_CNTR must first be enabled with the RPTC_CTRL\\[EN\\]. RPTC_CNTR can be reset with the RPTC_CTRL\\[RST\\]. RPTC_CNTR can operate in either single-run mode or continues mode. Mode is selected with the RPTC_CTRL\\[SINGLE\\]."]
    #[inline(always)]
    pub const fn cntr(&self) -> &Cntr {
        &self.cntr
    }
    #[doc = "0x04 - Opencores PTC PWM v1 HRC register is a 2nd out of two reference/capture registers. It has two functions: - In reference mode it is used to assert high PWM output or to generate an interrupt - In capture mode it captures RPTC_CNTR value on high value of ptc_capt signal. The RPTC_HRC should have lower value than RPTC_LRC. This is because PWM output goes first high and later low."]
    #[inline(always)]
    pub const fn hrc(&self) -> &Hrc {
        &self.hrc
    }
    #[doc = "0x08 - Opencores PTC PWM v1 RPTC_LRC register is a 1st out of two reference/capture registers. It has two functions: - In reference mode it is used to assert low PWM output or to generate an interrupt - In capture mode it captures RPTC_CNTR value on low value of ptc_capt signal. The RPTC_LRC should have higher value than RPTC_HRC. This is because PWM output goes first high and later low."]
    #[inline(always)]
    pub const fn lrc(&self) -> &Lrc {
        &self.lrc
    }
    #[doc = "0x0c - Opencores PTC PWM v1 RPTC_CTRL register control operation of PTC core."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
}
#[doc = "cntr (rw) register accessor: Opencores PTC PWM v1 CNTR is the actual counter register. It is incremented at every counter/timer clock cycle. Source clock is either system clock or ptc_ecgt eclk/gate input. Selection between both clocks is performed with the RPTC_CTRL\\[ECLK\\]. Active edge of external clock is selected with the RPTC_CTRL\\[NEC\\]. In order to count, RPTC_CNTR must first be enabled with the RPTC_CTRL\\[EN\\]. RPTC_CNTR can be reset with the RPTC_CTRL\\[RST\\]. RPTC_CNTR can operate in either single-run mode or continues mode. Mode is selected with the RPTC_CTRL\\[SINGLE\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntr`]
module"]
#[doc(alias = "cntr")]
pub type Cntr = crate::Reg<cntr::CntrSpec>;
#[doc = "Opencores PTC PWM v1 CNTR is the actual counter register. It is incremented at every counter/timer clock cycle. Source clock is either system clock or ptc_ecgt eclk/gate input. Selection between both clocks is performed with the RPTC_CTRL\\[ECLK\\]. Active edge of external clock is selected with the RPTC_CTRL\\[NEC\\]. In order to count, RPTC_CNTR must first be enabled with the RPTC_CTRL\\[EN\\]. RPTC_CNTR can be reset with the RPTC_CTRL\\[RST\\]. RPTC_CNTR can operate in either single-run mode or continues mode. Mode is selected with the RPTC_CTRL\\[SINGLE\\]."]
pub mod cntr;
#[doc = "hrc (rw) register accessor: Opencores PTC PWM v1 HRC register is a 2nd out of two reference/capture registers. It has two functions: - In reference mode it is used to assert high PWM output or to generate an interrupt - In capture mode it captures RPTC_CNTR value on high value of ptc_capt signal. The RPTC_HRC should have lower value than RPTC_LRC. This is because PWM output goes first high and later low.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrc`]
module"]
#[doc(alias = "hrc")]
pub type Hrc = crate::Reg<hrc::HrcSpec>;
#[doc = "Opencores PTC PWM v1 HRC register is a 2nd out of two reference/capture registers. It has two functions: - In reference mode it is used to assert high PWM output or to generate an interrupt - In capture mode it captures RPTC_CNTR value on high value of ptc_capt signal. The RPTC_HRC should have lower value than RPTC_LRC. This is because PWM output goes first high and later low."]
pub mod hrc;
#[doc = "lrc (rw) register accessor: Opencores PTC PWM v1 RPTC_LRC register is a 1st out of two reference/capture registers. It has two functions: - In reference mode it is used to assert low PWM output or to generate an interrupt - In capture mode it captures RPTC_CNTR value on low value of ptc_capt signal. The RPTC_LRC should have higher value than RPTC_HRC. This is because PWM output goes first high and later low.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lrc`]
module"]
#[doc(alias = "lrc")]
pub type Lrc = crate::Reg<lrc::LrcSpec>;
#[doc = "Opencores PTC PWM v1 RPTC_LRC register is a 1st out of two reference/capture registers. It has two functions: - In reference mode it is used to assert low PWM output or to generate an interrupt - In capture mode it captures RPTC_CNTR value on low value of ptc_capt signal. The RPTC_LRC should have higher value than RPTC_HRC. This is because PWM output goes first high and later low."]
pub mod lrc;
#[doc = "ctrl (rw) register accessor: Opencores PTC PWM v1 RPTC_CTRL register control operation of PTC core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "ctrl")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Opencores PTC PWM v1 RPTC_CTRL register control operation of PTC core."]
pub mod ctrl;
