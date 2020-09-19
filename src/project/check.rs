use crate::project::*;
use crate::policy::policies;
use crate::policy::Policy;

use indicatif::ProgressBar;
use console::style;
use log::*;

/// Check the Project struct for sanity
pub fn check_hy<'a>(project: &'a Project) -> Vec<&'a Policy<'a>> {
    let mut failed: Vec<&Policy> = Vec::new();
    let bar = ProgressBar::new_spinner();

    bar.set_prefix(&format!("{}", style("[***]").bold().dim()));
    for policy in policies::POLICIES.iter() {
        debug!("checking policy for {} ::: {} ({})",
            &project.get_full_name(),
            policy.description,
            policy.name
        );

        bar.set_message(&[
            &project.get_full_name(),
            " (",
            &project.name,
            ") ::: ",
            policy.description]
        .join(""));
        bar.tick();

        // Why this weird syntax? See:
        // https://stackoverflow.com/questions/37370120/right-way-to-have-function-pointers-in-struct
        // Most likely, it's just a compiler oversight.

        if (policy.valid)(project) == false {
            error!("policy check failed for {} ::: {} ({})",
                &project.get_full_name(),
                policy.description,
                policy.name
            );
            
            failed.push(policy);
        }

    }

    bar.finish_and_clear();

    failed
}

pub fn user_out<'a>(failed: Vec<&'a Policy<'a>>) {
    if failed.len() == 0 {
        return;
    }
        
    println!("{} Some errors/warnings where detected!",
        style("[***]").bold().dim()
    );

    for policy in failed.iter() {
        if policy.important {
            println!("{} {}: {}",
                style("[!!!]").bold().red(), policy.name, policy.message
            );

            continue;
        }

        println!("{} {}: {}",
            style("[???]").bold().yellow(), policy.name, policy.message
        );
    }
}
