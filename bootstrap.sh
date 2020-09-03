# 1. Install dependencies
sudo apt install git bc bison flex libssl-dev make libc6-dev libncurses5-dev

# 2. Install toolchain
git clone https://github.com/raspberrypi/tools ~/tools
export PATH="$PATH:$HOME/tools/arm-bcm2708/arm-linux-gnueabihf/bin"

# 3. Get sources
git clone --depth=1 https://github.com/raspberrypi/linux

# 4. Build kernel
cd linux
KERNEL=kernel
make ARCH=arm CROSS_COMPILE=arm-linux-gnueabihf- bcmrpi_defconfig
make ARCH=arm CROSS_COMPILE=arm-linux-gnueabihf- zImage modules dtbs

echo "done"
