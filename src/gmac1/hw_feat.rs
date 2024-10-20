#[repr(C)]
#[doc = "Hardware Feature registers"]
#[doc(alias = "hw_feat")]
pub struct HwFeat {
    features0: Features0,
    features1: Features1,
    features2: Features2,
    features3: Features3,
}
impl HwFeat {
    #[doc = "0x00 - Hardware Features 0"]
    #[inline(always)]
    pub const fn features0(&self) -> &Features0 {
        &self.features0
    }
    #[doc = "0x04 - Hardware Features 1"]
    #[inline(always)]
    pub const fn features1(&self) -> &Features1 {
        &self.features1
    }
    #[doc = "0x08 - Hardware Features 2"]
    #[inline(always)]
    pub const fn features2(&self) -> &Features2 {
        &self.features2
    }
    #[doc = "0x0c - Hardware Features 3"]
    #[inline(always)]
    pub const fn features3(&self) -> &Features3 {
        &self.features3
    }
}
#[doc = "features0 (rw) register accessor: Hardware Features 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`features0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`features0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@features0`]
module"]
#[doc(alias = "features0")]
pub type Features0 = crate::Reg<features0::Features0Spec>;
#[doc = "Hardware Features 0"]
pub mod features0;
#[doc = "features1 (rw) register accessor: Hardware Features 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`features1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`features1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@features1`]
module"]
#[doc(alias = "features1")]
pub type Features1 = crate::Reg<features1::Features1Spec>;
#[doc = "Hardware Features 1"]
pub mod features1;
#[doc = "features2 (rw) register accessor: Hardware Features 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`features2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`features2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@features2`]
module"]
#[doc(alias = "features2")]
pub type Features2 = crate::Reg<features2::Features2Spec>;
#[doc = "Hardware Features 2"]
pub mod features2;
#[doc = "features3 (rw) register accessor: Hardware Features 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`features3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`features3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@features3`]
module"]
#[doc(alias = "features3")]
pub type Features3 = crate::Reg<features3::Features3Spec>;
#[doc = "Hardware Features 3"]
pub mod features3;
