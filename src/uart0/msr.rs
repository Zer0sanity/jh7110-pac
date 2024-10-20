#[doc = "Register `msr` reader"]
pub type R = crate::R<MsrSpec>;
#[doc = "Field `dcts` reader - Delta Clear to Send. This is used to indicate that the modem control line cts_n has changed since the last time the MSR was read. 0 = no change on ctsdsr_n since last read of MSR 1 = change on ctsdsr_n since last read of MSR Reading the MSR clears the DCTS bit. In Loopback Mode (MCR\\[4\\]
= 1), DCTS reflects changes on MCR\\[1\\]
(RTS). Note, if the DCTS bit is not set and the cts_n signal is asserted (low) and a reset occurs (software or otherwise), then the DCTS bit is set when the reset is removed if the cts_n signal remains asserted."]
pub type DctsR = crate::BitReader;
#[doc = "Field `ddsr` reader - Delta Data Set Ready. This is used to indicate that the modem control line dsr_n has changed since the last time the MSR was read. 0 = no change on dsr_n since last read of MSR 1 = change on dsr_n since last read of MSR Reading the MSR clears the DDSR bit. In Loopback Mode (MCR\\[4\\]
= 1), DDSR reflects changes on MCR\\[0\\]
(DTR). Note, if the DDSR bit is not set and the dsr_n signal is asserted (low) and a reset occurs (software or otherwise), then the DDSR bit is set when the reset is removed if the dsr_n signal remains asserted."]
pub type DdsrR = crate::BitReader;
#[doc = "Field `teri` reader - Trailing Edge of Ring Indicator. This is used to indicate that a change on the input ri_n (from an active-low to an inactive-high state) has occurred since the last time the MSR was read. 0 = no change on ri_n since last read of MSR 1 = change on ri_n since last read of MSR Reading the MSR clears the TERI bit. In Loopback Mode (MCR\\[4\\]
= 1), TERI reflects when MCR\\[2\\]
(Out1) has changed state from a high to a low."]
pub type TeriR = crate::BitReader;
#[doc = "Field `ddcd` reader - Delta Data Carrier Detect. This is used to indicate that the modem control line dcd_n has changed since the last time the MSR was read. 0 = no change on dcd_n since last read of MSR 1 = change on dcd_n since last read of MSR Reading the MSR clears the DDCD bit. In Loopback Mode (MCR\\[4\\]
= 1), DDCD reflects changes on MCR\\[3\\]
(Out2). Note, if the DDCD bit is not set and the dcd_n signal is asserted (low) and a reset occurs (software or otherwise), then the DDCD bit is set when the reset is removed if the dcd_n signal remains asserted."]
pub type DdcdR = crate::BitReader;
#[doc = "Field `cts` reader - Clear to Send. This is used to indicate the current state of the modem control line cts_n. This bit is the complement of cts_n. When the Clear to Send input (cts_n) is asserted it is an indication that the modem or data set is ready to exchange data with the DW_apb_uart. 0 = cts_n input is de-asserted (logic 1) 1 = cts_n input is asserted (logic 0) In Loopback Mode (MCR\\[4\\]
= 1), CTS is the same as MCR\\[1\\]
(RTS)"]
pub type CtsR = crate::BitReader;
#[doc = "Field `dsr` reader - Data Set Ready. This is used to indicate the current state of the modem control line dsr_n. This bit is the complement of dsr_n. When the Data Set Ready input (dsr_n) is asserted it is an indication that the modem or data set is ready to establish communications with the DW_apb_uart. 0 = dsr_n input is de-asserted (logic 1) 1 = dsr_n input is asserted (logic 0) In Loopback Mode (MCR\\[4\\]
set to one), DSR is the same as MCR\\[0\\]
(DTR)."]
pub type DsrR = crate::BitReader;
#[doc = "Field `ri` reader - Ring Indicator. This is used to indicate the current state of the modem control line ri_n. This bit is the complement of ri_n. When the Ring Indicator input (ri_n) is asserted it is an indication that a telephone ringing signal has been received by the modem or data set. 0 = ri_n input is de-asserted (logic 1) 1 = ri_n input is asserted (logic 0) In Loopback Mode (MCR\\[4\\]
set to one), RI is the same as MCR\\[2\\]
(Out1)."]
pub type RiR = crate::BitReader;
#[doc = "Field `dcd` reader - Data Carrier Detect. This is used to indicate the current state of the modem control line dcd_n. This bit is the complement of dcd_n. When the Data Carrier Detect input (dcd_n) is asserted it is an indication that the carrier has been detected by the modem or data set. 0 = dcd_n input is de-asserted (logic 1) 1 = dcd_n input is asserted (logic 0) In Loopback Mode (MCR\\[4\\]
set to one), DCD is the same as MCR\\[3\\]
(Out2)."]
pub type DcdR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Delta Clear to Send. This is used to indicate that the modem control line cts_n has changed since the last time the MSR was read. 0 = no change on ctsdsr_n since last read of MSR 1 = change on ctsdsr_n since last read of MSR Reading the MSR clears the DCTS bit. In Loopback Mode (MCR\\[4\\]
= 1), DCTS reflects changes on MCR\\[1\\]
(RTS). Note, if the DCTS bit is not set and the cts_n signal is asserted (low) and a reset occurs (software or otherwise), then the DCTS bit is set when the reset is removed if the cts_n signal remains asserted."]
    #[inline(always)]
    pub fn dcts(&self) -> DctsR {
        DctsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Delta Data Set Ready. This is used to indicate that the modem control line dsr_n has changed since the last time the MSR was read. 0 = no change on dsr_n since last read of MSR 1 = change on dsr_n since last read of MSR Reading the MSR clears the DDSR bit. In Loopback Mode (MCR\\[4\\]
= 1), DDSR reflects changes on MCR\\[0\\]
(DTR). Note, if the DDSR bit is not set and the dsr_n signal is asserted (low) and a reset occurs (software or otherwise), then the DDSR bit is set when the reset is removed if the dsr_n signal remains asserted."]
    #[inline(always)]
    pub fn ddsr(&self) -> DdsrR {
        DdsrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Trailing Edge of Ring Indicator. This is used to indicate that a change on the input ri_n (from an active-low to an inactive-high state) has occurred since the last time the MSR was read. 0 = no change on ri_n since last read of MSR 1 = change on ri_n since last read of MSR Reading the MSR clears the TERI bit. In Loopback Mode (MCR\\[4\\]
= 1), TERI reflects when MCR\\[2\\]
(Out1) has changed state from a high to a low."]
    #[inline(always)]
    pub fn teri(&self) -> TeriR {
        TeriR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Delta Data Carrier Detect. This is used to indicate that the modem control line dcd_n has changed since the last time the MSR was read. 0 = no change on dcd_n since last read of MSR 1 = change on dcd_n since last read of MSR Reading the MSR clears the DDCD bit. In Loopback Mode (MCR\\[4\\]
= 1), DDCD reflects changes on MCR\\[3\\]
(Out2). Note, if the DDCD bit is not set and the dcd_n signal is asserted (low) and a reset occurs (software or otherwise), then the DDCD bit is set when the reset is removed if the dcd_n signal remains asserted."]
    #[inline(always)]
    pub fn ddcd(&self) -> DdcdR {
        DdcdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear to Send. This is used to indicate the current state of the modem control line cts_n. This bit is the complement of cts_n. When the Clear to Send input (cts_n) is asserted it is an indication that the modem or data set is ready to exchange data with the DW_apb_uart. 0 = cts_n input is de-asserted (logic 1) 1 = cts_n input is asserted (logic 0) In Loopback Mode (MCR\\[4\\]
= 1), CTS is the same as MCR\\[1\\]
(RTS)"]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Set Ready. This is used to indicate the current state of the modem control line dsr_n. This bit is the complement of dsr_n. When the Data Set Ready input (dsr_n) is asserted it is an indication that the modem or data set is ready to establish communications with the DW_apb_uart. 0 = dsr_n input is de-asserted (logic 1) 1 = dsr_n input is asserted (logic 0) In Loopback Mode (MCR\\[4\\]
set to one), DSR is the same as MCR\\[0\\]
(DTR)."]
    #[inline(always)]
    pub fn dsr(&self) -> DsrR {
        DsrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Ring Indicator. This is used to indicate the current state of the modem control line ri_n. This bit is the complement of ri_n. When the Ring Indicator input (ri_n) is asserted it is an indication that a telephone ringing signal has been received by the modem or data set. 0 = ri_n input is de-asserted (logic 1) 1 = ri_n input is asserted (logic 0) In Loopback Mode (MCR\\[4\\]
set to one), RI is the same as MCR\\[2\\]
(Out1)."]
    #[inline(always)]
    pub fn ri(&self) -> RiR {
        RiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data Carrier Detect. This is used to indicate the current state of the modem control line dcd_n. This bit is the complement of dcd_n. When the Data Carrier Detect input (dcd_n) is asserted it is an indication that the carrier has been detected by the modem or data set. 0 = dcd_n input is de-asserted (logic 1) 1 = dcd_n input is asserted (logic 0) In Loopback Mode (MCR\\[4\\]
set to one), DCD is the same as MCR\\[3\\]
(Out2)."]
    #[inline(always)]
    pub fn dcd(&self) -> DcdR {
        DcdR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Modem Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsrSpec;
impl crate::RegisterSpec for MsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msr::R`](R) reader structure"]
impl crate::Readable for MsrSpec {}
#[doc = "`reset()` method sets msr to value 0"]
impl crate::Resettable for MsrSpec {
    const RESET_VALUE: u32 = 0;
}
