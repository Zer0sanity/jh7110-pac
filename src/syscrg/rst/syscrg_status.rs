#[repr(C)]
#[doc = "SYSCRG RESET Status"]
#[doc(alias = "syscrg_status")]
pub struct SyscrgStatus {
    rst0: Rst0,
    rst1: Rst1,
    rst2: Rst2,
    rst3: Rst3,
}
impl SyscrgStatus {
    #[doc = "0x00 - RESET 0"]
    #[inline(always)]
    pub const fn rst0(&self) -> &Rst0 {
        &self.rst0
    }
    #[doc = "0x04 - RESET 1"]
    #[inline(always)]
    pub const fn rst1(&self) -> &Rst1 {
        &self.rst1
    }
    #[doc = "0x08 - RESET 2"]
    #[inline(always)]
    pub const fn rst2(&self) -> &Rst2 {
        &self.rst2
    }
    #[doc = "0x0c - RESET 3"]
    #[inline(always)]
    pub const fn rst3(&self) -> &Rst3 {
        &self.rst3
    }
}
#[doc = "rst0 (rw) register accessor: RESET 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst0`]
module"]
#[doc(alias = "rst0")]
pub type Rst0 = crate::Reg<rst0::Rst0Spec>;
#[doc = "RESET 0"]
pub mod rst0;
#[doc = "rst1 (rw) register accessor: RESET 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst1`]
module"]
#[doc(alias = "rst1")]
pub type Rst1 = crate::Reg<rst1::Rst1Spec>;
#[doc = "RESET 1"]
pub mod rst1;
#[doc = "rst2 (rw) register accessor: RESET 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst2`]
module"]
#[doc(alias = "rst2")]
pub type Rst2 = crate::Reg<rst2::Rst2Spec>;
#[doc = "RESET 2"]
pub mod rst2;
#[doc = "rst3 (rw) register accessor: RESET 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst3`]
module"]
#[doc(alias = "rst3")]
pub type Rst3 = crate::Reg<rst3::Rst3Spec>;
#[doc = "RESET 3"]
pub mod rst3;
