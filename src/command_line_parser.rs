// SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
// SPDX-License-Identifier: Apache-2.0

pub struct CommandLineArguments {
    pub logfile: String,
    pub lazy_evaluation: bool,
    pub sub_sampling: bool,
    pub debug_console: bool,
    pub print_leaf_segments_only: bool,
    pub segmentation_setting: Option<SegmentationSetting>,
}

pub struct SegmentationSetting {
    pub tau_dif: usize,
    pub rho_dif: f32,
    pub amount: usize,
}

impl CommandLineArguments {
    fn new(
        logfile: String,
        lazy_evaluation: bool,
        sub_sampling: bool,
        debug_console: bool,
        print_leaf_segments_only: bool,
        segmentation_setting: Option<SegmentationSetting>,
    ) -> CommandLineArguments {
        CommandLineArguments {
            logfile,
            lazy_evaluation,
            sub_sampling,
            debug_console,
            print_leaf_segments_only,
            segmentation_setting,
        }
    }
}

pub fn parse_command_line() -> CommandLineArguments {
    // Basic app information
    let app = clap::App::new("Temporal Behavior Trees")
        .version("0.1.0")
        .about("Computes robustness and segmentation.")
        .author(clap::crate_authors!("\n"));
    // Define the name command line option
    let logfile = clap::Arg::with_name("logfile")
        .required(true)
        .short("f")
        .long("logfile")
        .takes_value(true)
        .value_name("FILE")
        .help("Get logfile location");
    let lazy_evaluation = clap::Arg::with_name("lazy_evaluation")
        .required(false)
        .short("l")
        .long("lazy")
        .takes_value(false)
        .help("Activates lazy evaluation");
    let sub_sampling = clap::Arg::with_name("sub_sampling")
        .required(false)
        .short("s")
        .long("sampling")
        .takes_value(false)
        .help("Activates subsampling");
    let debugging = clap::Arg::with_name("debugging")
        .required(false)
        .short("d")
        .long("debug")
        .takes_value(false)
        .help("Activates debugging prints");
    let tau_dif = clap::Arg::with_name("tau_dif")
        .required(false)
        .short("t")
        .long("tau")
        .takes_value(true)
        .default_value("20000")
        .value_name("TAU")
        .help("Specifies tau difference for alternative segmentation");
    let rho_dif = clap::Arg::with_name("rho_dif")
        .required(false)
        .short("r")
        .long("rho")
        .takes_value(true)
        .default_value("50.0")
        .value_name("RHO")
        .help("Specifies rho difference for alternative segmentation");
    let amount = clap::Arg::with_name("amount")
        .required(false)
        .short("a")
        .long("amount")
        .takes_value(true)
        .default_value("3")
        .value_name("UINT")
        .help("Specifies number of alternative segmentations");
    let children = clap::Arg::with_name("children")
        .required(false)
        .short("c")
        .long("children")
        .help("Specifies whether only leaf nodes of a segmentation shall be printed");
    // Add to the app to be parsed
    let app = app
        .arg(logfile)
        .arg(lazy_evaluation)
        .arg(sub_sampling)
        .arg(debugging)
        .arg(tau_dif)
        .arg(rho_dif)
        .arg(children)
        .arg(amount);
    // Extract the matches
    let matches = app.get_matches();
    // Extract data
    let logfile = matches
        .value_of("logfile")
        .expect("This can't be None, since it is required")
        .to_string();
    let lazy_evaluation = matches.is_present("lazy_evaluation");
    let sub_sampling = matches.is_present("sub_sampling");
    let debug_console = matches.is_present("debugging");
    let tau_dif = matches
        .value_of("tau_dif")
        .expect("This can't be None, since it is present")
        .parse()
        .unwrap();
    let rho_dif = matches
        .value_of("rho_dif")
        .expect("This can't be None, since it is present")
        .parse()
        .unwrap();
    let amount = matches
        .value_of("amount")
        .expect("This can't be None, since it is present")
        .parse()
        .unwrap();
    let print_leaf_segments_only = matches.is_present("children");
    let segmentation_setting = if !lazy_evaluation {
        Some(SegmentationSetting {
            tau_dif,
            rho_dif,
            amount,
        })
    } else {
        None
    };

    CommandLineArguments::new(
        logfile,
        lazy_evaluation,
        sub_sampling,
        debug_console,
        print_leaf_segments_only,
        segmentation_setting,
    )
}
