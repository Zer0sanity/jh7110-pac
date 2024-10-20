#[doc = "Register `rx_queue_ctrl[%s]` reader"]
pub type R = crate::R<RxQueueCtrlSpec>;
#[doc = "Register `rx_queue_ctrl[%s]` writer"]
pub type W = crate::W<RxQueueCtrlSpec>;
#[doc = "Field `avcpq` reader - AVCPQ"]
pub type AvcpqR = crate::FieldReader;
#[doc = "Field `avcpq` writer - AVCPQ"]
pub type AvcpqW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ptpq` reader - PTPQ"]
pub type PtpqR = crate::FieldReader;
#[doc = "Field `ptpq` writer - PTPQ"]
pub type PtpqW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dcbcpq` reader - DCBCPQ"]
pub type DcbcpqR = crate::FieldReader;
#[doc = "Field `dcbcpq` writer - DCBCPQ"]
pub type DcbcpqW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `upq` reader - UPQ"]
pub type UpqR = crate::FieldReader;
#[doc = "Field `upq` writer - UPQ"]
pub type UpqW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `mcbcq` reader - MCBCQ"]
pub type McbcqR = crate::FieldReader;
#[doc = "Field `mcbcq` writer - MCBCQ"]
pub type McbcqW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `mcbcqen` reader - MCBCQ Enable"]
pub type McbcqenR = crate::BitReader;
#[doc = "Field `mcbcqen` writer - MCBCQ Enable"]
pub type McbcqenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tacpqe` reader - TACPQE"]
pub type TacpqeR = crate::BitReader;
#[doc = "Field `tacpqe` writer - TACPQE"]
pub type TacpqeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fprq` reader - FPRQ"]
pub type FprqR = crate::FieldReader;
#[doc = "Field `fprq` writer - FPRQ"]
pub type FprqW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - AVCPQ"]
    #[inline(always)]
    pub fn avcpq(&self) -> AvcpqR {
        AvcpqR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - PTPQ"]
    #[inline(always)]
    pub fn ptpq(&self) -> PtpqR {
        PtpqR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - DCBCPQ"]
    #[inline(always)]
    pub fn dcbcpq(&self) -> DcbcpqR {
        DcbcpqR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - UPQ"]
    #[inline(always)]
    pub fn upq(&self) -> UpqR {
        UpqR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - MCBCQ"]
    #[inline(always)]
    pub fn mcbcq(&self) -> McbcqR {
        McbcqR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - MCBCQ Enable"]
    #[inline(always)]
    pub fn mcbcqen(&self) -> McbcqenR {
        McbcqenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TACPQE"]
    #[inline(always)]
    pub fn tacpqe(&self) -> TacpqeR {
        TacpqeR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:26 - FPRQ"]
    #[inline(always)]
    pub fn fprq(&self) -> FprqR {
        FprqR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - AVCPQ"]
    #[inline(always)]
    #[must_use]
    pub fn avcpq(&mut self) -> AvcpqW<RxQueueCtrlSpec> {
        AvcpqW::new(self, 0)
    }
    #[doc = "Bits 4:6 - PTPQ"]
    #[inline(always)]
    #[must_use]
    pub fn ptpq(&mut self) -> PtpqW<RxQueueCtrlSpec> {
        PtpqW::new(self, 4)
    }
    #[doc = "Bits 8:10 - DCBCPQ"]
    #[inline(always)]
    #[must_use]
    pub fn dcbcpq(&mut self) -> DcbcpqW<RxQueueCtrlSpec> {
        DcbcpqW::new(self, 8)
    }
    #[doc = "Bits 12:14 - UPQ"]
    #[inline(always)]
    #[must_use]
    pub fn upq(&mut self) -> UpqW<RxQueueCtrlSpec> {
        UpqW::new(self, 12)
    }
    #[doc = "Bits 16:18 - MCBCQ"]
    #[inline(always)]
    #[must_use]
    pub fn mcbcq(&mut self) -> McbcqW<RxQueueCtrlSpec> {
        McbcqW::new(self, 16)
    }
    #[doc = "Bit 20 - MCBCQ Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mcbcqen(&mut self) -> McbcqenW<RxQueueCtrlSpec> {
        McbcqenW::new(self, 20)
    }
    #[doc = "Bit 21 - TACPQE"]
    #[inline(always)]
    #[must_use]
    pub fn tacpqe(&mut self) -> TacpqeW<RxQueueCtrlSpec> {
        TacpqeW::new(self, 21)
    }
    #[doc = "Bits 24:26 - FPRQ"]
    #[inline(always)]
    #[must_use]
    pub fn fprq(&mut self) -> FprqW<RxQueueCtrlSpec> {
        FprqW::new(self, 24)
    }
}
#[doc = "MAC RX Queue Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_queue_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_queue_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxQueueCtrlSpec;
impl crate::RegisterSpec for RxQueueCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_queue_ctrl::R`](R) reader structure"]
impl crate::Readable for RxQueueCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_queue_ctrl::W`](W) writer structure"]
impl crate::Writable for RxQueueCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rx_queue_ctrl[%s]
to value 0"]
impl crate::Resettable for RxQueueCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
