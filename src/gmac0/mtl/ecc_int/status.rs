#[doc = "Register `status` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `status` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `txceis` reader - MTL ECC TXCE Interrupt Status - Write 1 to clear interrupt"]
pub type TxceisR = crate::BitReader;
#[doc = "Field `txceis` writer - MTL ECC TXCE Interrupt Status - Write 1 to clear interrupt"]
pub type TxceisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rxceis` reader - MTL ECC RXCE Interrupt Status - Write 1 to clear interrupt"]
pub type RxceisR = crate::BitReader;
#[doc = "Field `rxceis` writer - MTL ECC RXCE Interrupt Status - Write 1 to clear interrupt"]
pub type RxceisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `eceis` reader - MTL ECC ECE Interrupt Status - Write 1 to clear interrupt"]
pub type EceisR = crate::BitReader;
#[doc = "Field `eceis` writer - MTL ECC ECE Interrupt Status - Write 1 to clear interrupt"]
pub type EceisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rpceis` reader - MTL ECC RPCE Interrupt Status - Write 1 to clear interrupt"]
pub type RpceisR = crate::BitReader;
#[doc = "Field `rpceis` writer - MTL ECC RPCE Interrupt Status - Write 1 to clear interrupt"]
pub type RpceisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MTL ECC TXCE Interrupt Status - Write 1 to clear interrupt"]
    #[inline(always)]
    pub fn txceis(&self) -> TxceisR {
        TxceisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - MTL ECC RXCE Interrupt Status - Write 1 to clear interrupt"]
    #[inline(always)]
    pub fn rxceis(&self) -> RxceisR {
        RxceisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - MTL ECC ECE Interrupt Status - Write 1 to clear interrupt"]
    #[inline(always)]
    pub fn eceis(&self) -> EceisR {
        EceisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - MTL ECC RPCE Interrupt Status - Write 1 to clear interrupt"]
    #[inline(always)]
    pub fn rpceis(&self) -> RpceisR {
        RpceisR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MTL ECC TXCE Interrupt Status - Write 1 to clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txceis(&mut self) -> TxceisW<StatusSpec> {
        TxceisW::new(self, 0)
    }
    #[doc = "Bit 4 - MTL ECC RXCE Interrupt Status - Write 1 to clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxceis(&mut self) -> RxceisW<StatusSpec> {
        RxceisW::new(self, 4)
    }
    #[doc = "Bit 8 - MTL ECC ECE Interrupt Status - Write 1 to clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eceis(&mut self) -> EceisW<StatusSpec> {
        EceisW::new(self, 8)
    }
    #[doc = "Bit 12 - MTL ECC RPCE Interrupt Status - Write 1 to clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rpceis(&mut self) -> RpceisW<StatusSpec> {
        RpceisW::new(self, 12)
    }
}
#[doc = "MTL ECC Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets status to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
