use arduino_uno::prelude::*;
use atmega328p_hal::clock::MHz16;
use atmega328p_hal::pac::USART0;
use atmega328p_hal::port::mode::Floating;
use atmega328p_hal::port::mode::{Input, Output};
use atmega328p_hal::port::portd::{PD0, PD1};
use atmega328p_hal::usart::UsartReader;
use core::pin::Pin;
use core::task::Context;
use core::task::Poll;
use futures::stream::Stream;

type Rx = UsartReader<USART0, PD0<Input<Floating>>, PD1<Output>, MHz16>;

pub struct AsyncUsartReader(Rx);

impl AsyncUsartReader {
    pub fn new(rx: Rx) -> Self {
        Self(rx)
    }
}

impl Stream for AsyncUsartReader {
    type Item = u8;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.0.read() {
            Ok(byte) => Poll::Ready(Some(byte)),
            Err(e) => Poll::Pending,
        }
    }
}
