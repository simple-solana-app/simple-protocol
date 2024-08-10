use solana_program::{account_info::AccountInfo, msg};

pub fn execute(percent_tracker_pda: &AccountInfo) {

    msg!(
        "Percent Tracker: {} ({}): {:?}",
        percent_tracker_pda.key,
        percent_tracker_pda.owner,
        percent_tracker_pda.data
    );
}
