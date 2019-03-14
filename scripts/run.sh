#!/bin/sh

LIBS="libgtk-3-dev"
RUSTFLAGS="-Ctarget-cpu=native"
RBUILDFLAGS="--release"

for LIB in $LIBS; do
	if [ `dpkg -l | grep $LIB | wc -l` -eq 0 ]; then
		echo "$LIB not installed"

		exit 0
	fi
done

cargo +beta run $RBUILDFLAGS
