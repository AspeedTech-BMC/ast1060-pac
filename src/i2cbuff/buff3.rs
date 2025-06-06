#[doc = "Register `BUFF3` reader"]
pub type R = crate::R<Buff3Spec>;
#[doc = "Register `BUFF3` writer"]
pub type W = crate::W<Buff3Spec>;
#[doc = "Field `Data` reader - data"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `Data` writer - data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<Buff3Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "I2C Buffer Mode Buff3\n\nYou can [`read`](crate::Reg::read) this register and get [`buff3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buff3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Buff3Spec;
impl crate::RegisterSpec for Buff3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buff3::R`](R) reader structure"]
impl crate::Readable for Buff3Spec {}
#[doc = "`write(|w| ..)` method takes [`buff3::W`](W) writer structure"]
impl crate::Writable for Buff3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUFF3 to value 0"]
impl crate::Resettable for Buff3Spec {}
