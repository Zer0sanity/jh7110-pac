#[repr(C)]
#[doc = "Clock GPU registers"]
#[doc(alias = "clk_gpu")]
pub struct ClkGpu {
    root: Root,
}
impl ClkGpu {
    #[doc = "0x00 - Clock GPU Root"]
    #[inline(always)]
    pub const fn root(&self) -> &Root {
        &self.root
    }
}
#[doc = "root (rw) register accessor: Clock GPU Root\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`root::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`root::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@root`]
module"]
#[doc(alias = "root")]
pub type Root = crate::Reg<root::RootSpec>;
#[doc = "Clock GPU Root"]
pub mod root;
