#[doc = "Register `bufaddr_dscaddrl` reader"]
pub type R = crate::R<BufaddrDscaddrlSpec>;
#[doc = "Register `bufaddr_dscaddrl` writer"]
pub type W = crate::W<BufaddrDscaddrlSpec>;
#[doc = "Field `bufaddr_dscaddrl` reader - MMC Internal DMAC buffer address / DSC address"]
pub type BufaddrDscaddrlR = crate::FieldReader<u32>;
#[doc = "Field `bufaddr_dscaddrl` writer - MMC Internal DMAC buffer address / DSC address"]
pub type BufaddrDscaddrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MMC Internal DMAC buffer address / DSC address"]
    #[inline(always)]
    pub fn bufaddr_dscaddrl(&self) -> BufaddrDscaddrlR {
        BufaddrDscaddrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MMC Internal DMAC buffer address / DSC address"]
    #[inline(always)]
    #[must_use]
    pub fn bufaddr_dscaddrl(&mut self) -> BufaddrDscaddrlW<BufaddrDscaddrlSpec> {
        BufaddrDscaddrlW::new(self, 0)
    }
}
#[doc = "MMC internal DMAC buffer address / DSC address - HCON\\[ADDR_CONFIG\\]
32-bit(0): buffer address, HCON\\[ADDR_CONFIG\\]
64-bit(1): DSC address lower 32-bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bufaddr_dscaddrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bufaddr_dscaddrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BufaddrDscaddrlSpec;
impl crate::RegisterSpec for BufaddrDscaddrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bufaddr_dscaddrl::R`](R) reader structure"]
impl crate::Readable for BufaddrDscaddrlSpec {}
#[doc = "`write(|w| ..)` method takes [`bufaddr_dscaddrl::W`](W) writer structure"]
impl crate::Writable for BufaddrDscaddrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets bufaddr_dscaddrl to value 0"]
impl crate::Resettable for BufaddrDscaddrlSpec {
    const RESET_VALUE: u32 = 0;
}
