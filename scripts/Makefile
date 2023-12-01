# SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
# SPDX-License-Identifier: Apache-2.0
# This script allows to profile the build rust code. Only works with unix systems

############   
# PROFILING
############

profile = RUSTFLAGS='-g' cargo build --release; \
		valgrind --tool=callgrind --callgrind-out-file=callgrind.out \
				 --collect-jumps=yes --simulate-cache=yes \
				 ./target/release/$(1)

hyperfine = RUSTFLAGS='-g' cargo build --release; \
		hyperfine ./target/release/$(1)

profile.os:
	$(call profile,offlinesegmentation)

graph.os:
	kcachegrind callgrind.out

hyperfine.os:
	$(call hyperfine,offlinesegmentation)

clean:
	rm callgrind.out
