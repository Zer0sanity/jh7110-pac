#[doc = "Register `axi_id` reader"]
pub type R = crate::R<AxiIdSpec>;
#[doc = "Register `axi_id` writer"]
pub type W = crate::W<AxiIdSpec>;
#[doc = "Field `suffix(_read,_write)` reader - AXI ID suffix"]
pub type SuffixR = crate::FieldReader<u16>;
#[doc = "Field `suffix(_read,_write)` writer - AXI ID suffix"]
pub type SuffixW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "AXI ID suffix"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `suffix_read` field"]
    #[inline(always)]
    pub fn suffix(&self, n: u8) -> SuffixR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SuffixR::new(((self.bits >> (n * 16)) & 0xffff) as u16)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "AXI ID suffix"]
    #[inline(always)]
    pub fn suffix_iter(&self) -> impl Iterator<Item = SuffixR> + '_ {
        (0..2).map(move |n| SuffixR::new(((self.bits >> (n * 16)) & 0xffff) as u16))
    }
    #[doc = "Bits 0:15 - AXI ID suffix"]
    #[inline(always)]
    pub fn suffix_read(&self) -> SuffixR {
        SuffixR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - AXI ID suffix"]
    #[inline(always)]
    pub fn suffix_write(&self) -> SuffixR {
        SuffixR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "AXI ID suffix"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `suffix_read` field"]
    #[inline(always)]
    #[must_use]
    pub fn suffix(&mut self, n: u8) -> SuffixW<AxiIdSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SuffixW::new(self, n * 16)
    }
    #[doc = "Bits 0:15 - AXI ID suffix"]
    #[inline(always)]
    #[must_use]
    pub fn suffix_read(&mut self) -> SuffixW<AxiIdSpec> {
        SuffixW::new(self, 0)
    }
    #[doc = "Bits 16:31 - AXI ID suffix"]
    #[inline(always)]
    #[must_use]
    pub fn suffix_write(&mut self) -> SuffixW<AxiIdSpec> {
        SuffixW::new(self, 16)
    }
}
#[doc = "Channel AXI ID register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_id::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_id::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiIdSpec;
impl crate::RegisterSpec for AxiIdSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`axi_id::R`](R) reader structure"]
impl crate::Readable for AxiIdSpec {}
#[doc = "`write(|w| ..)` method takes [`axi_id::W`](W) writer structure"]
impl crate::Writable for AxiIdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets axi_id to value 0"]
impl crate::Resettable for AxiIdSpec {
    const RESET_VALUE: u64 = 0;
}
