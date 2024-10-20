#[repr(C)]
#[doc = "DMAC Raw Interrupt registers"]
#[doc(alias = "raw_int")]
pub struct RawInt {
    tc_status: TcStatus,
    error_status: ErrorStatus,
}
impl RawInt {
    #[doc = "0x00 - Raw Interrupt Terminal Count Status Register - indicates the DMA channels that are requesting a transfer complete, terminal count interrupt, prior to masking. A HIGH bit indicates that the terminal count interrupt request is active prior to masking."]
    #[inline(always)]
    pub const fn tc_status(&self) -> &TcStatus {
        &self.tc_status
    }
    #[doc = "0x04 - Raw Error Interrupt Status Register - indicates the DMA channels that are requesting an error interrupt prior to masking. A HIGH bit indicates that the error interrupt request is active prior to masking."]
    #[inline(always)]
    pub const fn error_status(&self) -> &ErrorStatus {
        &self.error_status
    }
}
#[doc = "tc_status (r) register accessor: Raw Interrupt Terminal Count Status Register - indicates the DMA channels that are requesting a transfer complete, terminal count interrupt, prior to masking. A HIGH bit indicates that the terminal count interrupt request is active prior to masking.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tc_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tc_status`]
module"]
#[doc(alias = "tc_status")]
pub type TcStatus = crate::Reg<tc_status::TcStatusSpec>;
#[doc = "Raw Interrupt Terminal Count Status Register - indicates the DMA channels that are requesting a transfer complete, terminal count interrupt, prior to masking. A HIGH bit indicates that the terminal count interrupt request is active prior to masking."]
pub mod tc_status;
#[doc = "error_status (r) register accessor: Raw Error Interrupt Status Register - indicates the DMA channels that are requesting an error interrupt prior to masking. A HIGH bit indicates that the error interrupt request is active prior to masking.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`error_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@error_status`]
module"]
#[doc(alias = "error_status")]
pub type ErrorStatus = crate::Reg<error_status::ErrorStatusSpec>;
#[doc = "Raw Error Interrupt Status Register - indicates the DMA channels that are requesting an error interrupt prior to masking. A HIGH bit indicates that the error interrupt request is active prior to masking."]
pub mod error_status;
