#[repr(C)]
#[doc = "USB3 device registers"]
#[doc(alias = "device")]
pub struct Device {
    usb_conf: UsbConf,
    usb_sts: UsbSts,
    usb_cmd: UsbCmd,
    usb_itpn: UsbItpn,
    usb_lpm: UsbLpm,
    usb_int: UsbInt,
    ep_sel: EpSel,
    ep_traddr: EpTraddr,
    ep_cfg: EpCfg,
    ep_cmd: EpCmd,
    ep_sts: EpSts,
    drbl: Drbl,
    ep_int: [EpInt; 2],
    usb_pwr: UsbPwr,
    usb_conf2: UsbConf2,
    usb_cap: UsbCap,
    usb_cpkt: [UsbCpkt; 3],
    ep_dma_ext_addr: EpDmaExtAddr,
    buf: Buf,
    dtrans: Dtrans,
    tdl: Tdl,
    _reserved21: [u8; 0x68],
    cfg1: Cfg1,
    dbg_link1: DbgLink1,
    dbg_link2: DbgLink2,
    cfg: [Cfg; 74],
    _reserved25: [u8; 0xcc],
    dma_axi: DmaAxi,
}
impl Device {
    #[doc = "0x00 - USB3 Global configuration."]
    #[inline(always)]
    pub const fn usb_conf(&self) -> &UsbConf {
        &self.usb_conf
    }
    #[doc = "0x04 - USB3 Global status."]
    #[inline(always)]
    pub const fn usb_sts(&self) -> &UsbSts {
        &self.usb_sts
    }
    #[doc = "0x08 - USB3 Global command."]
    #[inline(always)]
    pub const fn usb_cmd(&self) -> &UsbCmd {
        &self.usb_cmd
    }
    #[doc = "0x0c - ITP (SS) / SOF (FS/HS) number - SS: last ITP number, FS/HS: last SOF number."]
    #[inline(always)]
    pub const fn usb_itpn(&self) -> &UsbItpn {
        &self.usb_itpn
    }
    #[doc = "0x10 - Global LPM."]
    #[inline(always)]
    pub const fn usb_lpm(&self) -> &UsbLpm {
        &self.usb_lpm
    }
    #[doc = "0x14..0x1c - USB Interrupt registers."]
    #[inline(always)]
    pub const fn usb_int(&self) -> &UsbInt {
        &self.usb_int
    }
    #[doc = "0x1c - USB3 Endpoint select."]
    #[inline(always)]
    pub const fn ep_sel(&self) -> &EpSel {
        &self.ep_sel
    }
    #[doc = "0x20 - USB3 Endpoint transfer address."]
    #[inline(always)]
    pub const fn ep_traddr(&self) -> &EpTraddr {
        &self.ep_traddr
    }
    #[doc = "0x24 - USB3 Endpoint configuration."]
    #[inline(always)]
    pub const fn ep_cfg(&self) -> &EpCfg {
        &self.ep_cfg
    }
    #[doc = "0x28 - USB3 Endpoint command."]
    #[inline(always)]
    pub const fn ep_cmd(&self) -> &EpCmd {
        &self.ep_cmd
    }
    #[doc = "0x2c..0x38 - USB3 Endpoint status registers."]
    #[inline(always)]
    pub const fn ep_sts(&self) -> &EpSts {
        &self.ep_sts
    }
    #[doc = "0x38 - USB3 doorbell."]
    #[inline(always)]
    pub const fn drbl(&self) -> &Drbl {
        &self.drbl
    }
    #[doc = "0x3c..0x44 - USB3 Endpoint interrupt registers - ep_int0: enable, ep_int1: status."]
    #[inline(always)]
    pub const fn ep_int(&self, n: usize) -> &EpInt {
        &self.ep_int[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3c..0x44 - USB3 Endpoint interrupt registers - ep_int0: enable, ep_int1: status."]
    #[inline(always)]
    pub fn ep_int_iter(&self) -> impl Iterator<Item = &EpInt> {
        self.ep_int.iter()
    }
    #[doc = "0x3c - USB3 Endpoint interrupt registers - ep_int0: enable, ep_int1: status."]
    #[inline(always)]
    pub const fn ep_int_en(&self) -> &EpInt {
        self.ep_int(0)
    }
    #[doc = "0x40 - USB3 Endpoint interrupt registers - ep_int0: enable, ep_int1: status."]
    #[inline(always)]
    pub const fn ep_int_sts(&self) -> &EpInt {
        self.ep_int(1)
    }
    #[doc = "0x44 - USB3 Global power."]
    #[inline(always)]
    pub const fn usb_pwr(&self) -> &UsbPwr {
        &self.usb_pwr
    }
    #[doc = "0x48 - USB3 Global configurartion 2."]
    #[inline(always)]
    pub const fn usb_conf2(&self) -> &UsbConf2 {
        &self.usb_conf2
    }
    #[doc = "0x4c..0x64 - USB3 Global Capability registers."]
    #[inline(always)]
    pub const fn usb_cap(&self) -> &UsbCap {
        &self.usb_cap
    }
    #[doc = "0x64..0x70 - USB3 Global custom packet."]
    #[inline(always)]
    pub const fn usb_cpkt(&self, n: usize) -> &UsbCpkt {
        &self.usb_cpkt[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x64..0x70 - USB3 Global custom packet."]
    #[inline(always)]
    pub fn usb_cpkt_iter(&self) -> impl Iterator<Item = &UsbCpkt> {
        self.usb_cpkt.iter()
    }
    #[doc = "0x64 - USB3 Global custom packet."]
    #[inline(always)]
    pub const fn usb_cpkt1(&self) -> &UsbCpkt {
        self.usb_cpkt(0)
    }
    #[doc = "0x68 - USB3 Global custom packet."]
    #[inline(always)]
    pub const fn usb_cpkt2(&self) -> &UsbCpkt {
        self.usb_cpkt(1)
    }
    #[doc = "0x6c - USB3 Global custom packet."]
    #[inline(always)]
    pub const fn usb_cpkt3(&self) -> &UsbCpkt {
        self.usb_cpkt(2)
    }
    #[doc = "0x70 - USB3 Endpoint upper address for DMA operations."]
    #[inline(always)]
    pub const fn ep_dma_ext_addr(&self) -> &EpDmaExtAddr {
        &self.ep_dma_ext_addr
    }
    #[doc = "0x74..0x80 - USB3 On-chip buffer registers."]
    #[inline(always)]
    pub const fn buf(&self) -> &Buf {
        &self.buf
    }
    #[doc = "0x80 - USB3 DMA transfer mode."]
    #[inline(always)]
    pub const fn dtrans(&self) -> &Dtrans {
        &self.dtrans
    }
    #[doc = "0x84..0x98 - USB3 Device TDL registers."]
    #[inline(always)]
    pub const fn tdl(&self) -> &Tdl {
        &self.tdl
    }
    #[doc = "0x100 - Device configuration 1."]
    #[inline(always)]
    pub const fn cfg1(&self) -> &Cfg1 {
        &self.cfg1
    }
    #[doc = "0x104 - Device debug link 1."]
    #[inline(always)]
    pub const fn dbg_link1(&self) -> &DbgLink1 {
        &self.dbg_link1
    }
    #[doc = "0x108 - Device debug link 2."]
    #[inline(always)]
    pub const fn dbg_link2(&self) -> &DbgLink2 {
        &self.dbg_link2
    }
    #[doc = "0x10c..0x234 - Device configuration."]
    #[inline(always)]
    pub const fn cfg(&self, n: usize) -> &Cfg {
        &self.cfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10c..0x234 - Device configuration."]
    #[inline(always)]
    pub fn cfg_iter(&self) -> impl Iterator<Item = &Cfg> {
        self.cfg.iter()
    }
    #[doc = "0x300..0x314 - Device DMA registers."]
    #[inline(always)]
    pub const fn dma_axi(&self) -> &DmaAxi {
        &self.dma_axi
    }
}
#[doc = "usb_conf (rw) register accessor: USB3 Global configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_conf`]
module"]
#[doc(alias = "usb_conf")]
pub type UsbConf = crate::Reg<usb_conf::UsbConfSpec>;
#[doc = "USB3 Global configuration."]
pub mod usb_conf;
#[doc = "usb_sts (rw) register accessor: USB3 Global status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_sts`]
module"]
#[doc(alias = "usb_sts")]
pub type UsbSts = crate::Reg<usb_sts::UsbStsSpec>;
#[doc = "USB3 Global status."]
pub mod usb_sts;
#[doc = "usb_cmd (rw) register accessor: USB3 Global command.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_cmd`]
module"]
#[doc(alias = "usb_cmd")]
pub type UsbCmd = crate::Reg<usb_cmd::UsbCmdSpec>;
#[doc = "USB3 Global command."]
pub mod usb_cmd;
#[doc = "usb_itpn (rw) register accessor: ITP (SS) / SOF (FS/HS) number - SS: last ITP number, FS/HS: last SOF number.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_itpn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_itpn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_itpn`]
module"]
#[doc(alias = "usb_itpn")]
pub type UsbItpn = crate::Reg<usb_itpn::UsbItpnSpec>;
#[doc = "ITP (SS) / SOF (FS/HS) number - SS: last ITP number, FS/HS: last SOF number."]
pub mod usb_itpn;
#[doc = "usb_lpm (rw) register accessor: Global LPM.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_lpm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_lpm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_lpm`]
module"]
#[doc(alias = "usb_lpm")]
pub type UsbLpm = crate::Reg<usb_lpm::UsbLpmSpec>;
#[doc = "Global LPM."]
pub mod usb_lpm;
#[doc = "USB Interrupt registers."]
pub use self::usb_int::UsbInt;
#[doc = r"Cluster"]
#[doc = "USB Interrupt registers."]
pub mod usb_int;
#[doc = "ep_sel (rw) register accessor: USB3 Endpoint select.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_sel`]
module"]
#[doc(alias = "ep_sel")]
pub type EpSel = crate::Reg<ep_sel::EpSelSpec>;
#[doc = "USB3 Endpoint select."]
pub mod ep_sel;
#[doc = "ep_traddr (rw) register accessor: USB3 Endpoint transfer address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_traddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_traddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_traddr`]
module"]
#[doc(alias = "ep_traddr")]
pub type EpTraddr = crate::Reg<ep_traddr::EpTraddrSpec>;
#[doc = "USB3 Endpoint transfer address."]
pub mod ep_traddr;
#[doc = "ep_cfg (rw) register accessor: USB3 Endpoint configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_cfg`]
module"]
#[doc(alias = "ep_cfg")]
pub type EpCfg = crate::Reg<ep_cfg::EpCfgSpec>;
#[doc = "USB3 Endpoint configuration."]
pub mod ep_cfg;
#[doc = "ep_cmd (rw) register accessor: USB3 Endpoint command.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_cmd`]
module"]
#[doc(alias = "ep_cmd")]
pub type EpCmd = crate::Reg<ep_cmd::EpCmdSpec>;
#[doc = "USB3 Endpoint command."]
pub mod ep_cmd;
#[doc = "USB3 Endpoint status registers."]
pub use self::ep_sts::EpSts;
#[doc = r"Cluster"]
#[doc = "USB3 Endpoint status registers."]
pub mod ep_sts;
#[doc = "drbl (rw) register accessor: USB3 doorbell.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`drbl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`drbl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drbl`]
module"]
#[doc(alias = "drbl")]
pub type Drbl = crate::Reg<drbl::DrblSpec>;
#[doc = "USB3 doorbell."]
pub mod drbl;
#[doc = "ep_int (rw) register accessor: USB3 Endpoint interrupt registers - ep_int0: enable, ep_int1: status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_int`]
module"]
#[doc(alias = "ep_int")]
pub type EpInt = crate::Reg<ep_int::EpIntSpec>;
#[doc = "USB3 Endpoint interrupt registers - ep_int0: enable, ep_int1: status."]
pub mod ep_int;
#[doc = "usb_pwr (rw) register accessor: USB3 Global power.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_pwr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_pwr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_pwr`]
module"]
#[doc(alias = "usb_pwr")]
pub type UsbPwr = crate::Reg<usb_pwr::UsbPwrSpec>;
#[doc = "USB3 Global power."]
pub mod usb_pwr;
#[doc = "usb_conf2 (rw) register accessor: USB3 Global configurartion 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_conf2`]
module"]
#[doc(alias = "usb_conf2")]
pub type UsbConf2 = crate::Reg<usb_conf2::UsbConf2Spec>;
#[doc = "USB3 Global configurartion 2."]
pub mod usb_conf2;
#[doc = "USB3 Global Capability registers."]
pub use self::usb_cap::UsbCap;
#[doc = r"Cluster"]
#[doc = "USB3 Global Capability registers."]
pub mod usb_cap;
#[doc = "usb_cpkt (rw) register accessor: USB3 Global custom packet.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_cpkt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_cpkt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_cpkt`]
module"]
#[doc(alias = "usb_cpkt")]
pub type UsbCpkt = crate::Reg<usb_cpkt::UsbCpktSpec>;
#[doc = "USB3 Global custom packet."]
pub mod usb_cpkt;
#[doc = "ep_dma_ext_addr (rw) register accessor: USB3 Endpoint upper address for DMA operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_dma_ext_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_dma_ext_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_dma_ext_addr`]
module"]
#[doc(alias = "ep_dma_ext_addr")]
pub type EpDmaExtAddr = crate::Reg<ep_dma_ext_addr::EpDmaExtAddrSpec>;
#[doc = "USB3 Endpoint upper address for DMA operations."]
pub mod ep_dma_ext_addr;
#[doc = "USB3 On-chip buffer registers."]
pub use self::buf::Buf;
#[doc = r"Cluster"]
#[doc = "USB3 On-chip buffer registers."]
pub mod buf;
#[doc = "dtrans (rw) register accessor: USB3 DMA transfer mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtrans::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtrans::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtrans`]
module"]
#[doc(alias = "dtrans")]
pub type Dtrans = crate::Reg<dtrans::DtransSpec>;
#[doc = "USB3 DMA transfer mode."]
pub mod dtrans;
#[doc = "USB3 Device TDL registers."]
pub use self::tdl::Tdl;
#[doc = r"Cluster"]
#[doc = "USB3 Device TDL registers."]
pub mod tdl;
#[doc = "cfg1 (rw) register accessor: Device configuration 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg1`]
module"]
#[doc(alias = "cfg1")]
pub type Cfg1 = crate::Reg<cfg1::Cfg1Spec>;
#[doc = "Device configuration 1."]
pub mod cfg1;
#[doc = "dbg_link1 (rw) register accessor: Device debug link 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbg_link1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbg_link1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_link1`]
module"]
#[doc(alias = "dbg_link1")]
pub type DbgLink1 = crate::Reg<dbg_link1::DbgLink1Spec>;
#[doc = "Device debug link 1."]
pub mod dbg_link1;
#[doc = "dbg_link2 (rw) register accessor: Device debug link 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbg_link2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbg_link2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_link2`]
module"]
#[doc(alias = "dbg_link2")]
pub type DbgLink2 = crate::Reg<dbg_link2::DbgLink2Spec>;
#[doc = "Device debug link 2."]
pub mod dbg_link2;
#[doc = "cfg (rw) register accessor: Device configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
#[doc(alias = "cfg")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "Device configuration."]
pub mod cfg;
#[doc = "Device DMA registers."]
pub use self::dma_axi::DmaAxi;
#[doc = r"Cluster"]
#[doc = "Device DMA registers."]
pub mod dma_axi;
