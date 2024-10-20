#[doc = "Register `data_cmd` reader"]
pub type R = crate::R<DataCmdSpec>;
#[doc = "Register `data_cmd` writer"]
pub type W = crate::W<DataCmdSpec>;
#[doc = "Field `dat` reader - Data Command Data Byte"]
pub type DatR = crate::FieldReader;
#[doc = "Field `dat` writer - Data Command Data Byte"]
pub type DatW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `read` reader - Data Command READ Bit - 0: Write, 1: Read"]
pub type ReadR = crate::BitReader;
#[doc = "Field `read` writer - Data Command READ Bit - 0: Write, 1: Read"]
pub type ReadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `stop` reader - Data Command STOP Bit - 0: Non-terminal DATA command byte, 1: Terminal DATA command byte"]
pub type StopR = crate::BitReader;
#[doc = "Field `stop` writer - Data Command STOP Bit - 0: Non-terminal DATA command byte, 1: Terminal DATA command byte"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `restart` reader - Data Command RESTART Bit - 0: Do not restart the transfer, 1: Restart the transfer"]
pub type RestartR = crate::BitReader;
#[doc = "Field `restart` writer - Data Command RESTART Bit - 0: Do not restart the transfer, 1: Restart the transfer"]
pub type RestartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `first_data_byte` reader - Data Command First Data Byte - 0: False, 1: True"]
pub type FirstDataByteR = crate::BitReader;
#[doc = "Field `first_data_byte` writer - Data Command First Data Byte - 0: False, 1: True"]
pub type FirstDataByteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Data Command Data Byte"]
    #[inline(always)]
    pub fn dat(&self) -> DatR {
        DatR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Data Command READ Bit - 0: Write, 1: Read"]
    #[inline(always)]
    pub fn read(&self) -> ReadR {
        ReadR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Data Command STOP Bit - 0: Non-terminal DATA command byte, 1: Terminal DATA command byte"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data Command RESTART Bit - 0: Do not restart the transfer, 1: Restart the transfer"]
    #[inline(always)]
    pub fn restart(&self) -> RestartR {
        RestartR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data Command First Data Byte - 0: False, 1: True"]
    #[inline(always)]
    pub fn first_data_byte(&self) -> FirstDataByteR {
        FirstDataByteR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Command Data Byte"]
    #[inline(always)]
    #[must_use]
    pub fn dat(&mut self) -> DatW<DataCmdSpec> {
        DatW::new(self, 0)
    }
    #[doc = "Bit 8 - Data Command READ Bit - 0: Write, 1: Read"]
    #[inline(always)]
    #[must_use]
    pub fn read(&mut self) -> ReadW<DataCmdSpec> {
        ReadW::new(self, 8)
    }
    #[doc = "Bit 9 - Data Command STOP Bit - 0: Non-terminal DATA command byte, 1: Terminal DATA command byte"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<DataCmdSpec> {
        StopW::new(self, 9)
    }
    #[doc = "Bit 10 - Data Command RESTART Bit - 0: Do not restart the transfer, 1: Restart the transfer"]
    #[inline(always)]
    #[must_use]
    pub fn restart(&mut self) -> RestartW<DataCmdSpec> {
        RestartW::new(self, 10)
    }
    #[doc = "Bit 11 - Data Command First Data Byte - 0: False, 1: True"]
    #[inline(always)]
    #[must_use]
    pub fn first_data_byte(&mut self) -> FirstDataByteW<DataCmdSpec> {
        FirstDataByteW::new(self, 11)
    }
}
#[doc = "DesignWare I2C Data Command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataCmdSpec;
impl crate::RegisterSpec for DataCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_cmd::R`](R) reader structure"]
impl crate::Readable for DataCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`data_cmd::W`](W) writer structure"]
impl crate::Writable for DataCmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets data_cmd to value 0"]
impl crate::Resettable for DataCmdSpec {
    const RESET_VALUE: u32 = 0;
}
