# SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
# SPDX-License-Identifier: Apache-2.0

import argparse
import math
import matplotlib.pyplot as plt
import pandas as pd
import numpy as np


def read_csv(file, x_str, y_str, z_str, heading_str, is_ship=False):
    df = pd.read_csv(file, delimiter=",")
    x_arr = df[x_str]
    y_arr = df[y_str]
    z_arr = -df[z_str]
    heading_arr = df[heading_str]
    if is_ship:  # https://de.m.wikipedia.org/wiki/Sachsen_(F_219)
        angle_with_heading = np.radians(180) + heading_arr
        # moved the touchdown towards the end of the ship
        x_arr = x_arr + 60.0 * np.cos(angle_with_heading)
        # moved the touchdown towards the end of the ship
        y_arr = y_arr + 60.0 * np.sin(angle_with_heading)
        z_arr = z_arr + 5.0   # moved the touchdown a bit higher

    x_arr = x_arr.to_numpy()
    y_arr = y_arr.to_numpy()
    z_arr = z_arr.to_numpy()
    heading_arr = heading_arr.to_numpy()
    pos = len(z_arr)
    t_arr = [i / pos for i in range(len(z_arr))]
    # Computations
    return x_arr, y_arr, z_arr, heading_arr, t_arr


def show_plot(save_location, behavior, segments, number_of_skipped, x_arr_uas, y_arr_uas, z_arr_uas, x_arr_ship, y_arr_ship, z_arr_ship, heading_arr_ship):
    ax = plt.axes(projection='3d')
    segment_start = 0
    number_of_skipped = 1 if number_of_skipped == 0 else number_of_skipped
    # Draws UAS
    for segment in segments:
        beginning = segment_start * number_of_skipped
        end = segment * number_of_skipped
        ax.plot3D(x_arr_uas[beginning:end],
                  y_arr_uas[beginning:end],
                  z_arr_uas[beginning:end])
        segment_start = segment + 1
    ax.plot3D(x_arr_uas[segment_start:],
              y_arr_uas[segment_start:],
              z_arr_uas[segment_start:])
    # Compute best position
    (best_position_x, best_position_y, best_position_z) = compute_best_position(behavior,
                                                                                x_arr_ship,
                                                                                y_arr_ship,
                                                                                z_arr_ship,
                                                                                heading_arr_ship)
    ax.plot3D(x_arr_ship, y_arr_ship, z_arr_ship)
    ax.plot3D(best_position_x, best_position_y,
              best_position_z, linestyle="dashed", alpha=0.4)
    ax.plot3D(x_arr_ship, y_arr_ship, z_arr_ship +
              20, linestyle="dashed", alpha=0.4)
    ax.set_xlabel("x [m]")
    ax.set_ylabel("y [m]")
    ax.set_zlabel("z [m]")
    plt.legend(['Move to position', 'Stay in position', 'Move to touchdown', 'Descend',
               'Ship', "Maneuver: at position", "Maneuver: above ship",], bbox_to_anchor=(1.25, 1.13), ncol=3)
    if save_location is None:
        plt.show()
    else:
        plt.savefig(save_location)


def live_replay(behavior, sample_time, frequency_logs, speed_up, x_arr_uas, y_arr_uas, z_arr_uas, x_arr_ship, y_arr_ship, z_arr_ship, heading_arr_ship):
    try:
        # Replay
        plt.ion()
        fig = plt.figure()
        ax = fig.add_subplot(111, projection='3d')
        number_events = int((sample_time / frequency_logs) * speed_up)
        print(number_events, frequency_logs, sample_time, speed_up)
        current_step = 0
        while True:
            step = min(current_step, len(x_arr_uas))
            ax.plot(x_arr_uas[:step], y_arr_uas[:step], z_arr_uas[:step])
            ax.plot(x_arr_ship[:step], y_arr_ship[:step], z_arr_ship[:step])
            # Compute best position
            (best_position_x, best_position_y, best_position_z) = compute_best_position(behavior,
                                                                                        x_arr_ship,
                                                                                        y_arr_ship,
                                                                                        z_arr_ship,
                                                                                        heading_arr_ship)
            ax.plot(best_position_x[:step],
                    best_position_y[:step], best_position_z[:step])
            ax.plot3D(x_arr_ship[:step], y_arr_ship[:step], z_arr_ship[:step] +
                      20, linestyle="dashed", alpha=0.4)
            plt.legend(['UAS', 'Ship', 'Maneuver: at position', 'Maneuver: above ship'],
                       bbox_to_anchor=(0.75, 1.15), ncol=2)
            plt.draw()
            plt.pause(sample_time)
            ax.cla()
            if current_step > len(x_arr_ship):
                break
            current_step += number_events
    except:
        print("Stopped the program.")


