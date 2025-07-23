#[doc = "Register `I2CFILTERTHR18` reader"]
pub type R = crate::R<I2cfilterthr18Spec>;
#[doc = "Register `I2CFILTERTHR18` writer"]
pub type W = crate::W<I2cfilterthr18Spec>;
#[doc = "Field `WtableFail` reader - wtable fail"]
pub type WtableFailR = crate::BitReader;
#[doc = "Field `WtableFail` writer - wtable fail"]
pub type WtableFailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SlaveStart` reader - slave start"]
pub type SlaveStartR = crate::BitReader;
#[doc = "Field `SlaveStart` writer - slave start"]
pub type SlaveStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SlaveStop` reader - slave stop"]
pub type SlaveStopR = crate::BitReader;
#[doc = "Field `SlaveStop` writer - slave stop"]
pub type SlaveStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AbnMstart` reader - abnormal mstart"]
pub type AbnMstartR = crate::BitReader;
#[doc = "Field `AbnMstart` writer - abnormal mstart"]
pub type AbnMstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AbnMstop` reader - abnormal mstop"]
pub type AbnMstopR = crate::BitReader;
#[doc = "Field `AbnMstop` writer - abnormal mstop"]
pub type AbnMstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SlaveSclLowTimeout` reader - slave scl low timeout"]
pub type SlaveSclLowTimeoutR = crate::BitReader;
#[doc = "Field `SlaveSclLowTimeout` writer - slave scl low timeout"]
pub type SlaveSclLowTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SlaveSdaLowTimeout` reader - slave sda low timeout"]
pub type SlaveSdaLowTimeoutR = crate::BitReader;
#[doc = "Field `SlaveSdaLowTimeout` writer - slave sda low timeout"]
pub type SlaveSdaLowTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - wtable fail"]
    #[inline(always)]
    pub fn wtable_fail(&self) -> WtableFailR {
        WtableFailR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - slave start"]
    #[inline(always)]
    pub fn slave_start(&self) -> SlaveStartR {
        SlaveStartR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - slave stop"]
    #[inline(always)]
    pub fn slave_stop(&self) -> SlaveStopR {
        SlaveStopR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - abnormal mstart"]
    #[inline(always)]
    pub fn abn_mstart(&self) -> AbnMstartR {
        AbnMstartR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - abnormal mstop"]
    #[inline(always)]
    pub fn abn_mstop(&self) -> AbnMstopR {
        AbnMstopR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - slave scl low timeout"]
    #[inline(always)]
    pub fn slave_scl_low_timeout(&self) -> SlaveSclLowTimeoutR {
        SlaveSclLowTimeoutR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - slave sda low timeout"]
    #[inline(always)]
    pub fn slave_sda_low_timeout(&self) -> SlaveSdaLowTimeoutR {
        SlaveSdaLowTimeoutR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - wtable fail"]
    #[inline(always)]
    pub fn wtable_fail(&mut self) -> WtableFailW<I2cfilterthr18Spec> {
        WtableFailW::new(self, 0)
    }
    #[doc = "Bit 2 - slave start"]
    #[inline(always)]
    pub fn slave_start(&mut self) -> SlaveStartW<I2cfilterthr18Spec> {
        SlaveStartW::new(self, 2)
    }
    #[doc = "Bit 3 - slave stop"]
    #[inline(always)]
    pub fn slave_stop(&mut self) -> SlaveStopW<I2cfilterthr18Spec> {
        SlaveStopW::new(self, 3)
    }
    #[doc = "Bit 4 - abnormal mstart"]
    #[inline(always)]
    pub fn abn_mstart(&mut self) -> AbnMstartW<I2cfilterthr18Spec> {
        AbnMstartW::new(self, 4)
    }
    #[doc = "Bit 5 - abnormal mstop"]
    #[inline(always)]
    pub fn abn_mstop(&mut self) -> AbnMstopW<I2cfilterthr18Spec> {
        AbnMstopW::new(self, 5)
    }
    #[doc = "Bit 6 - slave scl low timeout"]
    #[inline(always)]
    pub fn slave_scl_low_timeout(&mut self) -> SlaveSclLowTimeoutW<I2cfilterthr18Spec> {
        SlaveSclLowTimeoutW::new(self, 6)
    }
    #[doc = "Bit 7 - slave sda low timeout"]
    #[inline(always)]
    pub fn slave_sda_low_timeout(&mut self) -> SlaveSdaLowTimeoutW<I2cfilterthr18Spec> {
        SlaveSdaLowTimeoutW::new(self, 7)
    }
}
#[doc = "I2CFLT\\_THRN\\_INTS\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cfilterthr18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cfilterthr18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cfilterthr18Spec;
impl crate::RegisterSpec for I2cfilterthr18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cfilterthr18::R`](R) reader structure"]
impl crate::Readable for I2cfilterthr18Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cfilterthr18::W`](W) writer structure"]
impl crate::Writable for I2cfilterthr18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CFILTERTHR18 to value 0"]
impl crate::Resettable for I2cfilterthr18Spec {}
