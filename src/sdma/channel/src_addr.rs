#[doc = "Register `src_addr` reader"]
pub type R = crate::R<SrcAddrSpec>;
#[doc = "Register `src_addr` writer"]
pub type W = crate::W<SrcAddrSpec>;
#[doc = "Field `src_addr` reader - DMA source address."]
pub type SrcAddrR = crate::FieldReader<u32>;
#[doc = "Field `src_addr` writer - DMA source address."]
pub type SrcAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA source address."]
    #[inline(always)]
    pub fn src_addr(&self) -> SrcAddrR {
        SrcAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA source address."]
    #[inline(always)]
    #[must_use]
    pub fn src_addr(&mut self) -> SrcAddrW<SrcAddrSpec> {
        SrcAddrW::new(self, 0)
    }
}
#[doc = "DMAC Source Address register - contain the current source address, byte-aligned, of the data to be transferred. Software programs each register directly before the appropriate channel is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcAddrSpec;
impl crate::RegisterSpec for SrcAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_addr::R`](R) reader structure"]
impl crate::Readable for SrcAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`src_addr::W`](W) writer structure"]
impl crate::Writable for SrcAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets src_addr to value 0"]
impl crate::Resettable for SrcAddrSpec {
    const RESET_VALUE: u32 = 0;
}
