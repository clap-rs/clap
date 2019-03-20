use build::{Arg, ArgSettings};

pub(crate) fn assert_low_index_multiples(positionals: &[(u64, &Arg)]) {
    // First we make sure if there is a positional that allows multiple values
    // the one before it (second to last) has one of these:
    //  * a value terminator
    //  * ArgSettings::Last
    //  * The last arg is Required

    let last = positionals.iter().last().unwrap().1;

    let second_to_last = positionals.iter().rev().next().unwrap().1;

    // Either the final positional is required
    // Or the second to last has a terminator or .last(true) set
    let ok = (last.is_set(ArgSettings::Required) || last.is_set(ArgSettings::Last))
        || (second_to_last.terminator.is_some()
        || second_to_last.is_set(ArgSettings::Last));
    assert!(
        ok,
        "When using a positional argument with .multiple(true) that is *not the \
            last* positional argument, the last positional argument (i.e the one \
            with the highest index) *must* have .required(true) or .last(true) set."
    );
}

pub(crate) fn assert_missing_positionals(positionals: &[(u64, &Arg)]) {
    // Check that if a required positional argument is found, all positions with a lower
    // index are also required.
    let mut found = false;
    let mut foundx2 = false;

    for (i, p) in positionals.iter().rev() {
        if foundx2 && !p.is_set(ArgSettings::Required) {
            assert!(
                p.is_set(ArgSettings::Required),
                "Found positional argument which is not required with a lower \
                    index than a required positional argument by two or more: {:?} \
                    index {:?}",
                p.id,
                i
            );
        } else if p.is_set(ArgSettings::Required) && !p.is_set(ArgSettings::Last) {
            // Args that .last(true) don't count since they can be required and have
            // positionals with a lower index that aren't required
            // Imagine: prog <req1> [opt1] -- <req2>
            // Both of these are valid invocations:
            //      $ prog r1 -- r2
            //      $ prog r1 o1 -- r2
            if found {
                foundx2 = true;
                continue;
            }
            found = true;
            continue;
        } else {
            found = false;
        }
    }
}

pub(crate) fn assert_only_one_last(positionals: &[(u64, &Arg)]) {
    assert!(
        positionals.iter().fold(0, |acc, (_, p)| if p.is_set(ArgSettings::Last) {
            acc + 1
        } else {
            acc
        }) < 2,
        "Only one positional argument may have last(true) set. Found two."
    );
}

pub(crate) fn assert_required_last_and_subcommands(positionals: &[(u64, &Arg)], has_subcmds: bool, subs_negate_reqs: bool) {
    assert!(!(positionals.iter()
        .any(|(_, p)| p.is_set(ArgSettings::Last) && p.is_set(ArgSettings::Required))
        && has_subcmds
        && !subs_negate_reqs),
            "Having a required positional argument with .last(true) set *and* child \
                subcommands without setting SubcommandsNegateReqs isn't compatible.");
}

pub(crate) fn assert_highest_index_matches_len(positionals: &[(u64, &Arg)]) {
    // Firt we verify that the highest supplied index, is equal to the number of
    // positional arguments to verify there are no gaps (i.e. supplying an index of 1 and 3
    // but no 2)
    let highest_idx = positionals.iter().max_by(|x, y| x.0.cmp(&y.0)).unwrap();

    let num_p = positionals.len();

    assert!(
        highest_idx.0 == num_p as u64,
        "Found positional argument whose index is {} but there \
            are only {} positional arguments defined",
        highest_idx.0,
        num_p
    );
}

