#[repr(C)]
#[doc = "Clock MCLK registers"]
#[doc(alias = "clk_mclk")]
pub struct ClkMclk {
    inner: Inner,
    mclk: Mclk,
    out: Out,
}
impl ClkMclk {
    #[doc = "0x00 - Clock MCLK Inner"]
    #[inline(always)]
    pub const fn inner(&self) -> &Inner {
        &self.inner
    }
    #[doc = "0x04 - Clock MCLK"]
    #[inline(always)]
    pub const fn mclk(&self) -> &Mclk {
        &self.mclk
    }
    #[doc = "0x08 - Clock MCLK Out"]
    #[inline(always)]
    pub const fn out(&self) -> &Out {
        &self.out
    }
}
#[doc = "inner (rw) register accessor: Clock MCLK Inner\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inner::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inner::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inner`]
module"]
#[doc(alias = "inner")]
pub type Inner = crate::Reg<inner::InnerSpec>;
#[doc = "Clock MCLK Inner"]
pub mod inner;
#[doc = "mclk (rw) register accessor: Clock MCLK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mclk`]
module"]
#[doc(alias = "mclk")]
pub type Mclk = crate::Reg<mclk::MclkSpec>;
#[doc = "Clock MCLK"]
pub mod mclk;
#[doc = "out (rw) register accessor: Clock MCLK Out\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out`]
module"]
#[doc(alias = "out")]
pub type Out = crate::Reg<out::OutSpec>;
#[doc = "Clock MCLK Out"]
pub mod out;
