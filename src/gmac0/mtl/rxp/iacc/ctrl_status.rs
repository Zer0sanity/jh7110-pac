#[doc = "Register `ctrl_status` reader"]
pub type R = crate::R<CtrlStatusSpec>;
#[doc = "Register `ctrl_status` writer"]
pub type W = crate::W<CtrlStatusSpec>;
#[doc = "Field `addr` reader - MTL IACC Address"]
pub type AddrR = crate::FieldReader<u16>;
#[doc = "Field `addr` writer - MTL IACC Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `wrrdn` reader - MTL IACC WRRDN"]
pub type WrrdnR = crate::BitReader;
#[doc = "Field `wrrdn` writer - MTL IACC WRRDN"]
pub type WrrdnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rxpeiee` reader - MTL IACC RXP EIEE"]
pub type RxpeieeR = crate::BitReader;
#[doc = "Field `rxpeiee` writer - MTL IACC RXP EIEE"]
pub type RxpeieeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rxpeiec` reader - MTL IACC RXP EIEC"]
pub type RxpeiecR = crate::FieldReader;
#[doc = "Field `rxpeiec` writer - MTL IACC RXP EIEC"]
pub type RxpeiecW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `startbusy` reader - MTL IACC Start Busy"]
pub type StartbusyR = crate::BitReader;
#[doc = "Field `startbusy` writer - MTL IACC Start Busy"]
pub type StartbusyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - MTL IACC Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - MTL IACC WRRDN"]
    #[inline(always)]
    pub fn wrrdn(&self) -> WrrdnR {
        WrrdnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - MTL IACC RXP EIEE"]
    #[inline(always)]
    pub fn rxpeiee(&self) -> RxpeieeR {
        RxpeieeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - MTL IACC RXP EIEC"]
    #[inline(always)]
    pub fn rxpeiec(&self) -> RxpeiecR {
        RxpeiecR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 31 - MTL IACC Start Busy"]
    #[inline(always)]
    pub fn startbusy(&self) -> StartbusyR {
        StartbusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MTL IACC Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<CtrlStatusSpec> {
        AddrW::new(self, 0)
    }
    #[doc = "Bit 16 - MTL IACC WRRDN"]
    #[inline(always)]
    #[must_use]
    pub fn wrrdn(&mut self) -> WrrdnW<CtrlStatusSpec> {
        WrrdnW::new(self, 16)
    }
    #[doc = "Bit 20 - MTL IACC RXP EIEE"]
    #[inline(always)]
    #[must_use]
    pub fn rxpeiee(&mut self) -> RxpeieeW<CtrlStatusSpec> {
        RxpeieeW::new(self, 20)
    }
    #[doc = "Bits 21:22 - MTL IACC RXP EIEC"]
    #[inline(always)]
    #[must_use]
    pub fn rxpeiec(&mut self) -> RxpeiecW<CtrlStatusSpec> {
        RxpeiecW::new(self, 21)
    }
    #[doc = "Bit 31 - MTL IACC Start Busy"]
    #[inline(always)]
    #[must_use]
    pub fn startbusy(&mut self) -> StartbusyW<CtrlStatusSpec> {
        StartbusyW::new(self, 31)
    }
}
#[doc = "MTL RXP IACC Control and Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlStatusSpec;
impl crate::RegisterSpec for CtrlStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_status::R`](R) reader structure"]
impl crate::Readable for CtrlStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl_status::W`](W) writer structure"]
impl crate::Writable for CtrlStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ctrl_status to value 0"]
impl crate::Resettable for CtrlStatusSpec {
    const RESET_VALUE: u32 = 0;
}
