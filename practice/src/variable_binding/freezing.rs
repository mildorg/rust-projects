pub fn practice_freezing() {
    let mut _mutable_integer = 7;

    {
        let _mutable_integer = _mutable_integer;

        // _mutable_integer = 5;
    }

    _mutable_integer = 1;
}
