#[doc = "Register `TIMER004` reader"]
pub type R = crate::R<Timer004Spec>;
#[doc = "Register `TIMER004` writer"]
pub type W = crate::W<Timer004Spec>;
#[doc = "Field `CounterReloadValueReg` reader - Counter reload value register"]
pub type CounterReloadValueRegR = crate::FieldReader<u32>;
#[doc = "Field `CounterReloadValueReg` writer - Counter reload value register"]
pub type CounterReloadValueRegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter reload value register"]
    #[inline(always)]
    pub fn counter_reload_value_reg(&self) -> CounterReloadValueRegR {
        CounterReloadValueRegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter reload value register"]
    #[inline(always)]
    pub fn counter_reload_value_reg(&mut self) -> CounterReloadValueRegW<Timer004Spec> {
        CounterReloadValueRegW::new(self, 0)
    }
}
#[doc = "Counter \\#1 Reload Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer004::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer004::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer004Spec;
impl crate::RegisterSpec for Timer004Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer004::R`](R) reader structure"]
impl crate::Readable for Timer004Spec {}
#[doc = "`write(|w| ..)` method takes [`timer004::W`](W) writer structure"]
impl crate::Writable for Timer004Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMER004 to value 0"]
impl crate::Resettable for Timer004Spec {}
