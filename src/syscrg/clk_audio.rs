#[repr(C)]
#[doc = "Clock Audio registers"]
#[doc(alias = "clk_audio")]
pub struct ClkAudio {
    root: Root,
}
impl ClkAudio {
    #[doc = "0x00 - Clock Audio Root"]
    #[inline(always)]
    pub const fn root(&self) -> &Root {
        &self.root
    }
}
#[doc = "root (rw) register accessor: Clock Audio Root\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`root::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`root::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@root`]
module"]
#[doc(alias = "root")]
pub type Root = crate::Reg<root::RootSpec>;
#[doc = "Clock Audio Root"]
pub mod root;
