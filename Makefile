TARGET=thumbv7m-none-eabi
STRIP=arm-none-eabi-strip
OBJCOPY=arm-none-eabi-objcopy
OBJDUMP=arm-none-eabi-objdump

PLATFORM_DIR=src/hal/$(PLATFORM)
LINK_SCRIPT=$(PLATFORM_DIR)/layout.ld

OUT_DIR=target/$(TARGET)/release
EXAMPLE_FILE=$(OUT_DIR)/examples/$(EXAMPLE_NAME)

ISR_SRC=src/hal/isr.rs
ISR_CRATE=$(shell rustc --print crate-name $(ISR_SRC))
ISR_FILE=$(ISR_CRATE).o

LDFLAGS=-mthumb -mcpu=cortex-m3 -T$(LINK_SCRIPT) -lm -lgcc $(ISR_FILE)

.PHONY: build clean listing
build: $(EXAMPLE_NAME).bin

clean:
	-rm *.bin
	-rm *.lst
	-rm $(ISR_FILE)
	cargo clean

listing: $(EXAMPLE_NAME).lst

$(EXAMPLE_FILE): $(ISR_FILE)
	cargo rustc --example $(EXAMPLE_NAME) --release --target=$(TARGET) --verbose -- -C link-args="$(LDFLAGS)"

$(EXAMPLE_NAME).bin: $(EXAMPLE_FILE)
	$(OBJCOPY) -O binary $< $@

$(EXAMPLE_NAME).lst: $(EXAMPLE_FILE)
	$(OBJDUMP) -D $< > $@

$(ISR_FILE): $(ISR_SRC)
	rustc --verbose -L target/$(TARGET)/debug/deps --target=$(TARGET) --emit=obj --cfg mcu_$(PLATFORM) -C opt-level=2 $<
	$(STRIP) -N rust_begin_unwind -N rust_stack_exhausted -N rust_eh_personality $@
