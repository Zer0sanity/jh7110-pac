#[repr(C)]
#[doc = "L2 Cache Control Directory Flush registers. Can be used for flushing specific cache blocks."]
#[doc(alias = "flush")]
pub struct Flush {
    flush64: Flush64,
    _reserved1: [u8; 0x38],
    flush32: Flush32,
}
impl Flush {
    #[doc = "0x00..0x08 - L2 Cache Control Flush 64-bit register. Flushes the cache block at the 64-bit address written."]
    #[inline(always)]
    pub const fn flush64(&self) -> &Flush64 {
        &self.flush64
    }
    #[doc = "0x40 - L2 Cache Control Flush 32-bit register. Flushes the cache block at the 32-bit address shifted left by 4 bytes."]
    #[inline(always)]
    pub const fn flush32(&self) -> &Flush32 {
        &self.flush32
    }
}
#[doc = "flush64 (w) register accessor: L2 Cache Control Flush 64-bit register. Flushes the cache block at the 64-bit address written.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flush64::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flush64`]
module"]
#[doc(alias = "flush64")]
pub type Flush64 = crate::Reg<flush64::Flush64Spec>;
#[doc = "L2 Cache Control Flush 64-bit register. Flushes the cache block at the 64-bit address written."]
pub mod flush64;
#[doc = "flush32 (w) register accessor: L2 Cache Control Flush 32-bit register. Flushes the cache block at the 32-bit address shifted left by 4 bytes.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flush32::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flush32`]
module"]
#[doc(alias = "flush32")]
pub type Flush32 = crate::Reg<flush32::Flush32Spec>;
#[doc = "L2 Cache Control Flush 32-bit register. Flushes the cache block at the 32-bit address shifted left by 4 bytes."]
pub mod flush32;
