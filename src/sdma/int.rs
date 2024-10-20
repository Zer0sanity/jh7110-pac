#[repr(C)]
#[doc = "DMAC Interrupt registers"]
#[doc(alias = "int")]
pub struct Int {
    status: Status,
    tc_status: TcStatus,
    tc_clear: TcClear,
    error_status: ErrorStatus,
    error_clear: ErrorClear,
}
impl Int {
    #[doc = "0x00 - Interrupt Status Register - shows the status of the interrupts after masking. A HIGH bit indicates that a specific DMA channel interrupt request is active. You can generate the request from either the error or terminal count interrupt requests."]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x04 - Interrupt Terminal Count Status Register - indicates the status of the terminal count after masking. You must use this register in conjunction with the DMACIntStatus Register if you use the combined interrupt request, DMACINTR, to request interrupts. If you use the DMACINTTC interrupt request, then you only have to read the DMACIntTCStatus Register to ascertain the source of the interrupt request."]
    #[inline(always)]
    pub const fn tc_status(&self) -> &TcStatus {
        &self.tc_status
    }
    #[doc = "0x08 - Interrupt Terminal Count Clear Register - clears a terminal count interrupt request. When writing to this register, each data bit that is set HIGH causes the corresponding bit in the Status Register to be cleared. Data bits that are LOW have no effect on the corresponding bit in the register."]
    #[inline(always)]
    pub const fn tc_clear(&self) -> &TcClear {
        &self.tc_clear
    }
    #[doc = "0x0c - Interrupt Error Status Register - indicates the status of the error request after masking. You must use this register in conjunction with the DMACIntStatus Register if you use the combined interrupt request, DMACINTR, to request interrupts. If you use the DMACINTERR interrupt request, then only read the DMACIntErrorStatus Register."]
    #[inline(always)]
    pub const fn error_status(&self) -> &ErrorStatus {
        &self.error_status
    }
    #[doc = "0x10 - Interrupt Error Clear Register - clears the error interrupt requests. When writing to this register, each data bit that is HIGH causes the corresponding bit in the Status Register to be cleared. Data bits that are LOW have no effect on the corresponding bit in the register."]
    #[inline(always)]
    pub const fn error_clear(&self) -> &ErrorClear {
        &self.error_clear
    }
}
#[doc = "status (r) register accessor: Interrupt Status Register - shows the status of the interrupts after masking. A HIGH bit indicates that a specific DMA channel interrupt request is active. You can generate the request from either the error or terminal count interrupt requests.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "status")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Interrupt Status Register - shows the status of the interrupts after masking. A HIGH bit indicates that a specific DMA channel interrupt request is active. You can generate the request from either the error or terminal count interrupt requests."]
pub mod status;
#[doc = "tc_status (r) register accessor: Interrupt Terminal Count Status Register - indicates the status of the terminal count after masking. You must use this register in conjunction with the DMACIntStatus Register if you use the combined interrupt request, DMACINTR, to request interrupts. If you use the DMACINTTC interrupt request, then you only have to read the DMACIntTCStatus Register to ascertain the source of the interrupt request.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tc_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tc_status`]
module"]
#[doc(alias = "tc_status")]
pub type TcStatus = crate::Reg<tc_status::TcStatusSpec>;
#[doc = "Interrupt Terminal Count Status Register - indicates the status of the terminal count after masking. You must use this register in conjunction with the DMACIntStatus Register if you use the combined interrupt request, DMACINTR, to request interrupts. If you use the DMACINTTC interrupt request, then you only have to read the DMACIntTCStatus Register to ascertain the source of the interrupt request."]
pub mod tc_status;
#[doc = "tc_clear (w) register accessor: Interrupt Terminal Count Clear Register - clears a terminal count interrupt request. When writing to this register, each data bit that is set HIGH causes the corresponding bit in the Status Register to be cleared. Data bits that are LOW have no effect on the corresponding bit in the register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tc_clear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tc_clear`]
module"]
#[doc(alias = "tc_clear")]
pub type TcClear = crate::Reg<tc_clear::TcClearSpec>;
#[doc = "Interrupt Terminal Count Clear Register - clears a terminal count interrupt request. When writing to this register, each data bit that is set HIGH causes the corresponding bit in the Status Register to be cleared. Data bits that are LOW have no effect on the corresponding bit in the register."]
pub mod tc_clear;
#[doc = "error_status (r) register accessor: Interrupt Error Status Register - indicates the status of the error request after masking. You must use this register in conjunction with the DMACIntStatus Register if you use the combined interrupt request, DMACINTR, to request interrupts. If you use the DMACINTERR interrupt request, then only read the DMACIntErrorStatus Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`error_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@error_status`]
module"]
#[doc(alias = "error_status")]
pub type ErrorStatus = crate::Reg<error_status::ErrorStatusSpec>;
#[doc = "Interrupt Error Status Register - indicates the status of the error request after masking. You must use this register in conjunction with the DMACIntStatus Register if you use the combined interrupt request, DMACINTR, to request interrupts. If you use the DMACINTERR interrupt request, then only read the DMACIntErrorStatus Register."]
pub mod error_status;
#[doc = "error_clear (w) register accessor: Interrupt Error Clear Register - clears the error interrupt requests. When writing to this register, each data bit that is HIGH causes the corresponding bit in the Status Register to be cleared. Data bits that are LOW have no effect on the corresponding bit in the register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`error_clear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@error_clear`]
module"]
#[doc(alias = "error_clear")]
pub type ErrorClear = crate::Reg<error_clear::ErrorClearSpec>;
#[doc = "Interrupt Error Clear Register - clears the error interrupt requests. When writing to this register, each data bit that is HIGH causes the corresponding bit in the Status Register to be cleared. Data bits that are LOW have no effect on the corresponding bit in the register."]
pub mod error_clear;
