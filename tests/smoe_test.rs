use chinese_currency::ChineseCurrency;

#[test]
fn smoke_test() {
    assert_eq!(12.to_chinese_currency(), "壹角贰分");
    assert_eq!(10.to_chinese_currency(), "壹角");
    assert_eq!(2.to_chinese_currency(), "零贰分");

    assert_eq!(0.to_chinese_currency(), "零圆整");

    assert_eq!(345.to_chinese_currency(), "叁圆肆角伍分");
    assert_eq!(340.to_chinese_currency(), "叁圆肆角");
    assert_eq!(305.to_chinese_currency(), "叁圆零伍分");
    assert_eq!(300.to_chinese_currency(), "叁圆整");

    assert_eq!(6789.to_chinese_currency(), "陆拾柒圆捌角玖分");
    assert_eq!(6780.to_chinese_currency(), "陆拾柒圆捌角");
    assert_eq!(6709.to_chinese_currency(), "陆拾柒圆零玖分");
    assert_eq!(6089.to_chinese_currency(), "陆拾圆捌角玖分");
    assert_eq!(6700.to_chinese_currency(), "陆拾柒圆整");
    assert_eq!(6009.to_chinese_currency(), "陆拾圆零玖分");
    assert_eq!(6000.to_chinese_currency(), "陆拾圆整");

    assert_eq!(12300.to_chinese_currency(), "壹佰贰拾叁圆整");
    assert_eq!(10300.to_chinese_currency(), "壹佰零叁圆整");
    assert_eq!(12000.to_chinese_currency(), "壹佰贰拾圆整");
    assert_eq!(10000.to_chinese_currency(), "壹佰圆整");

    assert_eq!(123400.to_chinese_currency(), "壹仟贰佰叁拾肆圆整");
    assert_eq!(100400.to_chinese_currency(), "壹仟零肆圆整");

    assert_eq!(
        1234500.to_chinese_currency(),
        "壹万贰仟叁佰肆拾伍圆整"
    );
    assert_eq!(1004500.to_chinese_currency(), "壹万零肆拾伍圆整");
    assert_eq!(1200500.to_chinese_currency(), "壹万贰仟零伍圆整");
    assert_eq!(1030500.to_chinese_currency(), "壹万零叁佰零伍圆整");
    assert_eq!(1000000.to_chinese_currency(), "壹万圆整");

    assert_eq!(
        12345600.to_chinese_currency(),
        "壹拾贰万叁仟肆佰伍拾陆圆整"
    );
    assert_eq!(10000000.to_chinese_currency(), "壹拾万圆整");

    assert_eq!(
        123456700.to_chinese_currency(),
        "壹佰贰拾叁万肆仟伍佰陆拾柒圆整"
    );
    assert_eq!(100000000.to_chinese_currency(), "壹佰万圆整");

    assert_eq!(
        1234567800.to_chinese_currency(),
        "壹仟贰佰叁拾肆万伍仟陆佰柒拾捌圆整"
    );
    assert_eq!(1000000000.to_chinese_currency(), "壹仟万圆整");

    assert_eq!(
        12345678900usize.to_chinese_currency(),
        "壹亿贰仟叁佰肆拾伍万陆仟柒佰捌拾玖圆整"
    );
    assert_eq!(10000000000usize.to_chinese_currency(), "壹亿圆整");

    assert_eq!(
        123456789100usize.to_chinese_currency(),
        "壹拾贰亿叁仟肆佰伍拾陆万柒仟捌佰玖拾壹圆整"
    );
    assert_eq!(100000000000usize.to_chinese_currency(), "壹拾亿圆整");

    assert_eq!(
        1234567891200usize.to_chinese_currency(),
        "壹佰贰拾叁亿肆仟伍佰陆拾柒万捌仟玖佰壹拾贰圆整"
    );
    assert_eq!(1000000000000usize.to_chinese_currency(), "壹佰亿圆整");

    assert_eq!(
        12345678912300usize.to_chinese_currency(),
        "壹仟贰佰叁拾肆亿伍仟陆佰柒拾捌万玖仟壹佰贰拾叁圆整"
    );
    assert_eq!(10000000000000usize.to_chinese_currency(), "壹仟亿圆整");

    assert_eq!(
        123456789123400usize.to_chinese_currency(),
        "壹万亿贰仟叁佰肆拾伍亿陆仟柒佰捌拾玖万壹仟贰佰叁拾肆圆整"
    );
    assert_eq!(
        100000000000000usize.to_chinese_currency(),
        "壹万亿圆整"
    );

    assert_eq!(
        1234567891234500usize.to_chinese_currency(),
        "壹拾贰万亿叁仟肆佰伍拾陆亿柒仟捌佰玖拾壹万贰仟叁佰肆拾伍圆整"
    );
    assert_eq!(
        1000000000000000usize.to_chinese_currency(),
        "壹拾万亿圆整"
    );

    assert_eq!(
        12345678912345600usize.to_chinese_currency(),
        "壹佰贰拾叁万亿肆仟伍佰陆拾柒亿捌仟玖佰壹拾贰万叁仟肆佰伍拾陆圆整"
    );
    assert_eq!(
        10000000000000000usize.to_chinese_currency(),
        "壹佰万亿圆整"
    );

    assert_eq!(
        123456789123456700usize.to_chinese_currency(),
        "壹仟贰佰叁拾肆万亿伍仟陆佰柒拾捌亿玖仟壹佰贰拾叁万肆仟伍佰陆拾柒圆整"
    );
    assert_eq!(
        100000000000000000usize.to_chinese_currency(),
        "壹仟万亿圆整"
    );

    assert_eq!(
        1234567891234567800usize.to_chinese_currency(),
        "壹亿亿贰仟叁佰肆拾伍万亿陆仟柒佰捌拾玖亿壹仟贰佰叁拾肆万伍仟陆佰柒拾捌圆整"
    );
    assert_eq!(
        1000000000000000000usize.to_chinese_currency(),
        "壹亿亿圆整"
    );

    assert_eq!(
        12345678912345678900usize.to_chinese_currency(),
        "壹拾贰亿亿叁仟肆佰伍拾陆万亿柒仟捌佰玖拾壹亿贰仟叁佰肆拾伍万陆仟柒佰捌拾玖圆整"
    );
    assert_eq!(
        10000000000000000000usize.to_chinese_currency(),
        "壹拾亿亿圆整"
    );
    assert_eq!(
        10000000910000000900usize.to_chinese_currency(),
        "壹拾亿亿零玖拾壹亿零玖圆整"
    );
}
