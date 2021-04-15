//! Validates 'route' and 'destination' in the configuration file for potential attacks.
//! 
//! Panic if routed files are missing or can't be read.
//! Other files in the `static` dir which are not routed should be checked by the user.
//! That's to say, user should know what they want when it comes to the js/css files.

use std::fs::symlink_metadata;
use std::process;

use crate::config::get_config;

/// reading symlink files pointing to a sensitive file can leak information,
/// this function validates a file is a symlink or not
fn safe_symlink(file: String) -> i32 {
    let config = get_config();

    let mut vulns_found = 0;

    let static_dir = config["static_directory"].to_string().replace("\"", "");
    let file_path = format!("{}/{}", static_dir, file);

    // check if file is a symlink
    let file_metadata = symlink_metadata(file_path.to_string()).expect(
        &format!(
            "Config Error: routed file `{}` missing or needs changing permission",
            file_path
        )
    );
    // if `follow_symlinks` is disabled in config, and found symlinks, show errors
    if !config["follow_symlinks"].as_bool().unwrap() && file_metadata.file_type().is_symlink() 
    {
        // increment vulnerabilities found
        vulns_found += 1;
        // print out help and error message to rectify the issues
        eprintln!(
            "\n[!] ERROR::FOUND_SYMLINK: The `{}/{}` file is a symlink.\n",
            static_dir, file
        );
        eprintln!(
            "\n[-] INFO: You've disabled symlinks in your configuration as it can lead to potential attacks.\n"
        );
        eprintln!(
            "\n[?] WHAT TO DO: You can either allow symlinks or delete the symlink file at `{}/{}`\n",
            static_dir, file
        );
    }

    vulns_found
}

/// reading files outside the process directory can leak information,
/// this function checks if the file is outside the process directory.
fn path_traversal(route: String) -> i32 {
    let mut vulns_found = 0;

    if route.contains("..") {
        // increment vulnerabilities found
        vulns_found += 1;
        // print out help and error message to rectify the issues
        println!(
            "\n[!] ERROR::PATH_TRAVERSAL: The `{}` file is pointed outside the `static`\
            directory you specified in your configuration.\n",
            route
        );
    }

    vulns_found
}

/// pass the routed files to symlink validation
fn validate_file(routes: serde_json::map::Iter) {
    // total vulnerabilities found
    let mut vulns_found = 0;

    // validate route files
    for (_key, value) in routes {
        let file = value.to_string().replace("\"", "");
        // this function returns an `int` of vulnerabilities found
        vulns_found += safe_symlink(file.to_string());
        // this function returns an `int` of vulnerabilities found
        vulns_found += path_traversal(file.to_string());
    }

    // print out notice message and exit the process in case of any potential vulnerabilities
    if vulns_found > 0 {
        println!(
            "\n[!] TOTAL POTENTIAL VULNERABILITIES FOUND: {}\n",
            vulns_found
        );
        println!(
            "\n[-] INFO: Please fix all the potential vulnerable configurations \
            in your `serve_opt.json` to run server.\n"
        );
        process::exit(1);
    }
}

/// pass all the routed files for validation
pub fn check_config_security() {
    let config = get_config();

    // converting JSON structures to iterable structures
    let routes = config["routes"].as_object().unwrap().into_iter();

    validate_file(routes);
}
