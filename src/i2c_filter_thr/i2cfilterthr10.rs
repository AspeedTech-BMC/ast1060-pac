#[doc = "Register `I2CFILTERTHR10` reader"]
pub type R = crate::R<I2cfilterthr10Spec>;
#[doc = "Register `I2CFILTERTHR10` writer"]
pub type W = crate::W<I2cfilterthr10Spec>;
#[doc = "Field `SclTimeoutCounter` reader - scl timeout counter"]
pub type SclTimeoutCounterR = crate::FieldReader<u16>;
#[doc = "Field `SclTimeoutCounter` writer - scl timeout counter"]
pub type SclTimeoutCounterW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BlockTimeoutCounterForStartStopUsage` reader - block timeout counter for start / stop usage"]
pub type BlockTimeoutCounterForStartStopUsageR = crate::FieldReader<u16>;
#[doc = "Field `BlockTimeoutCounterForStartStopUsage` writer - block timeout counter for start / stop usage"]
pub type BlockTimeoutCounterForStartStopUsageW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - scl timeout counter"]
    #[inline(always)]
    pub fn scl_timeout_counter(&self) -> SclTimeoutCounterR {
        SclTimeoutCounterR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - block timeout counter for start / stop usage"]
    #[inline(always)]
    pub fn block_timeout_counter_for_start_stop_usage(
        &self,
    ) -> BlockTimeoutCounterForStartStopUsageR {
        BlockTimeoutCounterForStartStopUsageR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - scl timeout counter"]
    #[inline(always)]
    pub fn scl_timeout_counter(&mut self) -> SclTimeoutCounterW<I2cfilterthr10Spec> {
        SclTimeoutCounterW::new(self, 0)
    }
    #[doc = "Bits 16:31 - block timeout counter for start / stop usage"]
    #[inline(always)]
    pub fn block_timeout_counter_for_start_stop_usage(
        &mut self,
    ) -> BlockTimeoutCounterForStartStopUsageW<I2cfilterthr10Spec> {
        BlockTimeoutCounterForStartStopUsageW::new(self, 16)
    }
}
#[doc = "I2CFLT\\_THRN\\_TMR\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cfilterthr10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cfilterthr10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cfilterthr10Spec;
impl crate::RegisterSpec for I2cfilterthr10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cfilterthr10::R`](R) reader structure"]
impl crate::Readable for I2cfilterthr10Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cfilterthr10::W`](W) writer structure"]
impl crate::Writable for I2cfilterthr10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CFILTERTHR10 to value 0"]
impl crate::Resettable for I2cfilterthr10Spec {}
