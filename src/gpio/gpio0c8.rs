#[doc = "Register `GPIO0C8` reader"]
pub type R = crate::R<Gpio0c8Spec>;
#[doc = "Register `GPIO0C8` writer"]
pub type W = crate::W<Gpio0c8Spec>;
#[doc = "Field `DataWrittenToGPIO070GPIOLGPIOKGPIOJGPIOI` reader - Data written to GPIO070.(GPIOL/GPIOK/GPIOJ/GPIOI)"]
pub type DataWrittenToGpio070gpiolgpiokgpiojgpioiR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data written to GPIO070.(GPIOL/GPIOK/GPIOJ/GPIOI)"]
    #[inline(always)]
    pub fn data_written_to_gpio070gpiolgpiokgpiojgpioi(
        &self,
    ) -> DataWrittenToGpio070gpiolgpiokgpiojgpioiR {
        DataWrittenToGpio070gpiolgpiokgpiojgpioiR::new(self.bits)
    }
}
impl W {}
#[doc = "GPIO\\_I/J/K/L Data Read Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0c8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0c8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio0c8Spec;
impl crate::RegisterSpec for Gpio0c8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio0c8::R`](R) reader structure"]
impl crate::Readable for Gpio0c8Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio0c8::W`](W) writer structure"]
impl crate::Writable for Gpio0c8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO0C8 to value 0"]
impl crate::Resettable for Gpio0c8Spec {}
