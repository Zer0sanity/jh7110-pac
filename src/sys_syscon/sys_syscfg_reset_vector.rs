#[repr(C)]
#[doc = "SYS SYSCONSAIF SYSCFG 104 - 128: Reset Vector registers."]
#[doc(alias = "sys_syscfg_reset_vector")]
pub struct SysSyscfgResetVector {
    reset_vector_35_0: [ResetVector35_0; 3],
    reset_vector_31_0_4: ResetVector31_0_4,
}
impl SysSyscfgResetVector {
    #[doc = "0x00..0x18 - Reset vector cluster of 36 vector fields"]
    #[inline(always)]
    pub const fn reset_vector_35_0(&self, n: usize) -> &ResetVector35_0 {
        &self.reset_vector_35_0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x18 - Reset vector cluster of 36 vector fields"]
    #[inline(always)]
    pub fn reset_vector_35_0_iter(&self) -> impl Iterator<Item = &ResetVector35_0> {
        self.reset_vector_35_0.iter()
    }
    #[doc = "0x00..0x08 - Reset vector cluster of 36 vector fields"]
    #[inline(always)]
    pub const fn reset_vector_35_0_1(&self) -> &ResetVector35_0 {
        self.reset_vector_35_0(0)
    }
    #[doc = "0x08..0x10 - Reset vector cluster of 36 vector fields"]
    #[inline(always)]
    pub const fn reset_vector_35_0_2(&self) -> &ResetVector35_0 {
        self.reset_vector_35_0(1)
    }
    #[doc = "0x10..0x18 - Reset vector cluster of 36 vector fields"]
    #[inline(always)]
    pub const fn reset_vector_35_0_3(&self) -> &ResetVector35_0 {
        self.reset_vector_35_0(2)
    }
    #[doc = "0x18 - Reset vector register with 32 vector fields"]
    #[inline(always)]
    pub const fn reset_vector_31_0_4(&self) -> &ResetVector31_0_4 {
        &self.reset_vector_31_0_4
    }
}
#[doc = "Reset vector cluster of 36 vector fields"]
pub use self::reset_vector_35_0::ResetVector35_0;
#[doc = r"Cluster"]
#[doc = "Reset vector cluster of 36 vector fields"]
pub mod reset_vector_35_0;
#[doc = "reset_vector_31_0_4 (rw) register accessor: Reset vector register with 32 vector fields\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_vector_31_0_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_vector_31_0_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_vector_31_0_4`]
module"]
#[doc(alias = "reset_vector_31_0_4")]
pub type ResetVector31_0_4 = crate::Reg<reset_vector_31_0_4::ResetVector31_0_4Spec>;
#[doc = "Reset vector register with 32 vector fields"]
pub mod reset_vector_31_0_4;
