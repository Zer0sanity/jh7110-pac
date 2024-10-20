#[doc = "Register `dbaddr_dbaddrl` reader"]
pub type R = crate::R<DbaddrDbaddrlSpec>;
#[doc = "Register `dbaddr_dbaddrl` writer"]
pub type W = crate::W<DbaddrDbaddrlSpec>;
#[doc = "Field `dbaddr_dbaddrl` reader - MMC Internal DMAC DB address"]
pub type DbaddrDbaddrlR = crate::FieldReader<u32>;
#[doc = "Field `dbaddr_dbaddrl` writer - MMC Internal DMAC DB address"]
pub type DbaddrDbaddrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MMC Internal DMAC DB address"]
    #[inline(always)]
    pub fn dbaddr_dbaddrl(&self) -> DbaddrDbaddrlR {
        DbaddrDbaddrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MMC Internal DMAC DB address"]
    #[inline(always)]
    #[must_use]
    pub fn dbaddr_dbaddrl(&mut self) -> DbaddrDbaddrlW<DbaddrDbaddrlSpec> {
        DbaddrDbaddrlW::new(self, 0)
    }
}
#[doc = "MMC internal DMAC DB address - HCON\\[ADDR_CONFIG\\]
32-bit(0): DB address, HCON\\[ADDR_CONFIG\\]
64-bit(1): DB address lower 32-bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbaddr_dbaddrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbaddr_dbaddrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbaddrDbaddrlSpec;
impl crate::RegisterSpec for DbaddrDbaddrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbaddr_dbaddrl::R`](R) reader structure"]
impl crate::Readable for DbaddrDbaddrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dbaddr_dbaddrl::W`](W) writer structure"]
impl crate::Writable for DbaddrDbaddrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dbaddr_dbaddrl to value 0"]
impl crate::Resettable for DbaddrDbaddrlSpec {
    const RESET_VALUE: u32 = 0;
}
