#[repr(C)]
#[doc = "Clock Timer"]
#[doc(alias = "clk_tim")]
pub struct ClkTim {
    apb: Apb,
    tim01: [Tim01; 2],
    tim23: [Tim23; 2],
}
impl ClkTim {
    #[doc = "0x00 - Clock Timer APB"]
    #[inline(always)]
    pub const fn apb(&self) -> &Apb {
        &self.apb
    }
    #[doc = "0x04..0x0c - Clock Timer 0-1"]
    #[inline(always)]
    pub const fn tim01(&self, n: usize) -> &Tim01 {
        &self.tim01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x0c - Clock Timer 0-1"]
    #[inline(always)]
    pub fn tim01_iter(&self) -> impl Iterator<Item = &Tim01> {
        self.tim01.iter()
    }
    #[doc = "0x04 - Clock Timer 0-1"]
    #[inline(always)]
    pub const fn tim01_0(&self) -> &Tim01 {
        self.tim01(0)
    }
    #[doc = "0x08 - Clock Timer 0-1"]
    #[inline(always)]
    pub const fn tim01_1(&self) -> &Tim01 {
        self.tim01(1)
    }
    #[doc = "0x0c..0x14 - Clock Timer: 2-3"]
    #[inline(always)]
    pub const fn tim23(&self, n: usize) -> &Tim23 {
        &self.tim23[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c..0x14 - Clock Timer: 2-3"]
    #[inline(always)]
    pub fn tim23_iter(&self) -> impl Iterator<Item = &Tim23> {
        self.tim23.iter()
    }
    #[doc = "0x0c - Clock Timer: 2-3"]
    #[inline(always)]
    pub const fn tim23_2(&self) -> &Tim23 {
        self.tim23(0)
    }
    #[doc = "0x10 - Clock Timer: 2-3"]
    #[inline(always)]
    pub const fn tim23_3(&self) -> &Tim23 {
        self.tim23(1)
    }
}
#[doc = "apb (rw) register accessor: Clock Timer APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb`]
module"]
#[doc(alias = "apb")]
pub type Apb = crate::Reg<apb::ApbSpec>;
#[doc = "Clock Timer APB"]
pub mod apb;
#[doc = "tim01 (rw) register accessor: Clock Timer 0-1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim01`]
module"]
#[doc(alias = "tim01")]
pub type Tim01 = crate::Reg<tim01::Tim01Spec>;
#[doc = "Clock Timer 0-1"]
pub mod tim01;
#[doc = "tim23 (rw) register accessor: Clock Timer: 2-3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim23`]
module"]
#[doc(alias = "tim23")]
pub type Tim23 = crate::Reg<tim23::Tim23Spec>;
#[doc = "Clock Timer: 2-3"]
pub mod tim23;
