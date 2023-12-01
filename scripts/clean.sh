# SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
# SPDX-License-Identifier: Apache-2.0
# This script cleans unnecessary files
 
rm ip.log callgrind.out smt_formula.txt smt_trace_changes.txt smt_model.txt
cargo clean