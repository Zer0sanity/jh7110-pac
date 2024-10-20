#[doc = "Register `bufaddr[%s]` reader"]
pub type R = crate::R<BufaddrSpec>;
#[doc = "Register `bufaddr[%s]` writer"]
pub type W = crate::W<BufaddrSpec>;
#[doc = "Field `bufaddr` reader - MMC Internal DMAC reserved / buffer address"]
pub type BufaddrR = crate::FieldReader<u32>;
#[doc = "Field `bufaddr` writer - MMC Internal DMAC reserved / buffer address"]
pub type BufaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MMC Internal DMAC reserved / buffer address"]
    #[inline(always)]
    pub fn bufaddr(&self) -> BufaddrR {
        BufaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MMC Internal DMAC reserved / buffer address"]
    #[inline(always)]
    #[must_use]
    pub fn bufaddr(&mut self) -> BufaddrW<BufaddrSpec> {
        BufaddrW::new(self, 0)
    }
}
#[doc = "MMC internal DMAC reserved / buffer address - HCON\\[ADDR_CONFIG\\]
32-bit(0): reserved, HCON\\[ADDR_CONFIG\\]
64-bit(1): buffer address lower/upper 32-bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bufaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bufaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BufaddrSpec;
impl crate::RegisterSpec for BufaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bufaddr::R`](R) reader structure"]
impl crate::Readable for BufaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`bufaddr::W`](W) writer structure"]
impl crate::Writable for BufaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets bufaddr[%s]
to value 0"]
impl crate::Resettable for BufaddrSpec {
    const RESET_VALUE: u32 = 0;
}
