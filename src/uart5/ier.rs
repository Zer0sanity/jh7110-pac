#[doc = "Register `ier` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `ier` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `erbfi` reader - Enable Received Data Available Interrupt. This is used to enable/disable the generation of Received Data Available Interrupt and the Character Timeout Interrupt (if in FIFO mode and FIFOs enabled). These are the second highest priority interrupts. 0 = disabled 1 = enabled"]
pub type ErbfiR = crate::BitReader;
#[doc = "Field `erbfi` writer - Enable Received Data Available Interrupt. This is used to enable/disable the generation of Received Data Available Interrupt and the Character Timeout Interrupt (if in FIFO mode and FIFOs enabled). These are the second highest priority interrupts. 0 = disabled 1 = enabled"]
pub type ErbfiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `etbei` reader - Enable Transmit Holding Register Empty Interrupt. This is used to enable/disable the generation of Transmitter Holding Register Empty Interrupt. This is the third highest priority interrupt. 0 = disabled 1 = enabled"]
pub type EtbeiR = crate::BitReader;
#[doc = "Field `etbei` writer - Enable Transmit Holding Register Empty Interrupt. This is used to enable/disable the generation of Transmitter Holding Register Empty Interrupt. This is the third highest priority interrupt. 0 = disabled 1 = enabled"]
pub type EtbeiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `elsi` reader - Enable Receiver Line Status Interrupt. This is used to enable/disable the generation of Receiver Line Status Interrupt. This is the highest priority interrupt. 0 = disabled 1 = enabled"]
pub type ElsiR = crate::BitReader;
#[doc = "Field `elsi` writer - Enable Receiver Line Status Interrupt. This is used to enable/disable the generation of Receiver Line Status Interrupt. This is the highest priority interrupt. 0 = disabled 1 = enabled"]
pub type ElsiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `edssi` reader - Enable Modem Status Interrupt. This is used to enable/disable the generation of Modem Status Interrupt. This is the fourth highest priority interrupt. 0 = disabled 1 = enabled"]
pub type EdssiR = crate::BitReader;
#[doc = "Field `edssi` writer - Enable Modem Status Interrupt. This is used to enable/disable the generation of Modem Status Interrupt. This is the fourth highest priority interrupt. 0 = disabled 1 = enabled"]
pub type EdssiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ptime` reader - Programmable THRE Interrupt Mode Enable that can be written to only when THRE_MODE_USER == Enabled, always readable. This is used to enable/disable the generation of THRE Interrupt. 0 = disabled 1 = enabled"]
pub type PtimeR = crate::BitReader;
#[doc = "Field `ptime` writer - Programmable THRE Interrupt Mode Enable that can be written to only when THRE_MODE_USER == Enabled, always readable. This is used to enable/disable the generation of THRE Interrupt. 0 = disabled 1 = enabled"]
pub type PtimeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Received Data Available Interrupt. This is used to enable/disable the generation of Received Data Available Interrupt and the Character Timeout Interrupt (if in FIFO mode and FIFOs enabled). These are the second highest priority interrupts. 0 = disabled 1 = enabled"]
    #[inline(always)]
    pub fn erbfi(&self) -> ErbfiR {
        ErbfiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Transmit Holding Register Empty Interrupt. This is used to enable/disable the generation of Transmitter Holding Register Empty Interrupt. This is the third highest priority interrupt. 0 = disabled 1 = enabled"]
    #[inline(always)]
    pub fn etbei(&self) -> EtbeiR {
        EtbeiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Receiver Line Status Interrupt. This is used to enable/disable the generation of Receiver Line Status Interrupt. This is the highest priority interrupt. 0 = disabled 1 = enabled"]
    #[inline(always)]
    pub fn elsi(&self) -> ElsiR {
        ElsiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Modem Status Interrupt. This is used to enable/disable the generation of Modem Status Interrupt. This is the fourth highest priority interrupt. 0 = disabled 1 = enabled"]
    #[inline(always)]
    pub fn edssi(&self) -> EdssiR {
        EdssiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Programmable THRE Interrupt Mode Enable that can be written to only when THRE_MODE_USER == Enabled, always readable. This is used to enable/disable the generation of THRE Interrupt. 0 = disabled 1 = enabled"]
    #[inline(always)]
    pub fn ptime(&self) -> PtimeR {
        PtimeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Received Data Available Interrupt. This is used to enable/disable the generation of Received Data Available Interrupt and the Character Timeout Interrupt (if in FIFO mode and FIFOs enabled). These are the second highest priority interrupts. 0 = disabled 1 = enabled"]
    #[inline(always)]
    #[must_use]
    pub fn erbfi(&mut self) -> ErbfiW<IerSpec> {
        ErbfiW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Transmit Holding Register Empty Interrupt. This is used to enable/disable the generation of Transmitter Holding Register Empty Interrupt. This is the third highest priority interrupt. 0 = disabled 1 = enabled"]
    #[inline(always)]
    #[must_use]
    pub fn etbei(&mut self) -> EtbeiW<IerSpec> {
        EtbeiW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Receiver Line Status Interrupt. This is used to enable/disable the generation of Receiver Line Status Interrupt. This is the highest priority interrupt. 0 = disabled 1 = enabled"]
    #[inline(always)]
    #[must_use]
    pub fn elsi(&mut self) -> ElsiW<IerSpec> {
        ElsiW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Modem Status Interrupt. This is used to enable/disable the generation of Modem Status Interrupt. This is the fourth highest priority interrupt. 0 = disabled 1 = enabled"]
    #[inline(always)]
    #[must_use]
    pub fn edssi(&mut self) -> EdssiW<IerSpec> {
        EdssiW::new(self, 3)
    }
    #[doc = "Bit 7 - Programmable THRE Interrupt Mode Enable that can be written to only when THRE_MODE_USER == Enabled, always readable. This is used to enable/disable the generation of THRE Interrupt. 0 = disabled 1 = enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ptime(&mut self) -> PtimeW<IerSpec> {
        PtimeW::new(self, 7)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ier to value 0"]
impl crate::Resettable for IerSpec {
    const RESET_VALUE: u32 = 0;
}
