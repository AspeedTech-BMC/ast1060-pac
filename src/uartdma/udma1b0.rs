#[doc = "Register `UDMA1B0` reader"]
pub type R = crate::R<Udma1b0Spec>;
#[doc = "Register `UDMA1B0` writer"]
pub type W = crate::W<Udma1b0Spec>;
#[doc = "Field `UART12RXReadPointer` reader - UART12 RX read pointer"]
pub type Uart12rxreadPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART12RXReadPointer` writer - UART12 RX read pointer"]
pub type Uart12rxreadPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - UART12 RX read pointer"]
    #[inline(always)]
    pub fn uart12rxread_pointer(&self) -> Uart12rxreadPointerR {
        Uart12rxreadPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART12 RX read pointer"]
    #[inline(always)]
    pub fn uart12rxread_pointer(&mut self) -> Uart12rxreadPointerW<Udma1b0Spec> {
        Uart12rxreadPointerW::new(self, 0)
    }
}
#[doc = "UART12 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma1b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma1b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma1b0Spec;
impl crate::RegisterSpec for Udma1b0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma1b0::R`](R) reader structure"]
impl crate::Readable for Udma1b0Spec {}
#[doc = "`write(|w| ..)` method takes [`udma1b0::W`](W) writer structure"]
impl crate::Writable for Udma1b0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA1B0 to value 0"]
impl crate::Resettable for Udma1b0Spec {}
