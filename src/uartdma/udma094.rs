#[doc = "Register `UDMA094` reader"]
pub type R = crate::R<Udma094Spec>;
#[doc = "Register `UDMA094` writer"]
pub type W = crate::W<Udma094Spec>;
#[doc = "Field `UART3RXWrPointer` reader - UART3 RX write pointer"]
pub type Uart3rxwrPointerR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART3 RX write pointer"]
    #[inline(always)]
    pub fn uart3rxwr_pointer(&self) -> Uart3rxwrPointerR {
        Uart3rxwrPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "UART3 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma094::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma094::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma094Spec;
impl crate::RegisterSpec for Udma094Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma094::R`](R) reader structure"]
impl crate::Readable for Udma094Spec {}
#[doc = "`write(|w| ..)` method takes [`udma094::W`](W) writer structure"]
impl crate::Writable for Udma094Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA094 to value 0"]
impl crate::Resettable for Udma094Spec {}
