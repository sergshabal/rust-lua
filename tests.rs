use State;
use GLOBALSINDEX;
use Type;

#[test]
fn test_state_init() {
    let mut _s = State::new();
}

#[test]
#[should_fail]
fn test_error() {
    let mut s = State::new();
    s.pushinteger(42);
    s.error()
}

#[test]
fn test_errorstr() {
    let res = do ::std::task::try::<()> {
        let mut s = State::new();
        s.errorstr("some err");
    };
    let err = res.unwrap_err();
    let expected = "unprotected error in call to Lua API (some err)";
    let s = err.as_ref::<~str>();
    if s.is_some() {
        assert_eq!(s.unwrap().as_slice(), expected);
    } else {
        let s = err.as_ref::<&'static str>();
        if s.is_some() {
            assert_eq!(*s.unwrap(), expected);
        } else {
            fail!("unexpected failure result");
        }
    }
}

#[test]
fn test_describe() {
    let mut s = State::new();

    assert_eq!(s.typename(1), "no value");
    s.pushnil();
    assert_eq!(s.typename(-1), "nil");
    s.pushinteger(42);
    assert_eq!(s.typename(-1), "number");
    s.pushstring("test");
    assert_eq!(s.typename(-1), "string");
    s.pushboolean(true);
    assert_eq!(s.typename(-1), "boolean");
    s.pushcfunction(dummy);
    assert_eq!(s.typename(-1), "function");

    extern "C" fn dummy(_L: *mut ::raw::lua_State) -> ::std::libc::c_int {
        0
    }
}

#[test]
fn test_openlibs() {
    let mut s = State::new();

    s.openlibs();
    s.getfield(GLOBALSINDEX, "table");
    assert_eq!(s.type_(-1), Some(Type::Table));
}