def compute_best_position(behavior, x_arr_ship, y_arr_ship, z_arr_ship, heading_arr_ship):
    angle_best = -1.0
    meters_behind_ship = 20.0
    meters_above_ship = 20.0
    if "Lateral" == behavior:
        angle_best = 90.0
    elif "45Deg" == behavior:
        angle_best = 135.0
        meters_behind_ship = 30.0
    elif "Straight" == behavior:
        angle_best = 180.0
    elif "Oblique":
        angle_best = 135.0
        meters_behind_ship = 30.0
    else:
        print("Unknown maneuver!")
        exit()
    heading_x = []
    heading_y = []
    for h in heading_arr_ship:
        angle_with_heading = math.radians(angle_best) + h
        heading_x.append(math.cos(angle_with_heading))
        heading_y.append(math.sin(angle_with_heading))
    heading_x = np.array(heading_x)
    heading_y = np.array(heading_y)
    # Compute best position
    best_position_x = x_arr_ship + meters_behind_ship * heading_x
    best_position_y = y_arr_ship + meters_behind_ship * heading_y
    best_position_z = z_arr_ship + meters_above_ship
    return (best_position_x, best_position_y, best_position_z)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        prog='SegmentationVisualizer', description='Given a segmentation and a trace, it visualizes a logfile')
    '''
        Required
    '''
    parser.add_argument('action', choices=[
                        'live', 'plot'], help='Decide of you want to live replay the logfile or plot the segmentation')
    parser.add_argument('-l', '--logfile',
                        help='Provides the location of the logfile')
    parser.add_argument('-b', '--behavior', choices=[
                        'Lateral', '45Deg', 'Straight', 'Oblique'], help='Decide of you want to live replay the logfile or plot the segmentation')
    '''
        Plotting
    '''
    parser.add_argument('-s', '--segment', nargs='+', type=int,
                        help='Provide a list of integers that provide when other segments start')
    parser.add_argument('-e', '--skippedentries', type=int,
                        help='Provide a the number of entries skipped')
    parser.add_argument('-p', '--saveplot', type=str,
                        help='Provides location where to save the plot')
    '''
        Live replay
    '''
    parser.add_argument('-f', '--frequency', nargs=3, type=float, metavar=('<frequency logfile>', '<sample time>', '<speed up>'),
                        help='Provides the frequencies for live replay.')
    # Gets arguments
    args = parser.parse_args()
    try:
        if args.logfile == None:
            print("Error: Expected a logfile!")
            exit()
        logfile = args.logfile
        x_arr_uas, y_arr_uas, z_arr_uas, heading_arr_uas, t_arr_uas = read_csv(
            logfile+"/SIMOUT_UAS.csv", "xg", "yg", "zg", "psi")
        x_arr_ship, y_arr_ship, z_arr_ship, heading_arr_ship, t_arr_ship = read_csv(
            logfile+"/SIMOUT_Ship.csv", "xg", "yg", "zg", "psi", True)
        if args.behavior == None:
            print("Error: Expected to provide ground truth behavior!")
            exit()
        behavior = args.behavior
        if args.action == 'live':
            if args.frequency == None:
                print("Error: Expected to provide frequencies for live replay!")
                exit()
            frequency_logs = args.frequency[0]
            sample_time = args.frequency[1]
            speed_up = args.frequency[2]
            print(
                f"Live ---> Logfile: {logfile}, behavior: {behavior}, sample time: {sample_time}, frequency logs: {frequency_logs}, speed up: {speed_up}")
            live_replay(behavior, sample_time, frequency_logs, speed_up, x_arr_uas, y_arr_uas, z_arr_uas,
                        x_arr_ship, y_arr_ship, z_arr_ship, heading_arr_ship)
        elif args.action == 'plot':
            if args.segment == None:
                print("Error: Expected segment information!")
                exit()
            segments = [int(segment) for segment in args.segment]
            if args.skippedentries == None:
                print("Error: Expected number of skipped entries!")
                exit()
            skipped_entries = args.skippedentries
            save_location = args.saveplot
            print(
                f"Plot ---> Logfile: {logfile}, behavior: {behavior}, segments: {segments}, skipped_entries: {skipped_entries}, save plot: {save_location}")
            show_plot(save_location, behavior, segments, skipped_entries, x_arr_uas, y_arr_uas, z_arr_uas,
                      x_arr_ship, y_arr_ship, z_arr_ship, heading_arr_ship)
    except ValueError:
        print("Error: Invalid input. Make sure positions are either all strings for 'live' or all integers for 'plot'.")
