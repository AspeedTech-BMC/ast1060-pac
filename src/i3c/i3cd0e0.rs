#[doc = "Register `I3CD0E0` reader"]
pub type R = crate::R<I3cd0e0Spec>;
#[doc = "Register `I3CD0E0` writer"]
pub type W = crate::W<I3cd0e0Spec>;
#[doc = "Field `I3CVersionID` reader - I3C Version ID"]
pub type I3cversionIdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - I3C Version ID"]
    #[inline(always)]
    pub fn i3cversion_id(&self) -> I3cversionIdR {
        I3cversionIdR::new(self.bits)
    }
}
impl W {}
#[doc = "I3C Version ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd0e0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd0e0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd0e0Spec;
impl crate::RegisterSpec for I3cd0e0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd0e0::R`](R) reader structure"]
impl crate::Readable for I3cd0e0Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd0e0::W`](W) writer structure"]
impl crate::Writable for I3cd0e0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD0E0 to value 0"]
impl crate::Resettable for I3cd0e0Spec {}
