#[repr(C)]
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG: GPIO pad configuration"]
#[doc(alias = "padcfg")]
pub struct Padcfg {
    gpio0: Gpio0,
    gpio1: Gpio1,
    gpio2: Gpio2,
    gpio3: Gpio3,
    gpio4: Gpio4,
    gpio5: Gpio5,
    gpio6: Gpio6,
    gpio7: Gpio7,
    gpio8: Gpio8,
    gpio9: Gpio9,
    gpio10: Gpio10,
    gpio11: Gpio11,
    gpio12: Gpio12,
    gpio13: Gpio13,
    gpio14: Gpio14,
    gpio15: Gpio15,
    gpio16: Gpio16,
    gpio17: Gpio17,
    gpio18: Gpio18,
    gpio19: Gpio19,
    gpio20: Gpio20,
    gpio21: Gpio21,
    gpio22: Gpio22,
    gpio23: Gpio23,
    gpio24: Gpio24,
    gpio25: Gpio25,
    gpio26: Gpio26,
    gpio27: Gpio27,
    gpio28: Gpio28,
    gpio29: Gpio29,
    gpio30: Gpio30,
    gpio31: Gpio31,
    gpio32: Gpio32,
    gpio33: Gpio33,
    gpio34: Gpio34,
    gpio35: Gpio35,
    gpio36: Gpio36,
    gpio37: Gpio37,
    gpio38: Gpio38,
    gpio39: Gpio39,
    gpio40: Gpio40,
    gpio41: Gpio41,
    gpio42: Gpio42,
    gpio43: Gpio43,
    gpio44: Gpio44,
    gpio45: Gpio45,
    gpio46: Gpio46,
    gpio47: Gpio47,
    gpio48: Gpio48,
    gpio49: Gpio49,
    gpio50: Gpio50,
    gpio51: Gpio51,
    gpio52: Gpio52,
    gpio53: Gpio53,
    gpio54: Gpio54,
    gpio55: Gpio55,
    gpio56: Gpio56,
    gpio57: Gpio57,
    gpio58: Gpio58,
    gpio59: Gpio59,
    gpio60: Gpio60,
    gpio61: Gpio61,
    gpio62: Gpio62,
    gpio63: Gpio63,
    sd0_clk: Sd0Clk,
    sd0_cmd: Sd0Cmd,
    sd0_data0: Sd0Data0,
    sd0_data1: Sd0Data1,
    sd0_data2: Sd0Data2,
    sd0_data3: Sd0Data3,
    sd0_data4: Sd0Data4,
    sd0_data5: Sd0Data5,
    sd0_data6: Sd0Data6,
    sd0_data7: Sd0Data7,
    sd0_strb: Sd0Strb,
    gmac1: [Gmac1; 14],
    qspi_sclk: QspiSclk,
    qspi_csn0: QspiCsn0,
    qspi_data: [QspiData; 4],
}
impl Padcfg {
    #[doc = "0x00 - SYS IOMUX CFG SAIF SYSCFG PADCFG 288: GPIO0"]
    #[inline(always)]
    pub const fn gpio0(&self) -> &Gpio0 {
        &self.gpio0
    }
    #[doc = "0x04 - SYS IOMUX CFG SAIF SYSCFG PADCFG 292: GPIO1"]
    #[inline(always)]
    pub const fn gpio1(&self) -> &Gpio1 {
        &self.gpio1
    }
    #[doc = "0x08 - SYS IOMUX CFG SAIF SYSCFG PADCFG 296: GPIO2"]
    #[inline(always)]
    pub const fn gpio2(&self) -> &Gpio2 {
        &self.gpio2
    }
    #[doc = "0x0c - SYS IOMUX CFG SAIF SYSCFG PADCFG 300: GPIO3"]
    #[inline(always)]
    pub const fn gpio3(&self) -> &Gpio3 {
        &self.gpio3
    }
    #[doc = "0x10 - SYS IOMUX CFG SAIF SYSCFG PADCFG 304: GPIO4"]
    #[inline(always)]
    pub const fn gpio4(&self) -> &Gpio4 {
        &self.gpio4
    }
    #[doc = "0x14 - SYS IOMUX CFG SAIF SYSCFG PADCFG 308: GPIO5"]
    #[inline(always)]
    pub const fn gpio5(&self) -> &Gpio5 {
        &self.gpio5
    }
    #[doc = "0x18 - SYS IOMUX CFG SAIF SYSCFG PADCFG 312: GPIO6"]
    #[inline(always)]
    pub const fn gpio6(&self) -> &Gpio6 {
        &self.gpio6
    }
    #[doc = "0x1c - SYS IOMUX CFG SAIF SYSCFG PADCFG 316: GPIO7"]
    #[inline(always)]
    pub const fn gpio7(&self) -> &Gpio7 {
        &self.gpio7
    }
    #[doc = "0x20 - SYS IOMUX CFG SAIF SYSCFG PADCFG 320: GPIO8"]
    #[inline(always)]
    pub const fn gpio8(&self) -> &Gpio8 {
        &self.gpio8
    }
    #[doc = "0x24 - SYS IOMUX CFG SAIF SYSCFG PADCFG 324: GPIO9"]
    #[inline(always)]
    pub const fn gpio9(&self) -> &Gpio9 {
        &self.gpio9
    }
    #[doc = "0x28 - SYS IOMUX CFG SAIF SYSCFG PADCFG 328: GPIO10"]
    #[inline(always)]
    pub const fn gpio10(&self) -> &Gpio10 {
        &self.gpio10
    }
    #[doc = "0x2c - SYS IOMUX CFG SAIF SYSCFG PADCFG 332: GPIO11"]
    #[inline(always)]
    pub const fn gpio11(&self) -> &Gpio11 {
        &self.gpio11
    }
    #[doc = "0x30 - SYS IOMUX CFG SAIF SYSCFG PADCFG 336: GPIO12"]
    #[inline(always)]
    pub const fn gpio12(&self) -> &Gpio12 {
        &self.gpio12
    }
    #[doc = "0x34 - SYS IOMUX CFG SAIF SYSCFG PADCFG 340: GPIO13"]
    #[inline(always)]
    pub const fn gpio13(&self) -> &Gpio13 {
        &self.gpio13
    }
    #[doc = "0x38 - SYS IOMUX CFG SAIF SYSCFG PADCFG 344: GPIO14"]
    #[inline(always)]
    pub const fn gpio14(&self) -> &Gpio14 {
        &self.gpio14
    }
    #[doc = "0x3c - SYS IOMUX CFG SAIF SYSCFG PADCFG 348: GPIO15"]
    #[inline(always)]
    pub const fn gpio15(&self) -> &Gpio15 {
        &self.gpio15
    }
    #[doc = "0x40 - SYS IOMUX CFG SAIF SYSCFG PADCFG 352: GPIO16"]
    #[inline(always)]
    pub const fn gpio16(&self) -> &Gpio16 {
        &self.gpio16
    }
    #[doc = "0x44 - SYS IOMUX CFG SAIF SYSCFG PADCFG 356: GPIO17"]
    #[inline(always)]
    pub const fn gpio17(&self) -> &Gpio17 {
        &self.gpio17
    }
    #[doc = "0x48 - SYS IOMUX CFG SAIF SYSCFG PADCFG 360: GPIO18"]
    #[inline(always)]
    pub const fn gpio18(&self) -> &Gpio18 {
        &self.gpio18
    }
    #[doc = "0x4c - SYS IOMUX CFG SAIF SYSCFG PADCFG 364: GPIO19"]
    #[inline(always)]
    pub const fn gpio19(&self) -> &Gpio19 {
        &self.gpio19
    }
    #[doc = "0x50 - SYS IOMUX CFG SAIF SYSCFG PADCFG 368: GPIO20"]
    #[inline(always)]
    pub const fn gpio20(&self) -> &Gpio20 {
        &self.gpio20
    }
    #[doc = "0x54 - SYS IOMUX CFG SAIF SYSCFG PADCFG 372: GPIO21"]
    #[inline(always)]
    pub const fn gpio21(&self) -> &Gpio21 {
        &self.gpio21
    }
    #[doc = "0x58 - SYS IOMUX CFG SAIF SYSCFG PADCFG 376: GPIO22"]
    #[inline(always)]
    pub const fn gpio22(&self) -> &Gpio22 {
        &self.gpio22
    }
    #[doc = "0x5c - SYS IOMUX CFG SAIF SYSCFG PADCFG 380: GPIO23"]
    #[inline(always)]
    pub const fn gpio23(&self) -> &Gpio23 {
        &self.gpio23
    }
    #[doc = "0x60 - SYS IOMUX CFG SAIF SYSCFG PADCFG 384: GPIO24"]
    #[inline(always)]
    pub const fn gpio24(&self) -> &Gpio24 {
        &self.gpio24
    }
    #[doc = "0x64 - SYS IOMUX CFG SAIF SYSCFG PADCFG 388: GPIO25"]
    #[inline(always)]
    pub const fn gpio25(&self) -> &Gpio25 {
        &self.gpio25
    }
    #[doc = "0x68 - SYS IOMUX CFG SAIF SYSCFG PADCFG 392: GPIO26"]
    #[inline(always)]
    pub const fn gpio26(&self) -> &Gpio26 {
        &self.gpio26
    }
    #[doc = "0x6c - SYS IOMUX CFG SAIF SYSCFG PADCFG 396: GPIO27"]
    #[inline(always)]
    pub const fn gpio27(&self) -> &Gpio27 {
        &self.gpio27
    }
    #[doc = "0x70 - SYS IOMUX CFG SAIF SYSCFG PADCFG 400: GPIO28"]
    #[inline(always)]
    pub const fn gpio28(&self) -> &Gpio28 {
        &self.gpio28
    }
    #[doc = "0x74 - SYS IOMUX CFG SAIF SYSCFG PADCFG 404: GPIO29"]
    #[inline(always)]
    pub const fn gpio29(&self) -> &Gpio29 {
        &self.gpio29
    }
    #[doc = "0x78 - SYS IOMUX CFG SAIF SYSCFG PADCFG 408: GPIO30"]
    #[inline(always)]
    pub const fn gpio30(&self) -> &Gpio30 {
        &self.gpio30
    }
    #[doc = "0x7c - SYS IOMUX CFG SAIF SYSCFG PADCFG 412: GPIO31"]
    #[inline(always)]
    pub const fn gpio31(&self) -> &Gpio31 {
        &self.gpio31
    }
    #[doc = "0x80 - SYS IOMUX CFG SAIF SYSCFG PADCFG 416: GPIO32"]
    #[inline(always)]
    pub const fn gpio32(&self) -> &Gpio32 {
        &self.gpio32
    }
    #[doc = "0x84 - SYS IOMUX CFG SAIF SYSCFG PADCFG 420: GPIO33"]
    #[inline(always)]
    pub const fn gpio33(&self) -> &Gpio33 {
        &self.gpio33
    }
    #[doc = "0x88 - SYS IOMUX CFG SAIF SYSCFG PADCFG 424: GPIO34"]
    #[inline(always)]
    pub const fn gpio34(&self) -> &Gpio34 {
        &self.gpio34
    }
    #[doc = "0x8c - SYS IOMUX CFG SAIF SYSCFG PADCFG 428: GPIO35"]
    #[inline(always)]
    pub const fn gpio35(&self) -> &Gpio35 {
        &self.gpio35
    }
    #[doc = "0x90 - SYS IOMUX CFG SAIF SYSCFG PADCFG 432: GPIO36"]
    #[inline(always)]
    pub const fn gpio36(&self) -> &Gpio36 {
        &self.gpio36
    }
    #[doc = "0x94 - SYS IOMUX CFG SAIF SYSCFG PADCFG 436: GPIO37"]
    #[inline(always)]
    pub const fn gpio37(&self) -> &Gpio37 {
        &self.gpio37
    }
    #[doc = "0x98 - SYS IOMUX CFG SAIF SYSCFG PADCFG 440: GPIO38"]
    #[inline(always)]
    pub const fn gpio38(&self) -> &Gpio38 {
        &self.gpio38
    }
    #[doc = "0x9c - SYS IOMUX CFG SAIF SYSCFG PADCFG 444: GPIO39"]
    #[inline(always)]
    pub const fn gpio39(&self) -> &Gpio39 {
        &self.gpio39
    }
    #[doc = "0xa0 - SYS IOMUX CFG SAIF SYSCFG PADCFG 448: GPIO40"]
    #[inline(always)]
    pub const fn gpio40(&self) -> &Gpio40 {
        &self.gpio40
    }
    #[doc = "0xa4 - SYS IOMUX CFG SAIF SYSCFG PADCFG 452: GPIO41"]
    #[inline(always)]
    pub const fn gpio41(&self) -> &Gpio41 {
        &self.gpio41
    }
    #[doc = "0xa8 - SYS IOMUX CFG SAIF SYSCFG PADCFG 456: GPIO42"]
    #[inline(always)]
    pub const fn gpio42(&self) -> &Gpio42 {
        &self.gpio42
    }
    #[doc = "0xac - SYS IOMUX CFG SAIF SYSCFG PADCFG 460: GPIO43"]
    #[inline(always)]
    pub const fn gpio43(&self) -> &Gpio43 {
        &self.gpio43
    }
    #[doc = "0xb0 - SYS IOMUX CFG SAIF SYSCFG PADCFG 464: GPIO44"]
    #[inline(always)]
    pub const fn gpio44(&self) -> &Gpio44 {
        &self.gpio44
    }
    #[doc = "0xb4 - SYS IOMUX CFG SAIF SYSCFG PADCFG 468: GPIO45"]
    #[inline(always)]
    pub const fn gpio45(&self) -> &Gpio45 {
        &self.gpio45
    }
    #[doc = "0xb8 - SYS IOMUX CFG SAIF SYSCFG PADCFG 472: GPIO46"]
    #[inline(always)]
    pub const fn gpio46(&self) -> &Gpio46 {
        &self.gpio46
    }
    #[doc = "0xbc - SYS IOMUX CFG SAIF SYSCFG PADCFG 476: GPIO47"]
    #[inline(always)]
    pub const fn gpio47(&self) -> &Gpio47 {
        &self.gpio47
    }
    #[doc = "0xc0 - SYS IOMUX CFG SAIF SYSCFG PADCFG 480: GPIO48"]
    #[inline(always)]
    pub const fn gpio48(&self) -> &Gpio48 {
        &self.gpio48
    }
    #[doc = "0xc4 - SYS IOMUX CFG SAIF SYSCFG PADCFG 484: GPIO49"]
    #[inline(always)]
    pub const fn gpio49(&self) -> &Gpio49 {
        &self.gpio49
    }
    #[doc = "0xc8 - SYS IOMUX CFG SAIF SYSCFG PADCFG 488: GPIO50"]
    #[inline(always)]
    pub const fn gpio50(&self) -> &Gpio50 {
        &self.gpio50
    }
    #[doc = "0xcc - SYS IOMUX CFG SAIF SYSCFG PADCFG 492: GPIO51"]
    #[inline(always)]
    pub const fn gpio51(&self) -> &Gpio51 {
        &self.gpio51
    }
    #[doc = "0xd0 - SYS IOMUX CFG SAIF SYSCFG PADCFG 496: GPIO52"]
    #[inline(always)]
    pub const fn gpio52(&self) -> &Gpio52 {
        &self.gpio52
    }
    #[doc = "0xd4 - SYS IOMUX CFG SAIF SYSCFG PADCFG 500: GPIO53"]
    #[inline(always)]
    pub const fn gpio53(&self) -> &Gpio53 {
        &self.gpio53
    }
    #[doc = "0xd8 - SYS IOMUX CFG SAIF SYSCFG PADCFG 504: GPIO54"]
    #[inline(always)]
    pub const fn gpio54(&self) -> &Gpio54 {
        &self.gpio54
    }
    #[doc = "0xdc - SYS IOMUX CFG SAIF SYSCFG PADCFG 508: GPIO55"]
    #[inline(always)]
    pub const fn gpio55(&self) -> &Gpio55 {
        &self.gpio55
    }
    #[doc = "0xe0 - SYS IOMUX CFG SAIF SYSCFG PADCFG 512: GPIO56"]
    #[inline(always)]
    pub const fn gpio56(&self) -> &Gpio56 {
        &self.gpio56
    }
    #[doc = "0xe4 - SYS IOMUX CFG SAIF SYSCFG PADCFG 516: GPIO57"]
    #[inline(always)]
    pub const fn gpio57(&self) -> &Gpio57 {
        &self.gpio57
    }
    #[doc = "0xe8 - SYS IOMUX CFG SAIF SYSCFG PADCFG 520: GPIO58"]
    #[inline(always)]
    pub const fn gpio58(&self) -> &Gpio58 {
        &self.gpio58
    }
    #[doc = "0xec - SYS IOMUX CFG SAIF SYSCFG PADCFG 524: GPIO59"]
    #[inline(always)]
    pub const fn gpio59(&self) -> &Gpio59 {
        &self.gpio59
    }
    #[doc = "0xf0 - SYS IOMUX CFG SAIF SYSCFG PADCFG 528: GPIO60"]
    #[inline(always)]
    pub const fn gpio60(&self) -> &Gpio60 {
        &self.gpio60
    }
    #[doc = "0xf4 - SYS IOMUX CFG SAIF SYSCFG PADCFG 532: GPIO61"]
    #[inline(always)]
    pub const fn gpio61(&self) -> &Gpio61 {
        &self.gpio61
    }
    #[doc = "0xf8 - SYS IOMUX CFG SAIF SYSCFG PADCFG 536: GPIO62"]
    #[inline(always)]
    pub const fn gpio62(&self) -> &Gpio62 {
        &self.gpio62
    }
    #[doc = "0xfc - SYS IOMUX CFG SAIF SYSCFG PADCFG 540: GPIO63"]
    #[inline(always)]
    pub const fn gpio63(&self) -> &Gpio63 {
        &self.gpio63
    }
    #[doc = "0x100 - SYS IOMUX CFG SAIF SYSCFG PADCFG 544: SD0_CLK"]
    #[inline(always)]
    pub const fn sd0_clk(&self) -> &Sd0Clk {
        &self.sd0_clk
    }
    #[doc = "0x104 - SYS IOMUX CFG SAIF SYSCFG PADCFG 548: SD0_CMD"]
    #[inline(always)]
    pub const fn sd0_cmd(&self) -> &Sd0Cmd {
        &self.sd0_cmd
    }
    #[doc = "0x108 - SYS IOMUX CFG SAIF SYSCFG PADCFG 552: SD0_DATA0"]
    #[inline(always)]
    pub const fn sd0_data0(&self) -> &Sd0Data0 {
        &self.sd0_data0
    }
    #[doc = "0x10c - SYS IOMUX CFG SAIF SYSCFG PADCFG 556: SD0_DATA1"]
    #[inline(always)]
    pub const fn sd0_data1(&self) -> &Sd0Data1 {
        &self.sd0_data1
    }
    #[doc = "0x110 - SYS IOMUX CFG SAIF SYSCFG PADCFG 560: SD0_DATA2"]
    #[inline(always)]
    pub const fn sd0_data2(&self) -> &Sd0Data2 {
        &self.sd0_data2
    }
    #[doc = "0x114 - SYS IOMUX CFG SAIF SYSCFG PADCFG 564: SD0_DATA3"]
    #[inline(always)]
    pub const fn sd0_data3(&self) -> &Sd0Data3 {
        &self.sd0_data3
    }
    #[doc = "0x118 - SYS IOMUX CFG SAIF SYSCFG PADCFG 568: SD0_DATA4"]
    #[inline(always)]
    pub const fn sd0_data4(&self) -> &Sd0Data4 {
        &self.sd0_data4
    }
    #[doc = "0x11c - SYS IOMUX CFG SAIF SYSCFG PADCFG 572: SD0_DATA5"]
    #[inline(always)]
    pub const fn sd0_data5(&self) -> &Sd0Data5 {
        &self.sd0_data5
    }
    #[doc = "0x120 - SYS IOMUX CFG SAIF SYSCFG PADCFG 576: SD0_DATA6"]
    #[inline(always)]
    pub const fn sd0_data6(&self) -> &Sd0Data6 {
        &self.sd0_data6
    }
    #[doc = "0x124 - SYS IOMUX CFG SAIF SYSCFG PADCFG 580: SD0_DATA7"]
    #[inline(always)]
    pub const fn sd0_data7(&self) -> &Sd0Data7 {
        &self.sd0_data7
    }
    #[doc = "0x128 - SYS IOMUX CFG SAIF SYSCFG PADCFG 584: SD0_STRB"]
    #[inline(always)]
    pub const fn sd0_strb(&self) -> &Sd0Strb {
        &self.sd0_strb
    }
    #[doc = "0x12c..0x164 - SYS IOMUX CFG SAIF SYSCFG PADCFG: GPIO GMAC1 pads"]
    #[inline(always)]
    pub const fn gmac1(&self, n: usize) -> &Gmac1 {
        &self.gmac1[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x12c..0x164 - SYS IOMUX CFG SAIF SYSCFG PADCFG: GPIO GMAC1 pads"]
    #[inline(always)]
    pub fn gmac1_iter(&self) -> impl Iterator<Item = &Gmac1> {
        self.gmac1.iter()
    }
    #[doc = "0x12c - SYS IOMUX CFG SAIF SYSCFG PADCFG: GPIO GMAC1 pads"]
    #[inline(always)]
    pub const fn gmac1_mdc(&self) -> &Gmac1 {
        self.gmac1(0)
    }
    #[doc = "0x130 - SYS IOMUX CFG SAIF SYSCFG PADCFG: GPIO GMAC1 pads"]
    #[inline(always)]
    pub const fn gmac1_mdio(&self) -> &Gmac1 {
        self.gmac1(1)
    }
    #[doc = "0x134 - SYS IOMUX CFG SAIF SYSCFG PADCFG: GPIO GMAC1 pads"]
    #[inline(always)]
    pub const fn gmac1_rxd_0(&self) -> &Gmac1 {
        self.gmac1(2)
    }
    #[doc = "0x138 - SYS IOMUX CFG SAIF SYSCFG PADCFG: GPIO GMAC1 pads"]
    #[inline(always)]
    pub const fn gmac1_rxd_1(&self) -> &Gmac1 {
        self.gmac1(3)
    }
    #[doc = "0x13c - SYS IOMUX CFG SAIF SYSCFG PADCFG: GPIO GMAC1 pads"]
    #[inline(always)]
    pub const fn gmac1_rxd_2(&self) -> &Gmac1 {
        self.gmac1(4)
    }
    #[doc = "0x140 - SYS IOMUX CFG SAIF SYSCFG PADCFG: GPIO GMAC1 pads"]
    #[inline(always)]
    pub const fn gmac1_rxd_3(&self) -> &Gmac1 {
        self.gmac1(5)
    }
    #[doc = "0x144 - SYS IOMUX CFG SAIF SYSCFG PADCFG: GPIO GMAC1 pads"]
    #[inline(always)]
    pub const fn gmac1_rxdv(&self) -> &Gmac1 {
        self.gmac1(6)
    }
    #[doc = "0x148 - SYS IOMUX CFG SAIF SYSCFG PADCFG: GPIO GMAC1 pads"]
    #[inline(always)]
    pub const fn gmac1_rxc(&self) -> &Gmac1 {
        self.gmac1(7)
    }
    #[doc = "0x14c - SYS IOMUX CFG SAIF SYSCFG PADCFG: GPIO GMAC1 pads"]
    #[inline(always)]
    pub const fn gmac1_txd_0(&self) -> &Gmac1 {
        self.gmac1(8)
    }
    #[doc = "0x150 - SYS IOMUX CFG SAIF SYSCFG PADCFG: GPIO GMAC1 pads"]
    #[inline(always)]
    pub const fn gmac1_txd_1(&self) -> &Gmac1 {
        self.gmac1(9)
    }
    #[doc = "0x154 - SYS IOMUX CFG SAIF SYSCFG PADCFG: GPIO GMAC1 pads"]
    #[inline(always)]
    pub const fn gmac1_txd_2(&self) -> &Gmac1 {
        self.gmac1(10)
    }
    #[doc = "0x158 - SYS IOMUX CFG SAIF SYSCFG PADCFG: GPIO GMAC1 pads"]
    #[inline(always)]
    pub const fn gmac1_txd_3(&self) -> &Gmac1 {
        self.gmac1(11)
    }
    #[doc = "0x15c - SYS IOMUX CFG SAIF SYSCFG PADCFG: GPIO GMAC1 pads"]
    #[inline(always)]
    pub const fn gmac1_txen(&self) -> &Gmac1 {
        self.gmac1(12)
    }
    #[doc = "0x160 - SYS IOMUX CFG SAIF SYSCFG PADCFG: GPIO GMAC1 pads"]
    #[inline(always)]
    pub const fn gmac1_txc(&self) -> &Gmac1 {
        self.gmac1(13)
    }
    #[doc = "0x164 - SYS IOMUX CFG SAIF SYSCFG PADCFG 644: QSPI_SCLK"]
    #[inline(always)]
    pub const fn qspi_sclk(&self) -> &QspiSclk {
        &self.qspi_sclk
    }
    #[doc = "0x168 - SYS IOMUX CFG SAIF SYSCFG PADCFG 648: QSPI_CSN0"]
    #[inline(always)]
    pub const fn qspi_csn0(&self) -> &QspiCsn0 {
        &self.qspi_csn0
    }
    #[doc = "0x16c..0x17c - SYS IOMUX CFG SAIF SYSCFG PADCFG 28c-298: QSPI_DATA 0-3"]
    #[inline(always)]
    pub const fn qspi_data(&self, n: usize) -> &QspiData {
        &self.qspi_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x16c..0x17c - SYS IOMUX CFG SAIF SYSCFG PADCFG 28c-298: QSPI_DATA 0-3"]
    #[inline(always)]
    pub fn qspi_data_iter(&self) -> impl Iterator<Item = &QspiData> {
        self.qspi_data.iter()
    }
}
#[doc = "gpio0 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 288: GPIO0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0`]
module"]
#[doc(alias = "gpio0")]
pub type Gpio0 = crate::Reg<gpio0::Gpio0Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 288: GPIO0"]
pub mod gpio0;
#[doc = "gpio1 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 292: GPIO1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1`]
module"]
#[doc(alias = "gpio1")]
pub type Gpio1 = crate::Reg<gpio1::Gpio1Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 292: GPIO1"]
pub mod gpio1;
#[doc = "gpio2 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 296: GPIO2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2`]
module"]
#[doc(alias = "gpio2")]
pub type Gpio2 = crate::Reg<gpio2::Gpio2Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 296: GPIO2"]
pub mod gpio2;
#[doc = "gpio3 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 300: GPIO3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3`]
module"]
#[doc(alias = "gpio3")]
pub type Gpio3 = crate::Reg<gpio3::Gpio3Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 300: GPIO3"]
pub mod gpio3;
#[doc = "gpio4 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 304: GPIO4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio4`]
module"]
#[doc(alias = "gpio4")]
pub type Gpio4 = crate::Reg<gpio4::Gpio4Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 304: GPIO4"]
pub mod gpio4;
#[doc = "gpio5 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 308: GPIO5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio5`]
module"]
#[doc(alias = "gpio5")]
pub type Gpio5 = crate::Reg<gpio5::Gpio5Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 308: GPIO5"]
pub mod gpio5;
#[doc = "gpio6 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 312: GPIO6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio6`]
module"]
#[doc(alias = "gpio6")]
pub type Gpio6 = crate::Reg<gpio6::Gpio6Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 312: GPIO6"]
pub mod gpio6;
#[doc = "gpio7 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 316: GPIO7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio7`]
module"]
#[doc(alias = "gpio7")]
pub type Gpio7 = crate::Reg<gpio7::Gpio7Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 316: GPIO7"]
pub mod gpio7;
#[doc = "gpio8 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 320: GPIO8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio8`]
module"]
#[doc(alias = "gpio8")]
pub type Gpio8 = crate::Reg<gpio8::Gpio8Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 320: GPIO8"]
pub mod gpio8;
#[doc = "gpio9 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 324: GPIO9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio9`]
module"]
#[doc(alias = "gpio9")]
pub type Gpio9 = crate::Reg<gpio9::Gpio9Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 324: GPIO9"]
pub mod gpio9;
#[doc = "gpio10 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 328: GPIO10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio10`]
module"]
#[doc(alias = "gpio10")]
pub type Gpio10 = crate::Reg<gpio10::Gpio10Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 328: GPIO10"]
pub mod gpio10;
#[doc = "gpio11 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 332: GPIO11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio11`]
module"]
#[doc(alias = "gpio11")]
pub type Gpio11 = crate::Reg<gpio11::Gpio11Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 332: GPIO11"]
pub mod gpio11;
#[doc = "gpio12 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 336: GPIO12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio12`]
module"]
#[doc(alias = "gpio12")]
pub type Gpio12 = crate::Reg<gpio12::Gpio12Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 336: GPIO12"]
pub mod gpio12;
#[doc = "gpio13 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 340: GPIO13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio13`]
module"]
#[doc(alias = "gpio13")]
pub type Gpio13 = crate::Reg<gpio13::Gpio13Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 340: GPIO13"]
pub mod gpio13;
#[doc = "gpio14 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 344: GPIO14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio14`]
module"]
#[doc(alias = "gpio14")]
pub type Gpio14 = crate::Reg<gpio14::Gpio14Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 344: GPIO14"]
pub mod gpio14;
#[doc = "gpio15 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 348: GPIO15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio15`]
module"]
#[doc(alias = "gpio15")]
pub type Gpio15 = crate::Reg<gpio15::Gpio15Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 348: GPIO15"]
pub mod gpio15;
#[doc = "gpio16 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 352: GPIO16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio16`]
module"]
#[doc(alias = "gpio16")]
pub type Gpio16 = crate::Reg<gpio16::Gpio16Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 352: GPIO16"]
pub mod gpio16;
#[doc = "gpio17 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 356: GPIO17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio17`]
module"]
#[doc(alias = "gpio17")]
pub type Gpio17 = crate::Reg<gpio17::Gpio17Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 356: GPIO17"]
pub mod gpio17;
#[doc = "gpio18 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 360: GPIO18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio18`]
module"]
#[doc(alias = "gpio18")]
pub type Gpio18 = crate::Reg<gpio18::Gpio18Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 360: GPIO18"]
pub mod gpio18;
#[doc = "gpio19 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 364: GPIO19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio19`]
module"]
#[doc(alias = "gpio19")]
pub type Gpio19 = crate::Reg<gpio19::Gpio19Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 364: GPIO19"]
pub mod gpio19;
#[doc = "gpio20 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 368: GPIO20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio20`]
module"]
#[doc(alias = "gpio20")]
pub type Gpio20 = crate::Reg<gpio20::Gpio20Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 368: GPIO20"]
pub mod gpio20;
#[doc = "gpio21 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 372: GPIO21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio21`]
module"]
#[doc(alias = "gpio21")]
pub type Gpio21 = crate::Reg<gpio21::Gpio21Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 372: GPIO21"]
pub mod gpio21;
#[doc = "gpio22 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 376: GPIO22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio22`]
module"]
#[doc(alias = "gpio22")]
pub type Gpio22 = crate::Reg<gpio22::Gpio22Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 376: GPIO22"]
pub mod gpio22;
#[doc = "gpio23 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 380: GPIO23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio23`]
module"]
#[doc(alias = "gpio23")]
pub type Gpio23 = crate::Reg<gpio23::Gpio23Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 380: GPIO23"]
pub mod gpio23;
#[doc = "gpio24 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 384: GPIO24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio24`]
module"]
#[doc(alias = "gpio24")]
pub type Gpio24 = crate::Reg<gpio24::Gpio24Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 384: GPIO24"]
pub mod gpio24;
#[doc = "gpio25 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 388: GPIO25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio25`]
module"]
#[doc(alias = "gpio25")]
pub type Gpio25 = crate::Reg<gpio25::Gpio25Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 388: GPIO25"]
pub mod gpio25;
#[doc = "gpio26 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 392: GPIO26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio26`]
module"]
#[doc(alias = "gpio26")]
pub type Gpio26 = crate::Reg<gpio26::Gpio26Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 392: GPIO26"]
pub mod gpio26;
#[doc = "gpio27 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 396: GPIO27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio27`]
module"]
#[doc(alias = "gpio27")]
pub type Gpio27 = crate::Reg<gpio27::Gpio27Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 396: GPIO27"]
pub mod gpio27;
#[doc = "gpio28 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 400: GPIO28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio28`]
module"]
#[doc(alias = "gpio28")]
pub type Gpio28 = crate::Reg<gpio28::Gpio28Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 400: GPIO28"]
pub mod gpio28;
#[doc = "gpio29 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 404: GPIO29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio29`]
module"]
#[doc(alias = "gpio29")]
pub type Gpio29 = crate::Reg<gpio29::Gpio29Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 404: GPIO29"]
pub mod gpio29;
#[doc = "gpio30 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 408: GPIO30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio30`]
module"]
#[doc(alias = "gpio30")]
pub type Gpio30 = crate::Reg<gpio30::Gpio30Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 408: GPIO30"]
pub mod gpio30;
#[doc = "gpio31 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 412: GPIO31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio31`]
module"]
#[doc(alias = "gpio31")]
pub type Gpio31 = crate::Reg<gpio31::Gpio31Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 412: GPIO31"]
pub mod gpio31;
#[doc = "gpio32 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 416: GPIO32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio32`]
module"]
#[doc(alias = "gpio32")]
pub type Gpio32 = crate::Reg<gpio32::Gpio32Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 416: GPIO32"]
pub mod gpio32;
#[doc = "gpio33 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 420: GPIO33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio33`]
module"]
#[doc(alias = "gpio33")]
pub type Gpio33 = crate::Reg<gpio33::Gpio33Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 420: GPIO33"]
pub mod gpio33;
#[doc = "gpio34 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 424: GPIO34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio34`]
module"]
#[doc(alias = "gpio34")]
pub type Gpio34 = crate::Reg<gpio34::Gpio34Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 424: GPIO34"]
pub mod gpio34;
#[doc = "gpio35 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 428: GPIO35\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio35`]
module"]
#[doc(alias = "gpio35")]
pub type Gpio35 = crate::Reg<gpio35::Gpio35Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 428: GPIO35"]
pub mod gpio35;
#[doc = "gpio36 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 432: GPIO36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio36`]
module"]
#[doc(alias = "gpio36")]
pub type Gpio36 = crate::Reg<gpio36::Gpio36Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 432: GPIO36"]
pub mod gpio36;
#[doc = "gpio37 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 436: GPIO37\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio37::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio37::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio37`]
module"]
#[doc(alias = "gpio37")]
pub type Gpio37 = crate::Reg<gpio37::Gpio37Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 436: GPIO37"]
pub mod gpio37;
#[doc = "gpio38 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 440: GPIO38\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio38::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio38::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio38`]
module"]
#[doc(alias = "gpio38")]
pub type Gpio38 = crate::Reg<gpio38::Gpio38Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 440: GPIO38"]
pub mod gpio38;
#[doc = "gpio39 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 444: GPIO39\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio39::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio39`]
module"]
#[doc(alias = "gpio39")]
pub type Gpio39 = crate::Reg<gpio39::Gpio39Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 444: GPIO39"]
pub mod gpio39;
#[doc = "gpio40 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 448: GPIO40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio40::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio40::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio40`]
module"]
#[doc(alias = "gpio40")]
pub type Gpio40 = crate::Reg<gpio40::Gpio40Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 448: GPIO40"]
pub mod gpio40;
#[doc = "gpio41 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 452: GPIO41\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio41::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio41::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio41`]
module"]
#[doc(alias = "gpio41")]
pub type Gpio41 = crate::Reg<gpio41::Gpio41Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 452: GPIO41"]
pub mod gpio41;
#[doc = "gpio42 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 456: GPIO42\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio42::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio42::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio42`]
module"]
#[doc(alias = "gpio42")]
pub type Gpio42 = crate::Reg<gpio42::Gpio42Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 456: GPIO42"]
pub mod gpio42;
#[doc = "gpio43 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 460: GPIO43\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio43::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio43::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio43`]
module"]
#[doc(alias = "gpio43")]
pub type Gpio43 = crate::Reg<gpio43::Gpio43Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 460: GPIO43"]
pub mod gpio43;
#[doc = "gpio44 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 464: GPIO44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio44::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio44::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio44`]
module"]
#[doc(alias = "gpio44")]
pub type Gpio44 = crate::Reg<gpio44::Gpio44Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 464: GPIO44"]
pub mod gpio44;
#[doc = "gpio45 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 468: GPIO45\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio45`]
module"]
#[doc(alias = "gpio45")]
pub type Gpio45 = crate::Reg<gpio45::Gpio45Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 468: GPIO45"]
pub mod gpio45;
#[doc = "gpio46 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 472: GPIO46\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio46::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio46::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio46`]
module"]
#[doc(alias = "gpio46")]
pub type Gpio46 = crate::Reg<gpio46::Gpio46Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 472: GPIO46"]
pub mod gpio46;
#[doc = "gpio47 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 476: GPIO47\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio47::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio47::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio47`]
module"]
#[doc(alias = "gpio47")]
pub type Gpio47 = crate::Reg<gpio47::Gpio47Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 476: GPIO47"]
pub mod gpio47;
#[doc = "gpio48 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 480: GPIO48\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio48::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio48::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio48`]
module"]
#[doc(alias = "gpio48")]
pub type Gpio48 = crate::Reg<gpio48::Gpio48Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 480: GPIO48"]
pub mod gpio48;
#[doc = "gpio49 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 484: GPIO49\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio49::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio49::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio49`]
module"]
#[doc(alias = "gpio49")]
pub type Gpio49 = crate::Reg<gpio49::Gpio49Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 484: GPIO49"]
pub mod gpio49;
#[doc = "gpio50 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 488: GPIO50\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio50::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio50::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio50`]
module"]
#[doc(alias = "gpio50")]
pub type Gpio50 = crate::Reg<gpio50::Gpio50Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 488: GPIO50"]
pub mod gpio50;
#[doc = "gpio51 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 492: GPIO51\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio51::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio51::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio51`]
module"]
#[doc(alias = "gpio51")]
pub type Gpio51 = crate::Reg<gpio51::Gpio51Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 492: GPIO51"]
pub mod gpio51;
#[doc = "gpio52 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 496: GPIO52\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio52::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio52::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio52`]
module"]
#[doc(alias = "gpio52")]
pub type Gpio52 = crate::Reg<gpio52::Gpio52Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 496: GPIO52"]
pub mod gpio52;
#[doc = "gpio53 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 500: GPIO53\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio53::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio53::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio53`]
module"]
#[doc(alias = "gpio53")]
pub type Gpio53 = crate::Reg<gpio53::Gpio53Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 500: GPIO53"]
pub mod gpio53;
#[doc = "gpio54 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 504: GPIO54\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio54::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio54::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio54`]
module"]
#[doc(alias = "gpio54")]
pub type Gpio54 = crate::Reg<gpio54::Gpio54Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 504: GPIO54"]
pub mod gpio54;
#[doc = "gpio55 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 508: GPIO55\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio55::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio55::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio55`]
module"]
#[doc(alias = "gpio55")]
pub type Gpio55 = crate::Reg<gpio55::Gpio55Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 508: GPIO55"]
pub mod gpio55;
#[doc = "gpio56 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 512: GPIO56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio56::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio56::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio56`]
module"]
#[doc(alias = "gpio56")]
pub type Gpio56 = crate::Reg<gpio56::Gpio56Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 512: GPIO56"]
pub mod gpio56;
#[doc = "gpio57 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 516: GPIO57\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio57::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio57::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio57`]
module"]
#[doc(alias = "gpio57")]
pub type Gpio57 = crate::Reg<gpio57::Gpio57Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 516: GPIO57"]
pub mod gpio57;
#[doc = "gpio58 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 520: GPIO58\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio58::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio58::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio58`]
module"]
#[doc(alias = "gpio58")]
pub type Gpio58 = crate::Reg<gpio58::Gpio58Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 520: GPIO58"]
pub mod gpio58;
#[doc = "gpio59 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 524: GPIO59\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio59::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio59::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio59`]
module"]
#[doc(alias = "gpio59")]
pub type Gpio59 = crate::Reg<gpio59::Gpio59Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 524: GPIO59"]
pub mod gpio59;
#[doc = "gpio60 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 528: GPIO60\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio60::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio60::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio60`]
module"]
#[doc(alias = "gpio60")]
pub type Gpio60 = crate::Reg<gpio60::Gpio60Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 528: GPIO60"]
pub mod gpio60;
#[doc = "gpio61 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 532: GPIO61\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio61::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio61::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio61`]
module"]
#[doc(alias = "gpio61")]
pub type Gpio61 = crate::Reg<gpio61::Gpio61Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 532: GPIO61"]
pub mod gpio61;
#[doc = "gpio62 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 536: GPIO62\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio62::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio62::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio62`]
module"]
#[doc(alias = "gpio62")]
pub type Gpio62 = crate::Reg<gpio62::Gpio62Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 536: GPIO62"]
pub mod gpio62;
#[doc = "gpio63 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 540: GPIO63\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio63::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio63::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio63`]
module"]
#[doc(alias = "gpio63")]
pub type Gpio63 = crate::Reg<gpio63::Gpio63Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 540: GPIO63"]
pub mod gpio63;
#[doc = "sd0_clk (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 544: SD0_CLK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sd0_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sd0_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd0_clk`]
module"]
#[doc(alias = "sd0_clk")]
pub type Sd0Clk = crate::Reg<sd0_clk::Sd0ClkSpec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 544: SD0_CLK"]
pub mod sd0_clk;
#[doc = "sd0_cmd (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 548: SD0_CMD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sd0_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sd0_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd0_cmd`]
module"]
#[doc(alias = "sd0_cmd")]
pub type Sd0Cmd = crate::Reg<sd0_cmd::Sd0CmdSpec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 548: SD0_CMD"]
pub mod sd0_cmd;
#[doc = "sd0_data0 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 552: SD0_DATA0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sd0_data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sd0_data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd0_data0`]
module"]
#[doc(alias = "sd0_data0")]
pub type Sd0Data0 = crate::Reg<sd0_data0::Sd0Data0Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 552: SD0_DATA0"]
pub mod sd0_data0;
#[doc = "sd0_data1 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 556: SD0_DATA1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sd0_data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sd0_data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd0_data1`]
module"]
#[doc(alias = "sd0_data1")]
pub type Sd0Data1 = crate::Reg<sd0_data1::Sd0Data1Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 556: SD0_DATA1"]
pub mod sd0_data1;
#[doc = "sd0_data2 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 560: SD0_DATA2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sd0_data2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sd0_data2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd0_data2`]
module"]
#[doc(alias = "sd0_data2")]
pub type Sd0Data2 = crate::Reg<sd0_data2::Sd0Data2Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 560: SD0_DATA2"]
pub mod sd0_data2;
#[doc = "sd0_data3 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 564: SD0_DATA3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sd0_data3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sd0_data3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd0_data3`]
module"]
#[doc(alias = "sd0_data3")]
pub type Sd0Data3 = crate::Reg<sd0_data3::Sd0Data3Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 564: SD0_DATA3"]
pub mod sd0_data3;
#[doc = "sd0_data4 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 568: SD0_DATA4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sd0_data4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sd0_data4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd0_data4`]
module"]
#[doc(alias = "sd0_data4")]
pub type Sd0Data4 = crate::Reg<sd0_data4::Sd0Data4Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 568: SD0_DATA4"]
pub mod sd0_data4;
#[doc = "sd0_data5 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 572: SD0_DATA5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sd0_data5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sd0_data5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd0_data5`]
module"]
#[doc(alias = "sd0_data5")]
pub type Sd0Data5 = crate::Reg<sd0_data5::Sd0Data5Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 572: SD0_DATA5"]
pub mod sd0_data5;
#[doc = "sd0_data6 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 576: SD0_DATA6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sd0_data6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sd0_data6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd0_data6`]
module"]
#[doc(alias = "sd0_data6")]
pub type Sd0Data6 = crate::Reg<sd0_data6::Sd0Data6Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 576: SD0_DATA6"]
pub mod sd0_data6;
#[doc = "sd0_data7 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 580: SD0_DATA7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sd0_data7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sd0_data7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd0_data7`]
module"]
#[doc(alias = "sd0_data7")]
pub type Sd0Data7 = crate::Reg<sd0_data7::Sd0Data7Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 580: SD0_DATA7"]
pub mod sd0_data7;
#[doc = "sd0_strb (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 584: SD0_STRB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sd0_strb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sd0_strb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd0_strb`]
module"]
#[doc(alias = "sd0_strb")]
pub type Sd0Strb = crate::Reg<sd0_strb::Sd0StrbSpec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 584: SD0_STRB"]
pub mod sd0_strb;
#[doc = "gmac1 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG: GPIO GMAC1 pads\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac1`]
module"]
#[doc(alias = "gmac1")]
pub type Gmac1 = crate::Reg<gmac1::Gmac1Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG: GPIO GMAC1 pads"]
pub mod gmac1;
#[doc = "qspi_sclk (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 644: QSPI_SCLK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qspi_sclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qspi_sclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qspi_sclk`]
module"]
#[doc(alias = "qspi_sclk")]
pub type QspiSclk = crate::Reg<qspi_sclk::QspiSclkSpec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 644: QSPI_SCLK"]
pub mod qspi_sclk;
#[doc = "qspi_csn0 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 648: QSPI_CSN0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qspi_csn0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qspi_csn0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qspi_csn0`]
module"]
#[doc(alias = "qspi_csn0")]
pub type QspiCsn0 = crate::Reg<qspi_csn0::QspiCsn0Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 648: QSPI_CSN0"]
pub mod qspi_csn0;
#[doc = "qspi_data (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 28c-298: QSPI_DATA 0-3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qspi_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qspi_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qspi_data`]
module"]
#[doc(alias = "qspi_data")]
pub type QspiData = crate::Reg<qspi_data::QspiDataSpec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 28c-298: QSPI_DATA 0-3"]
pub mod qspi_data;
