#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    word: [Word; 524288],
}
impl RegisterBlock {
    #[doc = "0x00..0x200000 - SiFive U74(MC) SRAM (L2 LIM) word"]
    #[inline(always)]
    pub const fn word(&self, n: usize) -> &Word {
        &self.word[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x200000 - SiFive U74(MC) SRAM (L2 LIM) word"]
    #[inline(always)]
    pub fn word_iter(&self) -> impl Iterator<Item = &Word> {
        self.word.iter()
    }
}
#[doc = "word (rw) register accessor: SiFive U74(MC) SRAM (L2 LIM) word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`word::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`word::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word`]
module"]
#[doc(alias = "word")]
pub type Word = crate::Reg<word::WordSpec>;
#[doc = "SiFive U74(MC) SRAM (L2 LIM) word"]
pub mod word;
