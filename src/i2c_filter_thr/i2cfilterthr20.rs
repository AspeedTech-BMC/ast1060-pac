#[doc = "Register `I2CFILTERTHR20` reader"]
pub type R = crate::R<I2cfilterthr20Spec>;
#[doc = "Register `I2CFILTERTHR20` writer"]
pub type W = crate::W<I2cfilterthr20Spec>;
#[doc = "Field `Failfifonoempty` reader - fail_fifo_noempty"]
pub type FailfifonoemptyR = crate::BitReader;
#[doc = "Field `Failfifonoempty` writer - fail_fifo_noempty"]
pub type FailfifonoemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Failfifofull` reader - fail_fifo_full"]
pub type FailfifofullR = crate::BitReader;
#[doc = "Field `Failfifofull` writer - fail_fifo_full"]
pub type FailfifofullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - reserved0"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `Reserved0` writer - reserved0"]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Wlisten` reader - wlist_en"]
pub type WlistenR = crate::BitReader;
#[doc = "Field `Wlisten` writer - wlist_en"]
pub type WlistenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Wtablefail` reader - wtable_fail"]
pub type WtablefailR = crate::BitReader;
#[doc = "Field `Wtablefail` writer - wtable_fail"]
pub type WtablefailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved1"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved1"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Failrpt` reader - fail_rpt"]
pub type FailrptR = crate::FieldReader;
#[doc = "Field `Failrpt` writer - fail_rpt"]
pub type FailrptW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `Failwpt` reader - fail_wpt"]
pub type FailwptR = crate::FieldReader;
#[doc = "Field `Failwpt` writer - fail_wpt"]
pub type FailwptW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `Reserved2` reader - reserved2"]
pub type Reserved2R = crate::FieldReader<u16>;
#[doc = "Field `Reserved2` writer - reserved2"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - fail_fifo_noempty"]
    #[inline(always)]
    pub fn failfifonoempty(&self) -> FailfifonoemptyR {
        FailfifonoemptyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - fail_fifo_full"]
    #[inline(always)]
    pub fn failfifofull(&self) -> FailfifofullR {
        FailfifofullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - reserved0"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - wlist_en"]
    #[inline(always)]
    pub fn wlisten(&self) -> WlistenR {
        WlistenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - wtable_fail"]
    #[inline(always)]
    pub fn wtablefail(&self) -> WtablefailR {
        WtablefailR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - reserved1"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - fail_rpt"]
    #[inline(always)]
    pub fn failrpt(&self) -> FailrptR {
        FailrptR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - fail_wpt"]
    #[inline(always)]
    pub fn failwpt(&self) -> FailwptR {
        FailwptR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - reserved2"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - fail_fifo_noempty"]
    #[inline(always)]
    pub fn failfifonoempty(&mut self) -> FailfifonoemptyW<I2cfilterthr20Spec> {
        FailfifonoemptyW::new(self, 0)
    }
    #[doc = "Bit 1 - fail_fifo_full"]
    #[inline(always)]
    pub fn failfifofull(&mut self) -> FailfifofullW<I2cfilterthr20Spec> {
        FailfifofullW::new(self, 1)
    }
    #[doc = "Bits 2:3 - reserved0"]
    #[inline(always)]
    pub fn reserved0(&mut self) -> Reserved0W<I2cfilterthr20Spec> {
        Reserved0W::new(self, 2)
    }
    #[doc = "Bit 4 - wlist_en"]
    #[inline(always)]
    pub fn wlisten(&mut self) -> WlistenW<I2cfilterthr20Spec> {
        WlistenW::new(self, 4)
    }
    #[doc = "Bit 5 - wtable_fail"]
    #[inline(always)]
    pub fn wtablefail(&mut self) -> WtablefailW<I2cfilterthr20Spec> {
        WtablefailW::new(self, 5)
    }
    #[doc = "Bits 6:7 - reserved1"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<I2cfilterthr20Spec> {
        Reserved1W::new(self, 6)
    }
    #[doc = "Bits 8:11 - fail_rpt"]
    #[inline(always)]
    pub fn failrpt(&mut self) -> FailrptW<I2cfilterthr20Spec> {
        FailrptW::new(self, 8)
    }
    #[doc = "Bits 12:15 - fail_wpt"]
    #[inline(always)]
    pub fn failwpt(&mut self) -> FailwptW<I2cfilterthr20Spec> {
        FailwptW::new(self, 12)
    }
    #[doc = "Bits 16:31 - reserved2"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<I2cfilterthr20Spec> {
        Reserved2W::new(self, 16)
    }
}
#[doc = "I2CFLT\\_THRN\\_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cfilterthr20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cfilterthr20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cfilterthr20Spec;
impl crate::RegisterSpec for I2cfilterthr20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cfilterthr20::R`](R) reader structure"]
impl crate::Readable for I2cfilterthr20Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cfilterthr20::W`](W) writer structure"]
impl crate::Writable for I2cfilterthr20Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CFILTERTHR20 to value 0"]
impl crate::Resettable for I2cfilterthr20Spec {}
