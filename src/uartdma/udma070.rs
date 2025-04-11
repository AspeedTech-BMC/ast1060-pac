#[doc = "Register `UDMA070` reader"]
pub type R = crate::R<Udma070Spec>;
#[doc = "Register `UDMA070` writer"]
pub type W = crate::W<Udma070Spec>;
#[doc = "Field `UART2RXReadPointer` reader - UART2 RX read pointer"]
pub type Uart2rxreadPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART2RXReadPointer` writer - UART2 RX read pointer"]
pub type Uart2rxreadPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - UART2 RX read pointer"]
    #[inline(always)]
    pub fn uart2rxread_pointer(&self) -> Uart2rxreadPointerR {
        Uart2rxreadPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART2 RX read pointer"]
    #[inline(always)]
    pub fn uart2rxread_pointer(&mut self) -> Uart2rxreadPointerW<Udma070Spec> {
        Uart2rxreadPointerW::new(self, 0)
    }
}
#[doc = "UART2 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma070::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma070::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma070Spec;
impl crate::RegisterSpec for Udma070Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma070::R`](R) reader structure"]
impl crate::Readable for Udma070Spec {}
#[doc = "`write(|w| ..)` method takes [`udma070::W`](W) writer structure"]
impl crate::Writable for Udma070Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA070 to value 0"]
impl crate::Resettable for Udma070Spec {}
