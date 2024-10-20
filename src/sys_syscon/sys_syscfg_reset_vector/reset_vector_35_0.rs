#[repr(C)]
#[doc = "Reset vector cluster of 36 vector fields"]
#[doc(alias = "reset_vector_35_0")]
pub struct ResetVector35_0 {
    reset_vector_31_0: ResetVector31_0,
    reset_vector_35_32: ResetVector35_32,
}
impl ResetVector35_0 {
    #[doc = "0x00 - Reset vector register with 32 vector fields"]
    #[inline(always)]
    pub const fn reset_vector_31_0(&self) -> &ResetVector31_0 {
        &self.reset_vector_31_0
    }
    #[doc = "0x04 - Reset vector register with 4 vector fields"]
    #[inline(always)]
    pub const fn reset_vector_35_32(&self) -> &ResetVector35_32 {
        &self.reset_vector_35_32
    }
}
#[doc = "reset_vector_31_0 (rw) register accessor: Reset vector register with 32 vector fields\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_vector_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_vector_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_vector_31_0`]
module"]
#[doc(alias = "reset_vector_31_0")]
pub type ResetVector31_0 = crate::Reg<reset_vector_31_0::ResetVector31_0Spec>;
#[doc = "Reset vector register with 32 vector fields"]
pub mod reset_vector_31_0;
#[doc = "reset_vector_35_32 (rw) register accessor: Reset vector register with 4 vector fields\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_vector_35_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_vector_35_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_vector_35_32`]
module"]
#[doc(alias = "reset_vector_35_32")]
pub type ResetVector35_32 = crate::Reg<reset_vector_35_32::ResetVector35_32Spec>;
#[doc = "Reset vector register with 4 vector fields"]
pub mod reset_vector_35_32;
