#[repr(C)]
#[doc = "Clock CODAJ12 registers"]
#[doc(alias = "clk_jpeg")]
pub struct ClkJpeg {
    codec_axi: CodecAxi,
    codaj12_axi: Codaj12Axi,
    codaj12_core: Codaj12Core,
    codaj12_apb: Codaj12Apb,
}
impl ClkJpeg {
    #[doc = "0x00 - Clock JPEG Codec AXI"]
    #[inline(always)]
    pub const fn codec_axi(&self) -> &CodecAxi {
        &self.codec_axi
    }
    #[doc = "0x04 - Clock CODAJ12 AXI"]
    #[inline(always)]
    pub const fn codaj12_axi(&self) -> &Codaj12Axi {
        &self.codaj12_axi
    }
    #[doc = "0x08 - Clock CODAJ12 Core"]
    #[inline(always)]
    pub const fn codaj12_core(&self) -> &Codaj12Core {
        &self.codaj12_core
    }
    #[doc = "0x0c - Clock CODAJ12 APB"]
    #[inline(always)]
    pub const fn codaj12_apb(&self) -> &Codaj12Apb {
        &self.codaj12_apb
    }
}
#[doc = "codec_axi (rw) register accessor: Clock JPEG Codec AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`codec_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`codec_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codec_axi`]
module"]
#[doc(alias = "codec_axi")]
pub type CodecAxi = crate::Reg<codec_axi::CodecAxiSpec>;
#[doc = "Clock JPEG Codec AXI"]
pub mod codec_axi;
#[doc = "codaj12_axi (rw) register accessor: Clock CODAJ12 AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`codaj12_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`codaj12_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codaj12_axi`]
module"]
#[doc(alias = "codaj12_axi")]
pub type Codaj12Axi = crate::Reg<codaj12_axi::Codaj12AxiSpec>;
#[doc = "Clock CODAJ12 AXI"]
pub mod codaj12_axi;
#[doc = "codaj12_core (rw) register accessor: Clock CODAJ12 Core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`codaj12_core::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`codaj12_core::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codaj12_core`]
module"]
#[doc(alias = "codaj12_core")]
pub type Codaj12Core = crate::Reg<codaj12_core::Codaj12CoreSpec>;
#[doc = "Clock CODAJ12 Core"]
pub mod codaj12_core;
#[doc = "codaj12_apb (rw) register accessor: Clock CODAJ12 APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`codaj12_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`codaj12_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codaj12_apb`]
module"]
#[doc(alias = "codaj12_apb")]
pub type Codaj12Apb = crate::Reg<codaj12_apb::Codaj12ApbSpec>;
#[doc = "Clock CODAJ12 APB"]
pub mod codaj12_apb;
