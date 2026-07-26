#!/bin/sh
#$ cargo uninstall distrib
cargo binstall -y distrib

distrib

exit
