#![no_std]
#![no_main]

pub mod uart;

use core::arch::global_asm;

#[panic_handler]
fn panic_fmt(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

/* Copyright (c) 2019 Stephen Marz
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

global_asm! { r#"
# Megabyte of stack
.equ STACK_SIZE, 1036288

.global _start

_start:
    # Set up stacks
    csrr t0, mhartid
    slli t0, t0, 10
    la   sp, stacks + STACK_SIZE

    add sp, sp, t0

    csrr a0, mhartid
    bnez a0, park

    j main

park:
    wfi
    j park

stacks:
    .skip STACK_SIZE
"# }
