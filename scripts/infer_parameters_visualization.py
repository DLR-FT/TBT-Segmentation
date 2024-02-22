# SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
# SPDX-License-Identifier: Apache-2.0

import os
import re
import sys
import subprocess


def build_call(filename):
    with open(filename, 'r') as f:
        read_result = False
        skip = True
        all_segments = []
        segments = []
        skipped_entries = 0
        for line in f:
            if skip:
                skip = False
                continue
            if read_result:
                if line.startswith('lower:'):
                    pattern = r"lower:(\s+)(\d+)(\s+)upper:(\s+)(\d+)(.*)"
                    match = re.search(pattern, line)
                    if match:
                        lower_value = int(match.group(2))
                        upper_value = int(match.group(5))
                        segments.append((lower_value, upper_value))
                    else:
                        print(f"Something is of, expected {pattern} but was: {line}")
                        exit()
                else:
                    if segments != []:
                        all_segments.append(segments)
                    segments = []
            else:
                if line.strip().startswith('Is greedy:'):
                    pattern = r"(.*)\((\d+)\)(.*)"
                    match = re.search(pattern, line)
                    if match:
                        skipped_entries = int(match.group(2))                
                if line.startswith('Get segmentation after'):
                    read_result = True
                    skip = True
        behavior = None
        if 'Lateral' in filename:
            behavior = 'Lateral'
        elif '45Deg' in filename:
            behavior = '45Deg'
        elif 'Oblique' in filename:
            behavior = 'Oblique'
        elif 'Straight' in filename:
            behavior = 'Straight'
        else:
            print(f"Unexpected logfile, expected one of the bahviors in the filename but wasnt: {filename}")
        
        calls = []
        for i, segments in enumerate(all_segments):
            output_location = filename[:-4] + "_best"
            if i != 0:
                output_location = filename[:-4] + "_a" + str(i)
            directory = os.path.dirname(filename)
            call = ["python3", "visualize_ship_landing.py", f'-l' ,directory, '-b' ,f'{behavior}', '-s']
            segment_str = []
            for i, (_, upper) in enumerate(segments):
                if i != len(segments)-1:
                    segment_str.append(str(upper))
            for segment in segment_str:
                call.append(segment)
            call = call + ['-p', output_location, '-e', str(skipped_entries), 'plot']
            calls.append(call)
        return calls

if __name__ == "__main__":
    calls  = build_call(sys.argv[1])
    for call in calls:
        subprocess.run(call)
