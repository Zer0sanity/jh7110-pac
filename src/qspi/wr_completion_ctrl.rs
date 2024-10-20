#[doc = "Register `wr_completion_ctrl` reader"]
pub type R = crate::R<WrCompletionCtrlSpec>;
#[doc = "Register `wr_completion_ctrl` writer"]
pub type W = crate::W<WrCompletionCtrlSpec>;
#[doc = "Field `disable_auto_poll` reader - SPI NAND flashes require the address of the status register to be passed in the Read SR command. Also, some SPI NOR flashes like the Cypress Semper flash expect a 4-byte dummy address in the Read SR command in DTR mode. But this controller does not support address phase in the Read SR command when doing auto-HW polling. So, disable write completion polling on the controller's side. spi-nand and spi-nor will take care of polling the status register."]
pub type DisableAutoPollR = crate::BitReader;
#[doc = "Field `disable_auto_poll` writer - SPI NAND flashes require the address of the status register to be passed in the Read SR command. Also, some SPI NOR flashes like the Cypress Semper flash expect a 4-byte dummy address in the Read SR command in DTR mode. But this controller does not support address phase in the Read SR command when doing auto-HW polling. So, disable write completion polling on the controller's side. spi-nand and spi-nor will take care of polling the status register."]
pub type DisableAutoPollW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 14 - SPI NAND flashes require the address of the status register to be passed in the Read SR command. Also, some SPI NOR flashes like the Cypress Semper flash expect a 4-byte dummy address in the Read SR command in DTR mode. But this controller does not support address phase in the Read SR command when doing auto-HW polling. So, disable write completion polling on the controller's side. spi-nand and spi-nor will take care of polling the status register."]
    #[inline(always)]
    pub fn disable_auto_poll(&self) -> DisableAutoPollR {
        DisableAutoPollR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - SPI NAND flashes require the address of the status register to be passed in the Read SR command. Also, some SPI NOR flashes like the Cypress Semper flash expect a 4-byte dummy address in the Read SR command in DTR mode. But this controller does not support address phase in the Read SR command when doing auto-HW polling. So, disable write completion polling on the controller's side. spi-nand and spi-nor will take care of polling the status register."]
    #[inline(always)]
    #[must_use]
    pub fn disable_auto_poll(&mut self) -> DisableAutoPollW<WrCompletionCtrlSpec> {
        DisableAutoPollW::new(self, 14)
    }
}
#[doc = "Cadence QSPI Write Completion Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_completion_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_completion_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WrCompletionCtrlSpec;
impl crate::RegisterSpec for WrCompletionCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wr_completion_ctrl::R`](R) reader structure"]
impl crate::Readable for WrCompletionCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`wr_completion_ctrl::W`](W) writer structure"]
impl crate::Writable for WrCompletionCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets wr_completion_ctrl to value 0"]
impl crate::Resettable for WrCompletionCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
