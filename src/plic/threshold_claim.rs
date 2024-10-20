#[repr(C)]
#[doc = "PLIC threshold and claim_complete registers"]
#[doc(alias = "threshold_claim")]
pub struct ThresholdClaim {
    threshold: Threshold,
    claim_complete: ClaimComplete,
}
impl ThresholdClaim {
    #[doc = "0x00 - Interrupt priority threshold of each context. The PLIC will mask all PLIC interrupts of a priority less than or equal to `threshold`."]
    #[inline(always)]
    pub const fn threshold(&self) -> &Threshold {
        &self.threshold
    }
    #[doc = "0x04 - Interrupt source `claim` (read) and complete (write) register. The PLIC will write pending interrupt source information to the `claim` register. When the interrupt handler is finished, the interrupt source idendification should be written to the corresponding `complete` register."]
    #[inline(always)]
    pub const fn claim_complete(&self) -> &ClaimComplete {
        &self.claim_complete
    }
}
#[doc = "threshold (rw) register accessor: Interrupt priority threshold of each context. The PLIC will mask all PLIC interrupts of a priority less than or equal to `threshold`.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`threshold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`threshold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@threshold`]
module"]
#[doc(alias = "threshold")]
pub type Threshold = crate::Reg<threshold::ThresholdSpec>;
#[doc = "Interrupt priority threshold of each context. The PLIC will mask all PLIC interrupts of a priority less than or equal to `threshold`."]
pub mod threshold;
#[doc = "claim_complete (rw) register accessor: Interrupt source `claim` (read) and complete (write) register. The PLIC will write pending interrupt source information to the `claim` register. When the interrupt handler is finished, the interrupt source idendification should be written to the corresponding `complete` register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claim_complete::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claim_complete::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claim_complete`]
module"]
#[doc(alias = "claim_complete")]
pub type ClaimComplete = crate::Reg<claim_complete::ClaimCompleteSpec>;
#[doc = "Interrupt source `claim` (read) and complete (write) register. The PLIC will write pending interrupt source information to the `claim` register. When the interrupt handler is finished, the interrupt source idendification should be written to the corresponding `complete` register."]
pub mod claim_complete;
