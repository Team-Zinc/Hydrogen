use crate::project::*;
use crate::policy::policies;
use crate::policy::Policy;

use indicatif::ProgressBar;
use console::style;
use log::*;

/// Check the Project struct for sanity
pub fn check_and_print<'a>(/* project: &'a Project */) {
    let bar = ProgressBar::new_spinner();

    bar.set_prefix(&format!("{}", style("[***]").bold().dim()));
    for policy in policies::POLICIES.iter() {
        /* debug!("checking policy for {} ::: {} ({})",
            &project.get_full_name(),
            policy.description,
            policy.name
        );

        bar.set_message(&[
            &project.get_full_name(), " (",
            &project.name, ") ::: ",
            policy.description]
        .join("")); */

        bar.tick();

        // Why this weird syntax? See:
        // https://stackoverflow.com/questions/37370120/right-way-to-have-function-pointers-in-struct
        // Most likely, it's just a compiler oversight.
        /* if (policy.valid)(project) == false {
            error!("policy check failed for {} ::: {} ({})",
                &project.get_full_name(),
                policy.description,
                policy.name
            );
            
            print_policy_failed(policy);
        } */
    }

    bar.finish_and_clear();
}

/// Here we output all the failed policies for the user to see.
fn print_policy_failed(p: &Policy) {
    if p.important {
        eprintln!("{} {}: {}",
            style("[!!!]").bold().red(), p.name, p.message
        );

        return;
    }

    eprintln!("{} {}: {}",
        style("[???]").bold().yellow(), p.name, p.message
    );
}