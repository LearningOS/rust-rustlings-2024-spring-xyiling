// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.

// 这道题要求补充在创建 `PositiveNonzeroInteger` 时的错误处理逻辑。
// 需要增加new方法的match逻辑
#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            v if v < 0 => return Err(CreationError::Negative),
            0 => return Err(CreationError::Zero),
            x => return Ok(PositiveNonzeroInteger(value as u64))
        }
        // Hmm...? Why is this only returning an Ok value?
        // Ok(PositiveNonzeroInteger(value as u64))
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
