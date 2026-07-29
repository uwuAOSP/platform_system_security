pub(crate) fn should_route_to_keybox<F>(has_attestation_challenge: bool, keybox_enabled: F) -> bool
where
    F: FnOnce() -> bool,
{
    has_attestation_challenge && keybox_enabled()
}

#[cfg(test)]
mod tests {
    use super::should_route_to_keybox;
    use std::cell::Cell;

    #[test]
    fn routing_requires_challenge_and_service_opt_in() {
        let queried = Cell::new(false);
        assert!(!should_route_to_keybox(false, || {
            queried.set(true);
            true
        }));
        assert!(!queried.get());

        for (has_challenge, service_opt_in, expected) in
            [(false, false, false), (false, true, false), (true, false, false), (true, true, true)]
        {
            assert_eq!(should_route_to_keybox(has_challenge, || service_opt_in), expected);
        }
    }
}
