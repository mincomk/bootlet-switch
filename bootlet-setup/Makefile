TARGET = x86_64-unknown-linux-musl
TMP_DIR = .tmp

.PHONY: all clean

all: bootlet.img

bootlet.img: bin/run-bootlet-switch bin/it bin/bootit
	@echo "Creating bootlet image..."
	bootlet -o bootlet.img

clean:
	rm -rf $(TMP_DIR) bin/ bootlet.img

$(TMP_DIR):
	mkdir -p $(TMP_DIR)

bin/:
	mkdir -p bin/

bin/run-bootlet-switch: | bin/
	cd ../run-bootlet-switch/ && cargo build --release --target $(TARGET)
	cp ../target/$(TARGET)/release/run-bootlet-switch bin/run-bootlet-switch

bin/it bin/bootit &: | bin/ $(TMP_DIR)
	wget https://github.com/mincomk/bootit/releases/latest/download/bootit-linux-x86_64.tar.gz -P $(TMP_DIR)
	tar -xzf $(TMP_DIR)/bootit-linux-x86_64.tar.gz -C $(TMP_DIR)
	mv $(TMP_DIR)/it bin/it
	mv $(TMP_DIR)/bootit bin/bootit
