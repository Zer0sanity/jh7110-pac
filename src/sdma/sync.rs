#[doc = "Register `sync` reader"]
pub type R = crate::R<SyncSpec>;
#[doc = "Register `sync` writer"]
pub type W = crate::W<SyncSpec>;
#[doc = "Field `sync` reader - DMAC Sync - synchronization logic for DMA request signals enabled or disabled. A LOW bit indicates that the synchronization logic for the request signals is enabled. A HIGH bit indicates that the synchronization logic is disabled."]
pub type SyncR = crate::FieldReader<u16>;
#[doc = "Field `sync` writer - DMAC Sync - synchronization logic for DMA request signals enabled or disabled. A LOW bit indicates that the synchronization logic for the request signals is enabled. A HIGH bit indicates that the synchronization logic is disabled."]
pub type SyncW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - DMAC Sync - synchronization logic for DMA request signals enabled or disabled. A LOW bit indicates that the synchronization logic for the request signals is enabled. A HIGH bit indicates that the synchronization logic is disabled."]
    #[inline(always)]
    pub fn sync(&self) -> SyncR {
        SyncR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DMAC Sync - synchronization logic for DMA request signals enabled or disabled. A LOW bit indicates that the synchronization logic for the request signals is enabled. A HIGH bit indicates that the synchronization logic is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn sync(&mut self) -> SyncW<SyncSpec> {
        SyncW::new(self, 0)
    }
}
#[doc = "DMA Synchronization register - enables or disables synchronization logic for the DMA request signals. A bit set to 0 enables the synchronization logic for a particular group of DMA requests. A bit set to 1 disables the synchronization logic for a particular group of DMA requests. This register is reset to 0, and synchronization logic enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sync::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sync::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncSpec;
impl crate::RegisterSpec for SyncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sync::R`](R) reader structure"]
impl crate::Readable for SyncSpec {}
#[doc = "`write(|w| ..)` method takes [`sync::W`](W) writer structure"]
impl crate::Writable for SyncSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sync to value 0"]
impl crate::Resettable for SyncSpec {
    const RESET_VALUE: u32 = 0;
}
